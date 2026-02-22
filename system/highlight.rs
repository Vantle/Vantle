use std::path::Path;

use owo_colors::{Rgb, Style, Styled};

pub struct Syntax;

impl miette::highlighters::Highlighter for Syntax {
    fn start_highlighter_state<'h>(
        &'h self,
        source: &dyn miette::SpanContents<'_>,
    ) -> Box<dyn miette::highlighters::HighlighterState + 'h> {
        let language = detect(source);
        let text = std::str::from_utf8(source.data()).unwrap_or("");
        let lines = match &language {
            Some(Language::Rust) => classify_rust(text),
            Some(Language::Molten) => classify_molten(text),
            Some(Language::Starlark) => classify_starlark(text),
            None => Vec::new(),
        };
        Box::new(State { line: 0, lines })
    }
}

struct State {
    line: usize,
    lines: Vec<Vec<Classified>>,
}

impl miette::highlighters::HighlighterState for State {
    fn highlight_line<'s>(&mut self, line: &'s str) -> Vec<Styled<&'s str>> {
        let current = self.line;
        self.line += 1;

        let spans = match self.lines.get(current) {
            Some(spans) if !spans.is_empty() => spans,
            _ => return vec![Style::default().style(line)],
        };

        let mut result = Vec::new();
        let mut pos = 0;
        let len = line.len();

        for span in spans {
            let start = span.start.min(len);
            let end = span.end.min(len);
            if start > pos {
                result.push(Style::default().style(&line[pos..start]));
            }
            if start < end {
                result.push(span.category.style().style(&line[start..end]));
            }
            pos = end;
        }

        if pos < len {
            result.push(Style::default().style(&line[pos..]));
        }

        result
    }
}

fn detect(source: &dyn miette::SpanContents<'_>) -> Option<Language> {
    source.language().and_then(Language::from_name).or_else(|| {
        source.name().and_then(|name| {
            Path::new(name)
                .extension()
                .and_then(|e| e.to_str())
                .and_then(Language::from_extension)
        })
    })
}

enum Language {
    Rust,
    Molten,
    Starlark,
}

impl Language {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "rust" => Some(Self::Rust),
            "molten" => Some(Self::Molten),
            "starlark" => Some(Self::Starlark),
            _ => None,
        }
    }

    fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "rs" => Some(Self::Rust),
            "magma" | "lava" => Some(Self::Molten),
            "bzl" | "bazel" | "star" => Some(Self::Starlark),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
enum Category {
    Keyword,
    Entity,
    String,
    Constant,
    Storage,
    Punctuation,
    Comment,
}

impl Category {
    fn style(self) -> Style {
        match self {
            Self::Keyword => Style::new().color(Rgb(212, 160, 224)),
            Self::Entity => Style::new().color(Rgb(240, 148, 62)),
            Self::String => Style::new().color(Rgb(126, 200, 158)),
            Self::Constant => Style::new().color(Rgb(222, 187, 107)),
            Self::Storage => Style::new().color(Rgb(143, 168, 212)),
            Self::Punctuation => Style::new().color(Rgb(146, 141, 135)),
            Self::Comment => Style::new().color(Rgb(132, 127, 125)),
        }
    }
}

struct Classified {
    start: usize,
    end: usize,
    category: Category,
}

fn classify_rust(source: &str) -> Vec<Vec<Classified>> {
    let Ok(ast) = syn::parse_file(source) else {
        return Vec::new();
    };

    let line_count = source.lines().count().max(1);
    let mut lines: Vec<Vec<Classified>> = (0..line_count).map(|_| Vec::new()).collect();
    let mut collector = RustCollector { lines: &mut lines };

    for item in &ast.items {
        collector.item(item);
    }

    for line in &mut lines {
        line.sort_by_key(|c| c.start);
    }

    lines
}

struct RustCollector<'a> {
    lines: &'a mut [Vec<Classified>],
}

impl RustCollector<'_> {
    fn emit(&mut self, span: proc_macro2::Span, category: Category) {
        let start = span.start();
        let end = span.end();
        if start.line == 0 || start.line > self.lines.len() {
            return;
        }
        let idx = start.line - 1;
        let last = if start.line == end.line {
            end.column
        } else {
            usize::MAX
        };
        if start.column < last {
            self.lines[idx].push(Classified {
                start: start.column,
                end: last,
                category,
            });
        }
    }

    fn item(&mut self, item: &syn::Item) {
        match item {
            syn::Item::Fn(f) => {
                self.visibility(&f.vis);
                if let Some(t) = f.sig.constness {
                    self.emit(t.span, Category::Storage);
                }
                if let Some(t) = f.sig.asyncness {
                    self.emit(t.span, Category::Storage);
                }
                if let Some(t) = f.sig.unsafety {
                    self.emit(t.span, Category::Storage);
                }
                self.emit(f.sig.fn_token.span, Category::Keyword);
                self.emit(f.sig.ident.span(), Category::Entity);
                self.generics(&f.sig.generics);
                for input in &f.sig.inputs {
                    self.fn_arg(input);
                }
                self.return_type(&f.sig.output);
                if let Some(wc) = &f.sig.generics.where_clause {
                    self.where_clause(wc);
                }
                self.block(&f.block);
            }
            syn::Item::Struct(s) => {
                self.visibility(&s.vis);
                self.emit(s.struct_token.span, Category::Keyword);
                self.emit(s.ident.span(), Category::Entity);
                self.generics(&s.generics);
                if let Some(wc) = &s.generics.where_clause {
                    self.where_clause(wc);
                }
                self.fields(&s.fields);
            }
            syn::Item::Enum(e) => {
                self.visibility(&e.vis);
                self.emit(e.enum_token.span, Category::Keyword);
                self.emit(e.ident.span(), Category::Entity);
                self.generics(&e.generics);
                for variant in &e.variants {
                    self.emit(variant.ident.span(), Category::Entity);
                    self.fields(&variant.fields);
                }
            }
            syn::Item::Impl(i) => {
                if let Some(t) = &i.unsafety {
                    self.emit(t.span, Category::Storage);
                }
                self.emit(i.impl_token.span, Category::Keyword);
                self.generics(&i.generics);
                if let Some((_, path, for_token)) = &i.trait_ {
                    self.path(path);
                    self.emit(for_token.span, Category::Keyword);
                }
                self.ty(&i.self_ty);
                if let Some(wc) = &i.generics.where_clause {
                    self.where_clause(wc);
                }
                for item in &i.items {
                    self.impl_item(item);
                }
            }
            syn::Item::Trait(t) => {
                self.visibility(&t.vis);
                if let Some(u) = &t.unsafety {
                    self.emit(u.span, Category::Storage);
                }
                self.emit(t.trait_token.span, Category::Keyword);
                self.emit(t.ident.span(), Category::Entity);
                self.generics(&t.generics);
                for item in &t.items {
                    self.trait_item(item);
                }
            }
            syn::Item::Use(u) => {
                self.visibility(&u.vis);
                self.emit(u.use_token.span, Category::Keyword);
                self.use_tree(&u.tree);
            }
            syn::Item::Const(c) => {
                self.visibility(&c.vis);
                self.emit(c.const_token.span, Category::Storage);
                self.ty(&c.ty);
                self.expr(&c.expr);
            }
            syn::Item::Static(s) => {
                self.visibility(&s.vis);
                self.emit(s.static_token.span, Category::Storage);
                if let syn::StaticMutability::Mut(t) = &s.mutability {
                    self.emit(t.span, Category::Keyword);
                }
                self.ty(&s.ty);
                self.expr(&s.expr);
            }
            syn::Item::Type(t) => {
                self.visibility(&t.vis);
                self.emit(t.type_token.span, Category::Keyword);
                self.emit(t.ident.span(), Category::Entity);
                self.generics(&t.generics);
                self.ty(&t.ty);
            }
            syn::Item::Mod(m) => {
                self.visibility(&m.vis);
                self.emit(m.mod_token.span, Category::Keyword);
                if let Some((_, items)) = &m.content {
                    for item in items {
                        self.item(item);
                    }
                }
            }
            syn::Item::Macro(m) => {
                self.path(&m.mac.path);
            }
            _ => {}
        }
    }

    fn impl_item(&mut self, item: &syn::ImplItem) {
        match item {
            syn::ImplItem::Fn(f) => {
                self.visibility(&f.vis);
                if let Some(t) = f.sig.constness {
                    self.emit(t.span, Category::Storage);
                }
                if let Some(t) = f.sig.asyncness {
                    self.emit(t.span, Category::Storage);
                }
                if let Some(t) = f.sig.unsafety {
                    self.emit(t.span, Category::Storage);
                }
                self.emit(f.sig.fn_token.span, Category::Keyword);
                self.emit(f.sig.ident.span(), Category::Entity);
                self.generics(&f.sig.generics);
                for input in &f.sig.inputs {
                    self.fn_arg(input);
                }
                self.return_type(&f.sig.output);
                if let Some(wc) = &f.sig.generics.where_clause {
                    self.where_clause(wc);
                }
                self.block(&f.block);
            }
            syn::ImplItem::Type(t) => {
                self.emit(t.type_token.span, Category::Keyword);
                self.emit(t.ident.span(), Category::Entity);
                self.ty(&t.ty);
            }
            syn::ImplItem::Const(c) => {
                self.emit(c.const_token.span, Category::Storage);
                self.ty(&c.ty);
                self.expr(&c.expr);
            }
            _ => {}
        }
    }

    fn trait_item(&mut self, item: &syn::TraitItem) {
        match item {
            syn::TraitItem::Fn(f) => {
                self.emit(f.sig.fn_token.span, Category::Keyword);
                self.emit(f.sig.ident.span(), Category::Entity);
                self.generics(&f.sig.generics);
                for input in &f.sig.inputs {
                    self.fn_arg(input);
                }
                self.return_type(&f.sig.output);
                if let Some(block) = &f.default {
                    self.block(block);
                }
            }
            syn::TraitItem::Type(t) => {
                self.emit(t.type_token.span, Category::Keyword);
                self.emit(t.ident.span(), Category::Entity);
            }
            _ => {}
        }
    }

    fn visibility(&mut self, vis: &syn::Visibility) {
        match vis {
            syn::Visibility::Public(token) => {
                self.emit(token.span, Category::Keyword);
            }
            syn::Visibility::Restricted(r) => {
                self.emit(r.pub_token.span, Category::Keyword);
            }
            syn::Visibility::Inherited => {}
        }
    }

    fn generics(&mut self, generics: &syn::Generics) {
        for param in &generics.params {
            match param {
                syn::GenericParam::Type(t) => {
                    self.emit(t.ident.span(), Category::Entity);
                    for bound in &t.bounds {
                        self.type_param_bound(bound);
                    }
                }
                syn::GenericParam::Lifetime(l) => {
                    self.emit(l.lifetime.span(), Category::Storage);
                }
                syn::GenericParam::Const(c) => {
                    self.emit(c.const_token.span, Category::Keyword);
                    self.emit(c.ident.span(), Category::Entity);
                    self.ty(&c.ty);
                }
            }
        }
    }

    fn where_clause(&mut self, clause: &syn::WhereClause) {
        self.emit(clause.where_token.span, Category::Keyword);
        for predicate in &clause.predicates {
            if let syn::WherePredicate::Type(t) = predicate {
                self.ty(&t.bounded_ty);
                for bound in &t.bounds {
                    self.type_param_bound(bound);
                }
            }
        }
    }

    fn type_param_bound(&mut self, bound: &syn::TypeParamBound) {
        match bound {
            syn::TypeParamBound::Trait(t) => self.path(&t.path),
            syn::TypeParamBound::Lifetime(l) => {
                self.emit(l.span(), Category::Storage);
            }
            _ => {}
        }
    }

    fn fn_arg(&mut self, arg: &syn::FnArg) {
        match arg {
            syn::FnArg::Receiver(r) => {
                if let Some((_, Some(lt))) = &r.reference {
                    self.emit(lt.span(), Category::Storage);
                }
                if let Some(t) = &r.mutability {
                    self.emit(t.span, Category::Keyword);
                }
                self.emit(r.self_token.span, Category::Keyword);
            }
            syn::FnArg::Typed(t) => {
                self.pat(&t.pat);
                self.ty(&t.ty);
            }
        }
    }

    fn return_type(&mut self, rt: &syn::ReturnType) {
        if let syn::ReturnType::Type(_, ty) = rt {
            self.ty(ty);
        }
    }

    fn block(&mut self, block: &syn::Block) {
        for stmt in &block.stmts {
            self.stmt(stmt);
        }
    }

    fn stmt(&mut self, stmt: &syn::Stmt) {
        match stmt {
            syn::Stmt::Local(local) => {
                self.emit(local.let_token.span, Category::Keyword);
                self.pat(&local.pat);
                if let Some(init) = &local.init {
                    self.expr(&init.expr);
                    if let Some((_, diverge)) = &init.diverge {
                        self.expr(diverge);
                    }
                }
            }
            syn::Stmt::Item(item) => self.item(item),
            syn::Stmt::Expr(expr, _) => self.expr(expr),
            syn::Stmt::Macro(m) => self.path(&m.mac.path),
        }
    }

    fn ty(&mut self, ty: &syn::Type) {
        match ty {
            syn::Type::Path(p) => {
                for segment in &p.path.segments {
                    let name = segment.ident.to_string();
                    if name.starts_with(|c: char| c.is_uppercase()) {
                        self.emit(segment.ident.span(), Category::Entity);
                    }
                    self.path_arguments(&segment.arguments);
                }
            }
            syn::Type::Reference(r) => {
                if let Some(lt) = &r.lifetime {
                    self.emit(lt.span(), Category::Storage);
                }
                if let Some(t) = &r.mutability {
                    self.emit(t.span, Category::Keyword);
                }
                self.ty(&r.elem);
            }
            syn::Type::Slice(s) => self.ty(&s.elem),
            syn::Type::Array(a) => {
                self.ty(&a.elem);
                self.expr(&a.len);
            }
            syn::Type::Tuple(t) => {
                for elem in &t.elems {
                    self.ty(elem);
                }
            }
            syn::Type::ImplTrait(i) => {
                self.emit(i.impl_token.span, Category::Keyword);
                for bound in &i.bounds {
                    self.type_param_bound(bound);
                }
            }
            syn::Type::TraitObject(o) => {
                if let Some(t) = &o.dyn_token {
                    self.emit(t.span, Category::Keyword);
                }
                for bound in &o.bounds {
                    self.type_param_bound(bound);
                }
            }
            _ => {}
        }
    }

    fn path(&mut self, path: &syn::Path) {
        for segment in &path.segments {
            let name = segment.ident.to_string();
            if name.starts_with(|c: char| c.is_uppercase()) {
                self.emit(segment.ident.span(), Category::Entity);
            }
            self.path_arguments(&segment.arguments);
        }
    }

    fn path_arguments(&mut self, args: &syn::PathArguments) {
        match args {
            syn::PathArguments::AngleBracketed(a) => {
                for arg in &a.args {
                    match arg {
                        syn::GenericArgument::Type(t) => self.ty(t),
                        syn::GenericArgument::Lifetime(l) => {
                            self.emit(l.span(), Category::Storage);
                        }
                        _ => {}
                    }
                }
            }
            syn::PathArguments::Parenthesized(p) => {
                for input in &p.inputs {
                    self.ty(input);
                }
                if let syn::ReturnType::Type(_, ty) = &p.output {
                    self.ty(ty);
                }
            }
            syn::PathArguments::None => {}
        }
    }

    fn expr(&mut self, expr: &syn::Expr) {
        match expr {
            syn::Expr::Lit(lit) => {
                let span = lit.lit.span();
                match &lit.lit {
                    syn::Lit::Str(_)
                    | syn::Lit::ByteStr(_)
                    | syn::Lit::CStr(_)
                    | syn::Lit::Char(_)
                    | syn::Lit::Byte(_) => self.emit(span, Category::String),
                    syn::Lit::Int(_) | syn::Lit::Float(_) | syn::Lit::Bool(_) => {
                        self.emit(span, Category::Constant);
                    }
                    _ => {}
                }
            }
            syn::Expr::Path(p) => {
                for segment in &p.path.segments {
                    let name = segment.ident.to_string();
                    if name.starts_with(|c: char| c.is_uppercase()) {
                        self.emit(segment.ident.span(), Category::Entity);
                    } else if matches!(name.as_str(), "true" | "false") {
                        self.emit(segment.ident.span(), Category::Constant);
                    }
                    self.path_arguments(&segment.arguments);
                }
            }
            syn::Expr::Call(c) => {
                self.expr(&c.func);
                for arg in &c.args {
                    self.expr(arg);
                }
            }
            syn::Expr::MethodCall(mc) => {
                self.expr(&mc.receiver);
                self.emit(mc.method.span(), Category::Entity);
                if let Some(turbo) = &mc.turbofish {
                    for arg in &turbo.args {
                        if let syn::GenericArgument::Type(t) = arg {
                            self.ty(t);
                        }
                    }
                }
                for arg in &mc.args {
                    self.expr(arg);
                }
            }
            syn::Expr::Block(b) => {
                for stmt in &b.block.stmts {
                    self.stmt(stmt);
                }
            }
            syn::Expr::If(i) => {
                self.emit(i.if_token.span, Category::Keyword);
                self.expr(&i.cond);
                for stmt in &i.then_branch.stmts {
                    self.stmt(stmt);
                }
                if let Some((else_token, branch)) = &i.else_branch {
                    self.emit(else_token.span, Category::Keyword);
                    self.expr(branch);
                }
            }
            syn::Expr::Match(m) => {
                self.emit(m.match_token.span, Category::Keyword);
                self.expr(&m.expr);
                for arm in &m.arms {
                    self.pat(&arm.pat);
                    if let Some((_, guard)) = &arm.guard {
                        self.expr(guard);
                    }
                    self.expr(&arm.body);
                }
            }
            syn::Expr::Return(r) => {
                self.emit(r.return_token.span, Category::Keyword);
                if let Some(e) = &r.expr {
                    self.expr(e);
                }
            }
            syn::Expr::Reference(r) => {
                if let Some(t) = &r.mutability {
                    self.emit(t.span, Category::Keyword);
                }
                self.expr(&r.expr);
            }
            syn::Expr::Binary(b) => {
                self.expr(&b.left);
                self.expr(&b.right);
            }
            syn::Expr::Unary(u) => self.expr(&u.expr),
            syn::Expr::Try(t) => self.expr(&t.expr),
            syn::Expr::Await(a) => {
                self.expr(&a.base);
                self.emit(a.await_token.span, Category::Keyword);
            }
            syn::Expr::Closure(c) => {
                if let Some(t) = c.constness {
                    self.emit(t.span, Category::Storage);
                }
                if let Some(t) = c.asyncness {
                    self.emit(t.span, Category::Storage);
                }
                if let Some(t) = c.capture {
                    self.emit(t.span, Category::Keyword);
                }
                for input in &c.inputs {
                    self.pat(input);
                }
                self.return_type(&c.output);
                self.expr(&c.body);
            }
            syn::Expr::Let(l) => {
                self.emit(l.let_token.span, Category::Keyword);
                self.pat(&l.pat);
                self.expr(&l.expr);
            }
            syn::Expr::Cast(c) => {
                self.expr(&c.expr);
                self.emit(c.as_token.span, Category::Keyword);
                self.ty(&c.ty);
            }
            syn::Expr::Struct(s) => {
                self.path(&s.path);
                for field in &s.fields {
                    self.expr(&field.expr);
                }
            }
            syn::Expr::Field(f) => self.expr(&f.base),
            syn::Expr::Index(i) => {
                self.expr(&i.expr);
                self.expr(&i.index);
            }
            syn::Expr::Tuple(t) => {
                for elem in &t.elems {
                    self.expr(elem);
                }
            }
            syn::Expr::Array(a) => {
                for elem in &a.elems {
                    self.expr(elem);
                }
            }
            syn::Expr::Paren(p) => self.expr(&p.expr),
            syn::Expr::Range(r) => {
                if let Some(start) = &r.start {
                    self.expr(start);
                }
                if let Some(end) = &r.end {
                    self.expr(end);
                }
            }
            syn::Expr::ForLoop(f) => {
                self.emit(f.for_token.span, Category::Keyword);
                self.pat(&f.pat);
                self.emit(f.in_token.span, Category::Keyword);
                self.expr(&f.expr);
                for stmt in &f.body.stmts {
                    self.stmt(stmt);
                }
            }
            syn::Expr::While(w) => {
                self.emit(w.while_token.span, Category::Keyword);
                self.expr(&w.cond);
                for stmt in &w.body.stmts {
                    self.stmt(stmt);
                }
            }
            syn::Expr::Loop(l) => {
                self.emit(l.loop_token.span, Category::Keyword);
                for stmt in &l.body.stmts {
                    self.stmt(stmt);
                }
            }
            syn::Expr::Break(b) => {
                self.emit(b.break_token.span, Category::Keyword);
                if let Some(e) = &b.expr {
                    self.expr(e);
                }
            }
            syn::Expr::Continue(c) => {
                self.emit(c.continue_token.span, Category::Keyword);
            }
            syn::Expr::Macro(m) => self.path(&m.mac.path),
            syn::Expr::Unsafe(u) => {
                self.emit(u.unsafe_token.span, Category::Storage);
                for stmt in &u.block.stmts {
                    self.stmt(stmt);
                }
            }
            _ => {}
        }
    }

    fn pat(&mut self, pat: &syn::Pat) {
        match pat {
            syn::Pat::Ident(i) => {
                if let Some(t) = &i.by_ref {
                    self.emit(t.span, Category::Keyword);
                }
                if let Some(t) = &i.mutability {
                    self.emit(t.span, Category::Keyword);
                }
            }
            syn::Pat::TupleStruct(ts) => {
                self.path(&ts.path);
                for elem in &ts.elems {
                    self.pat(elem);
                }
            }
            syn::Pat::Struct(s) => {
                self.path(&s.path);
                for field in &s.fields {
                    self.pat(&field.pat);
                }
            }
            syn::Pat::Tuple(t) => {
                for elem in &t.elems {
                    self.pat(elem);
                }
            }
            syn::Pat::Reference(r) => {
                if let Some(t) = &r.mutability {
                    self.emit(t.span, Category::Keyword);
                }
                self.pat(&r.pat);
            }
            syn::Pat::Slice(s) => {
                for elem in &s.elems {
                    self.pat(elem);
                }
            }
            syn::Pat::Or(o) => {
                for case in &o.cases {
                    self.pat(case);
                }
            }
            syn::Pat::Lit(l) => {
                let span = l.lit.span();
                match &l.lit {
                    syn::Lit::Str(_)
                    | syn::Lit::ByteStr(_)
                    | syn::Lit::CStr(_)
                    | syn::Lit::Char(_)
                    | syn::Lit::Byte(_) => self.emit(span, Category::String),
                    syn::Lit::Int(_) | syn::Lit::Float(_) | syn::Lit::Bool(_) => {
                        self.emit(span, Category::Constant);
                    }
                    _ => {}
                }
            }
            syn::Pat::Type(t) => {
                self.pat(&t.pat);
                self.ty(&t.ty);
            }
            _ => {}
        }
    }

    fn use_tree(&mut self, tree: &syn::UseTree) {
        match tree {
            syn::UseTree::Path(p) => {
                let name = p.ident.to_string();
                if name.starts_with(|c: char| c.is_uppercase()) {
                    self.emit(p.ident.span(), Category::Entity);
                }
                self.use_tree(&p.tree);
            }
            syn::UseTree::Name(n) => {
                let name = n.ident.to_string();
                if name.starts_with(|c: char| c.is_uppercase()) {
                    self.emit(n.ident.span(), Category::Entity);
                }
            }
            syn::UseTree::Rename(r) => {
                let name = r.ident.to_string();
                if name.starts_with(|c: char| c.is_uppercase()) {
                    self.emit(r.ident.span(), Category::Entity);
                }
                self.emit(r.as_token.span, Category::Keyword);
            }
            syn::UseTree::Group(g) => {
                for item in &g.items {
                    self.use_tree(item);
                }
            }
            syn::UseTree::Glob(_) => {}
        }
    }

    fn fields(&mut self, fields: &syn::Fields) {
        match fields {
            syn::Fields::Named(named) => {
                for field in &named.named {
                    self.visibility(&field.vis);
                    self.ty(&field.ty);
                }
            }
            syn::Fields::Unnamed(unnamed) => {
                for field in &unnamed.unnamed {
                    self.visibility(&field.vis);
                    self.ty(&field.ty);
                }
            }
            syn::Fields::Unit => {}
        }
    }
}

fn classify_molten(source: &str) -> Vec<Vec<Classified>> {
    source.lines().map(classify_molten_line).collect()
}

fn classify_molten_line(line: &str) -> Vec<Classified> {
    let mut spans = Vec::new();
    let bytes = line.as_bytes();
    let mut pos = 0;

    while pos < bytes.len() {
        match bytes[pos] {
            b'[' | b']' | b',' => {
                spans.push(Classified {
                    start: pos,
                    end: pos + 1,
                    category: Category::Keyword,
                });
                pos += 1;
            }
            b'(' | b')' | b'.' => {
                spans.push(Classified {
                    start: pos,
                    end: pos + 1,
                    category: Category::Punctuation,
                });
                pos += 1;
            }
            c if c.is_ascii_alphabetic() || c == b'_' => {
                let start = pos;
                while pos < bytes.len()
                    && (bytes[pos].is_ascii_alphanumeric() || bytes[pos] == b'_')
                {
                    pos += 1;
                }
                spans.push(Classified {
                    start,
                    end: pos,
                    category: Category::Entity,
                });
            }
            _ => pos += 1,
        }
    }

    spans
}

fn classify_starlark(source: &str) -> Vec<Vec<Classified>> {
    let line_count = source.lines().count().max(1);
    let mut lines: Vec<Vec<Classified>> = (0..line_count).map(|_| Vec::new()).collect();

    let mut parser = tree_sitter::Parser::new();
    if parser
        .set_language(&tree_sitter_python::LANGUAGE.into())
        .is_err()
    {
        return lines;
    }
    let Some(tree) = parser.parse(source, None) else {
        return lines;
    };

    classify_terminal(tree.root_node(), source, &mut lines, category_starlark);
    lines
}

fn classify_terminal(
    node: tree_sitter::Node,
    source: &str,
    lines: &mut [Vec<Classified>],
    categorize: fn(&tree_sitter::Node, &str) -> Option<Category>,
) {
    if node.child_count() == 0 {
        let start = node.start_position();
        let end = node.end_position();
        let text = &source[node.start_byte()..node.end_byte()];

        if let Some(category) = categorize(&node, text) {
            if start.row == end.row {
                if let Some(line) = lines.get_mut(start.row) {
                    line.push(Classified {
                        start: start.column,
                        end: end.column,
                        category,
                    });
                }
            } else {
                for row in start.row..=end.row {
                    if let Some(line) = lines.get_mut(row) {
                        let first = if row == start.row { start.column } else { 0 };
                        let last = if row == end.row {
                            end.column
                        } else {
                            source.lines().nth(row).map_or(0, str::len)
                        };
                        if first < last {
                            line.push(Classified {
                                start: first,
                                end: last,
                                category,
                            });
                        }
                    }
                }
            }
        }
        return;
    }

    #[expect(clippy::cast_possible_truncation)]
    for i in 0..node.child_count() as u32 {
        if let Some(child) = node.child(i) {
            classify_terminal(child, source, lines, categorize);
        }
    }
}

fn category_starlark(node: &tree_sitter::Node, text: &str) -> Option<Category> {
    if node.is_error() || node.is_missing() {
        return None;
    }

    let kind = node.kind();

    if node.is_named() {
        return match kind {
            "comment" => Some(Category::Comment),
            "string" | "string_content" | "string_start" | "string_end" | "escape_sequence" => {
                Some(Category::String)
            }
            "integer" | "float" | "true" | "false" | "none" => Some(Category::Constant),
            "identifier" => {
                if matches!(text, "True" | "False" | "None") {
                    return Some(Category::Constant);
                }
                if let Some(parent) = node.parent() {
                    match parent.kind() {
                        "function_definition" | "class_definition" => {
                            if parent.child_by_field_name("name").as_ref() == Some(node) {
                                return Some(Category::Entity);
                            }
                        }
                        "call" => {
                            if parent.child_by_field_name("function").as_ref() == Some(node) {
                                return Some(Category::Entity);
                            }
                        }
                        _ => {}
                    }
                }
                None
            }
            _ => None,
        };
    }

    match kind {
        "def" | "if" | "elif" | "else" | "for" | "in" | "return" | "pass" | "break"
        | "continue" | "and" | "or" | "not" | "lambda" | "class" | "import" | "from" | "as"
        | "with" | "while" | "try" | "except" | "finally" | "raise" | "yield" | "del"
        | "assert" | "global" | "nonlocal" | "is" => Some(Category::Keyword),
        "(" | ")" | "[" | "]" | "{" | "}" | ":" | "," | "." | ";" | "=" | "+=" | "-=" | "*="
        | "/=" | "//=" | "%=" | "**=" | "&=" | "|=" | "^=" | ">>=" | "<<=" | "+" | "-" | "*"
        | "/" | "//" | "%" | "**" | "<" | ">" | "<=" | ">=" | "==" | "!=" | "|" | "&" | "^"
        | "~" | "<<" | ">>" | "@" => Some(Category::Punctuation),
        _ => None,
    }
}
