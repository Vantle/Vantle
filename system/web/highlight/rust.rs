use std::fmt::Write;

use syn::visit::Visit;
use syn::{
    Expr, FnArg, GenericArgument, Item, Lit, Pat, PathArguments, ReturnType, Stmt, Type, UseTree,
};

pub fn rust(ast: &syn::File) -> miette::Result<String> {
    let mut visitor = Visitor::new();
    visitor.visit_file(ast);
    Ok(visitor.output)
}

#[must_use]
pub fn snippet(ast: &syn::File) -> String {
    let mut visitor = Visitor::new();
    let Some(Item::Fn(f)) = ast.items.first() else {
        return visitor.output;
    };
    for (i, stmt) in f.block.stmts.iter().enumerate() {
        if i > 0 {
            visitor.output.push('\n');
        }
        visitor.emit_statement(stmt, "");
    }
    visitor.output
}

struct Visitor {
    output: String,
}

impl Visitor {
    fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    fn token(&mut self, class: &str, text: &str) {
        write!(
            self.output,
            "<span class=\"syntax-{class}\">{}</span>",
            escape::escape(text)
        )
        .unwrap();
    }

    fn literal(&mut self, class: &str, word: &str) {
        write!(self.output, "<span class=\"syntax-{class}\">{word}</span>").unwrap();
    }

    fn keyword(&mut self, word: &str) {
        self.literal("keyword", word);
    }
    fn entity(&mut self, name: &str) {
        self.token("entity", name);
    }
    fn string(&mut self, text: &str) {
        self.token("string", text);
    }
    fn constant(&mut self, text: &str) {
        self.token("constant", text);
    }
    fn storage(&mut self, text: &str) {
        self.literal("storage", text);
    }
    fn punctuation(&mut self, text: &str) {
        self.token("punctuation", text);
    }
    fn variable(&mut self, text: &str) {
        self.token("variable", text);
    }
    fn function(&mut self, text: &str) {
        self.token("function", text);
    }
    fn operator(&mut self, text: &str) {
        self.token("operator", text);
    }
    fn macro_name(&mut self, text: &str) {
        self.token("macro", text);
    }
    fn comment(&mut self, text: &str) {
        self.token("comment", text);
    }

    fn plain(&mut self, text: &str) {
        self.output.push_str(&escape::escape(text));
    }

    fn node(&mut self, class: &str) {
        escape::open(&mut self.output, class);
    }

    fn end(&mut self) {
        escape::close(&mut self.output);
    }

    fn leading(&self) -> String {
        let bytes = self.output.as_bytes();
        let start = bytes.iter().rposition(|&b| b == b'\n').map_or(0, |p| p + 1);
        let segment = &self.output[start..];
        let mut spaces = 0;
        let mut tag = false;
        for character in segment.chars() {
            match character {
                '<' => tag = true,
                '>' if tag => tag = false,
                ' ' if !tag => spaces += 1,
                _ if !tag => break,
                _ => {}
            }
        }
        " ".repeat(spaces + 4)
    }

    fn emit_chain(&mut self, expression: &Expr) {
        let (root, links, trailing) = flatten(expression);
        let indent = self.leading();
        self.emit_expr(root);
        for link in &links {
            self.output.push('\n');
            self.output.push_str(&indent);
            match link {
                Link::Method(mc) => {
                    self.punctuation(".");
                    self.function(&mc.method.to_string());
                    if let Some(turbofish) = &mc.turbofish {
                        self.punctuation("::<");
                        for (i, arg) in turbofish.args.iter().enumerate() {
                            if i > 0 {
                                self.plain(", ");
                            }
                            match arg {
                                GenericArgument::Type(t) => self.emit_type(t),
                                _ => self.plain(&arg.to_token_stream().to_string()),
                            }
                        }
                        self.punctuation(">");
                    }
                    self.punctuation("(");
                    for (i, arg) in mc.args.iter().enumerate() {
                        if i > 0 {
                            self.plain(", ");
                        }
                        self.emit_expr(arg);
                    }
                    self.punctuation(")");
                }
                Link::Field(f) => {
                    self.punctuation(".");
                    self.variable(&f.member.to_token_stream().to_string());
                }
                Link::Await => {
                    self.punctuation(".");
                    self.keyword("await");
                }
            }
        }
        if trailing {
            self.operator("?");
        }
    }

    fn emit_attrs(&mut self, attrs: &[syn::Attribute]) {
        if attrs.is_empty() {
            return;
        }
        self.node("attributes");
        for attr in attrs {
            self.macro_name(&attr.to_token_stream().to_string());
            self.output.push('\n');
        }
        self.end();
    }

    fn emit_visibility(&mut self, vis: &syn::Visibility) {
        match vis {
            syn::Visibility::Public(_) => {
                self.node("visibility");
                self.keyword("pub");
                self.output.push(' ');
                self.end();
            }
            syn::Visibility::Restricted(restricted) => {
                self.node("visibility");
                self.keyword("pub");
                self.punctuation("(");
                self.keyword(&restricted.path.to_token_stream().to_string());
                self.punctuation(")");
                self.output.push(' ');
                self.end();
            }
            syn::Visibility::Inherited => {}
        }
    }

    fn emit_generics(&mut self, generics: &syn::Generics) {
        if !generics.params.is_empty() {
            self.node("generics");
            self.punctuation("<");
            for (i, param) in generics.params.iter().enumerate() {
                if i > 0 {
                    self.plain(", ");
                }
                match param {
                    syn::GenericParam::Type(t) => {
                        self.entity(&t.ident.to_string());
                        for (j, bound) in t.bounds.iter().enumerate() {
                            if j == 0 {
                                self.punctuation(": ");
                            } else {
                                self.plain(" + ");
                            }
                            self.emit_type_bound(bound);
                        }
                    }
                    syn::GenericParam::Lifetime(l) => {
                        self.storage(&format!("'{}", l.lifetime.ident));
                    }
                    syn::GenericParam::Const(c) => {
                        self.keyword("const");
                        self.plain(" ");
                        self.constant(&c.ident.to_string());
                        self.punctuation(": ");
                        self.emit_type(&c.ty);
                    }
                }
            }
            self.punctuation(">");
            self.end();
        }
    }

    fn emit_type_bound(&mut self, bound: &syn::TypeParamBound) {
        match bound {
            syn::TypeParamBound::Trait(t) => {
                self.emit_path(&t.path);
            }
            syn::TypeParamBound::Lifetime(l) => {
                self.storage(&format!("'{}", l.ident));
            }
            _ => {
                self.plain(&bound.to_token_stream().to_string());
            }
        }
    }

    fn emit_where_clause(&mut self, clause: &syn::WhereClause) {
        self.node("where");
        self.output.push('\n');
        self.keyword("where");
        self.output.push('\n');
        for (i, predicate) in clause.predicates.iter().enumerate() {
            if i > 0 {
                self.plain(",\n");
            }
            self.plain("    ");
            match predicate {
                syn::WherePredicate::Type(t) => {
                    self.emit_type(&t.bounded_ty);
                    self.punctuation(": ");
                    for (j, bound) in t.bounds.iter().enumerate() {
                        if j > 0 {
                            self.plain(" + ");
                        }
                        self.emit_type_bound(bound);
                    }
                }
                _ => {
                    self.plain(&predicate.to_token_stream().to_string());
                }
            }
        }
        self.plain(",");
        self.end();
    }

    fn emit_path(&mut self, path: &syn::Path) {
        self.node("path");
        for (i, segment) in path.segments.iter().enumerate() {
            if i > 0 {
                self.punctuation("::");
            }
            let name = segment.ident.to_string();
            if is_type_name(&name) {
                self.entity(&name);
            } else {
                self.plain(&name);
            }
            match &segment.arguments {
                PathArguments::AngleBracketed(args) => {
                    self.punctuation("::<");
                    for (j, arg) in args.args.iter().enumerate() {
                        if j > 0 {
                            self.plain(", ");
                        }
                        match arg {
                            GenericArgument::Type(t) => self.emit_type(t),
                            GenericArgument::Lifetime(l) => {
                                self.storage(&format!("'{}", l.ident));
                            }
                            _ => self.plain(&arg.to_token_stream().to_string()),
                        }
                    }
                    self.punctuation(">");
                }
                PathArguments::Parenthesized(args) => {
                    self.punctuation("(");
                    for (j, input) in args.inputs.iter().enumerate() {
                        if j > 0 {
                            self.plain(", ");
                        }
                        self.emit_type(input);
                    }
                    self.punctuation(")");
                    if let ReturnType::Type(_, ty) = &args.output {
                        self.plain(" ");
                        self.operator("->");
                        self.plain(" ");
                        self.emit_type(ty);
                    }
                }
                PathArguments::None => {}
            }
        }
        self.end();
    }

    fn emit_type(&mut self, ty: &Type) {
        self.node("type");
        match ty {
            Type::Path(path) => {
                if let Some(qself) = &path.qself {
                    self.punctuation("<");
                    self.emit_type(&qself.ty);
                    self.plain(" ");
                    self.keyword("as");
                    self.plain(" ");
                    if let Some(segment) = path.path.segments.first() {
                        let name = segment.ident.to_string();
                        self.entity(&name);
                    }
                    self.punctuation(">");
                    self.punctuation("::");
                    if path.path.segments.len() > 1 {
                        for (i, segment) in path.path.segments.iter().skip(1).enumerate() {
                            if i > 0 {
                                self.punctuation("::");
                            }
                            self.entity(&segment.ident.to_string());
                        }
                    }
                } else {
                    for (i, segment) in path.path.segments.iter().enumerate() {
                        if i > 0 {
                            self.punctuation("::");
                        }
                        let name = segment.ident.to_string();
                        if is_type_name(&name) {
                            self.entity(&name);
                        } else {
                            self.plain(&name);
                        }
                        match &segment.arguments {
                            PathArguments::AngleBracketed(args) => {
                                self.punctuation("<");
                                for (j, arg) in args.args.iter().enumerate() {
                                    if j > 0 {
                                        self.plain(", ");
                                    }
                                    match arg {
                                        GenericArgument::Type(t) => self.emit_type(t),
                                        GenericArgument::Lifetime(l) => {
                                            self.storage(&format!("'{}", l.ident));
                                        }
                                        _ => self.plain(&arg.to_token_stream().to_string()),
                                    }
                                }
                                self.punctuation(">");
                            }
                            PathArguments::Parenthesized(args) => {
                                self.punctuation("(");
                                for (j, input) in args.inputs.iter().enumerate() {
                                    if j > 0 {
                                        self.plain(", ");
                                    }
                                    self.emit_type(input);
                                }
                                self.punctuation(")");
                                if let ReturnType::Type(_, t) = &args.output {
                                    self.plain(" ");
                                    self.operator("->");
                                    self.plain(" ");
                                    self.emit_type(t);
                                }
                            }
                            PathArguments::None => {}
                        }
                    }
                }
            }
            Type::Reference(r) => {
                self.punctuation("&");
                if let Some(lifetime) = &r.lifetime {
                    self.storage(&format!("'{}", lifetime.ident));
                    self.plain(" ");
                }
                if r.mutability.is_some() {
                    self.keyword("mut");
                    self.plain(" ");
                }
                self.emit_type(&r.elem);
            }
            Type::Slice(s) => {
                self.punctuation("[");
                self.emit_type(&s.elem);
                self.punctuation("]");
            }
            Type::Array(a) => {
                self.punctuation("[");
                self.emit_type(&a.elem);
                self.plain("; ");
                self.plain(&a.len.to_token_stream().to_string());
                self.punctuation("]");
            }
            Type::Tuple(t) => {
                self.punctuation("(");
                for (i, elem) in t.elems.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.emit_type(elem);
                }
                self.punctuation(")");
            }
            Type::ImplTrait(imp) => {
                self.keyword("impl");
                self.plain(" ");
                for (i, bound) in imp.bounds.iter().enumerate() {
                    if i > 0 {
                        self.plain(" + ");
                    }
                    self.emit_type_bound(bound);
                }
            }
            Type::TraitObject(obj) => {
                self.keyword("dyn");
                self.plain(" ");
                for (i, bound) in obj.bounds.iter().enumerate() {
                    if i > 0 {
                        self.plain(" + ");
                    }
                    self.emit_type_bound(bound);
                }
            }
            Type::Infer(_) => self.plain("_"),
            Type::Never(_) => self.punctuation("!"),
            _ => self.plain(&ty.to_token_stream().to_string()),
        }
        self.end();
    }

    fn emit_expr(&mut self, expr: &Expr) {
        self.node("expression");
        match expr {
            Expr::Lit(lit) => self.emit_lit(&lit.lit),
            Expr::Path(path) => {
                if path.path.segments.len() == 1 {
                    let name = path.path.segments[0].ident.to_string();
                    if is_type_name(&name) {
                        self.entity(&name);
                    } else if is_keyword_value(&name) {
                        self.constant(&name);
                    } else {
                        self.variable(&name);
                    }
                } else {
                    self.emit_path(&path.path);
                }
            }
            Expr::Call(call) => {
                self.emit_expr(&call.func);
                self.punctuation("(");
                for (i, arg) in call.args.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.emit_expr(arg);
                }
                self.punctuation(")");
            }
            Expr::MethodCall(_) | Expr::Field(_) | Expr::Try(_) | Expr::Await(_)
                if chain(expr) >= 3 =>
            {
                self.emit_chain(expr);
            }
            Expr::MethodCall(mc) => {
                self.emit_expr(&mc.receiver);
                self.punctuation(".");
                self.function(&mc.method.to_string());
                if let Some(turbofish) = &mc.turbofish {
                    self.punctuation("::<");
                    for (i, arg) in turbofish.args.iter().enumerate() {
                        if i > 0 {
                            self.plain(", ");
                        }
                        match arg {
                            GenericArgument::Type(t) => self.emit_type(t),
                            _ => self.plain(&arg.to_token_stream().to_string()),
                        }
                    }
                    self.punctuation(">");
                }
                self.punctuation("(");
                for (i, arg) in mc.args.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.emit_expr(arg);
                }
                self.punctuation(")");
            }
            Expr::Reference(r) => {
                self.punctuation("&");
                if r.mutability.is_some() {
                    self.keyword("mut");
                    self.plain(" ");
                }
                self.emit_expr(&r.expr);
            }
            Expr::Block(block) => {
                self.plain(" {\n");
                for stmt in &block.block.stmts {
                    self.emit_statement(stmt, "");
                }
                self.plain("}");
            }
            Expr::If(expr_if) => {
                self.keyword("if");
                self.plain(" ");
                self.emit_expr(&expr_if.cond);
                self.plain(" {\n");
                for stmt in &expr_if.then_branch.stmts {
                    self.emit_statement(stmt, "");
                }
                self.plain("}");
                if let Some((_, else_branch)) = &expr_if.else_branch {
                    self.plain(" ");
                    self.keyword("else");
                    self.plain(" ");
                    self.emit_expr(else_branch);
                }
            }
            Expr::Match(m) => {
                self.keyword("match");
                self.plain(" ");
                self.emit_expr(&m.expr);
                self.plain(" {\n");
                for arm in &m.arms {
                    self.plain("    ");
                    self.emit_pat(&arm.pat);
                    self.plain(" ");
                    self.operator("=>");
                    self.plain(" ");
                    self.emit_expr(&arm.body);
                    self.plain(",\n");
                }
                self.plain("}");
            }
            Expr::Closure(c) => {
                if c.asyncness.is_some() {
                    self.keyword("async");
                    self.plain(" ");
                }
                if c.movability.is_some() {
                    self.keyword("static");
                    self.plain(" ");
                }
                if c.capture.is_some() {
                    self.keyword("move");
                    self.plain(" ");
                }
                self.punctuation("|");
                for (i, input) in c.inputs.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.emit_pat(input);
                }
                self.punctuation("|");
                self.plain(" ");
                self.emit_expr(&c.body);
            }
            Expr::Struct(s) => {
                self.emit_path(&s.path);
                self.plain(" {\n");
                for (i, field) in s.fields.iter().enumerate() {
                    if i > 0 {
                        self.plain(",\n");
                    }
                    self.plain("    ");
                    if field.colon_token.is_some() {
                        self.variable(&field.member.to_token_stream().to_string());
                        self.punctuation(":");
                        self.plain(" ");
                    }
                    self.emit_expr(&field.expr);
                }
                self.plain("\n}");
            }
            Expr::Field(f) => {
                self.emit_expr(&f.base);
                self.punctuation(".");
                self.variable(&f.member.to_token_stream().to_string());
            }
            Expr::Index(idx) => {
                self.emit_expr(&idx.expr);
                self.punctuation("[");
                self.emit_expr(&idx.index);
                self.punctuation("]");
            }
            Expr::Unary(u) => {
                self.operator(&u.op.to_token_stream().to_string());
                self.emit_expr(&u.expr);
            }
            Expr::Binary(b) => {
                self.emit_expr(&b.left);
                self.plain(" ");
                self.operator(&b.op.to_token_stream().to_string());
                self.plain(" ");
                self.emit_expr(&b.right);
            }
            Expr::Let(l) => {
                self.keyword("let");
                self.plain(" ");
                self.emit_pat(&l.pat);
                self.plain(" ");
                self.operator("=");
                self.plain(" ");
                self.emit_expr(&l.expr);
            }
            Expr::Return(r) => {
                self.keyword("return");
                if let Some(expr) = &r.expr {
                    self.plain(" ");
                    self.emit_expr(expr);
                }
            }
            Expr::Try(t) => {
                self.emit_expr(&t.expr);
                self.operator("?");
            }
            Expr::Await(a) => {
                self.emit_expr(&a.base);
                self.punctuation(".");
                self.keyword("await");
            }
            Expr::Paren(p) => {
                self.punctuation("(");
                self.emit_expr(&p.expr);
                self.punctuation(")");
            }
            Expr::Tuple(t) => {
                self.punctuation("(");
                for (i, elem) in t.elems.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.emit_expr(elem);
                }
                self.punctuation(")");
            }
            Expr::Array(a) => {
                self.punctuation("[");
                for (i, elem) in a.elems.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.emit_expr(elem);
                }
                self.punctuation("]");
            }
            Expr::Cast(c) => {
                self.emit_expr(&c.expr);
                self.plain(" ");
                self.keyword("as");
                self.plain(" ");
                self.emit_type(&c.ty);
            }
            Expr::Range(r) => {
                if let Some(start) = &r.start {
                    self.emit_expr(start);
                }
                self.operator(&r.limits.to_token_stream().to_string());
                if let Some(end) = &r.end {
                    self.emit_expr(end);
                }
            }
            Expr::Macro(m) => {
                self.emit_path(&m.mac.path);
                self.macro_name("!");
                self.punctuation("(");
                self.plain(&m.mac.tokens.to_string());
                self.punctuation(")");
            }
            _ => self.plain(&expr.to_token_stream().to_string()),
        }
        self.end();
    }

    fn emit_lit(&mut self, lit: &Lit) {
        match lit {
            Lit::Str(s) => self.string(&format!("\"{}\"", s.value())),
            Lit::Char(c) => self.string(&format!("'{}'", c.value())),
            Lit::Int(i) => self.constant(&i.to_string()),
            Lit::Float(f) => self.constant(&f.to_string()),
            Lit::Bool(b) => self.constant(&b.value.to_string()),
            Lit::ByteStr(b) => self.string(&format!("b\"{}\"", escape::bytes(&b.value()))),
            Lit::Byte(b) => self.string(&format!("b'{}'", b.value() as char)),
            _ => self.plain(&lit.to_token_stream().to_string()),
        }
    }

    fn emit_pat(&mut self, pat: &Pat) {
        self.node("pattern");
        match pat {
            Pat::Ident(ident) => {
                if ident.by_ref.is_some() {
                    self.keyword("ref");
                    self.plain(" ");
                }
                if ident.mutability.is_some() {
                    self.keyword("mut");
                    self.plain(" ");
                }
                self.variable(&ident.ident.to_string());
            }
            Pat::Wild(_) => self.plain("_"),
            Pat::Lit(lit) => self.emit_lit(&lit.lit),
            Pat::Path(path) => self.emit_path(&path.path),
            Pat::Tuple(t) => {
                self.punctuation("(");
                for (i, elem) in t.elems.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.emit_pat(elem);
                }
                self.punctuation(")");
            }
            Pat::TupleStruct(ts) => {
                self.emit_path(&ts.path);
                self.punctuation("(");
                for (i, elem) in ts.elems.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.emit_pat(elem);
                }
                self.punctuation(")");
            }
            Pat::Struct(s) => {
                self.emit_path(&s.path);
                self.plain(" { ");
                for (i, field) in s.fields.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.variable(&field.member.to_token_stream().to_string());
                    if field.colon_token.is_some() {
                        self.punctuation(": ");
                        self.emit_pat(&field.pat);
                    }
                }
                if s.qself.is_some() {
                    self.plain(", ..");
                }
                self.plain(" }");
            }
            Pat::Reference(r) => {
                self.punctuation("&");
                if r.mutability.is_some() {
                    self.keyword("mut");
                    self.plain(" ");
                }
                self.emit_pat(&r.pat);
            }
            Pat::Slice(s) => {
                self.punctuation("[");
                for (i, elem) in s.elems.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.emit_pat(elem);
                }
                self.punctuation("]");
            }
            Pat::Or(o) => {
                for (i, case) in o.cases.iter().enumerate() {
                    if i > 0 {
                        self.plain(" | ");
                    }
                    self.emit_pat(case);
                }
            }
            Pat::Rest(_) => self.plain(".."),
            Pat::Type(t) => {
                self.emit_pat(&t.pat);
                self.punctuation(": ");
                self.emit_type(&t.ty);
            }
            _ => self.plain(&pat.to_token_stream().to_string()),
        }
        self.end();
    }

    fn emit_statement(&mut self, stmt: &Stmt, indent: &str) {
        match stmt {
            Stmt::Local(local) => {
                self.node("let");
                self.output.push_str(indent);
                self.keyword("let");
                self.plain(" ");
                if local.pat.to_token_stream().to_string().starts_with("mut ") {
                    self.keyword("mut");
                    self.plain(" ");
                    if let Pat::Ident(ident) = &local.pat {
                        self.variable(&ident.ident.to_string());
                    }
                } else {
                    self.emit_pat(&local.pat);
                }
                if let Some(init) = &local.init {
                    self.plain(" ");
                    self.operator("=");
                    self.plain(" ");
                    self.emit_expr(&init.expr);
                }
                self.punctuation(";");
                self.output.push('\n');
                self.end();
            }
            Stmt::Expr(expr, semi) => {
                self.node("statement");
                self.output.push_str(indent);
                self.emit_expr(expr);
                if semi.is_some() {
                    self.punctuation(";");
                }
                self.output.push('\n');
                self.end();
            }
            Stmt::Item(item) => self.emit_item(item),
            Stmt::Macro(m) => {
                self.node("statement");
                self.output.push_str(indent);
                self.emit_path(&m.mac.path);
                self.macro_name("!");
                self.punctuation("(");
                self.plain(&m.mac.tokens.to_string());
                self.punctuation(")");
                if m.semi_token.is_some() {
                    self.punctuation(";");
                }
                self.output.push('\n');
                self.end();
            }
        }
    }

    fn emit_signature(&mut self, sig: &syn::Signature) {
        if sig.constness.is_some() {
            self.storage("const");
            self.plain(" ");
        }
        if sig.asyncness.is_some() {
            self.storage("async");
            self.plain(" ");
        }
        if sig.unsafety.is_some() {
            self.storage("unsafe");
            self.plain(" ");
        }
        self.keyword("fn");
        self.plain(" ");
        self.entity(&sig.ident.to_string());
        self.emit_generics(&sig.generics);
        self.emit_parameters(&sig.inputs);
        if let ReturnType::Type(_, ty) = &sig.output {
            self.plain(" ");
            self.operator("->");
            self.plain(" ");
            self.emit_type(ty);
        }
        if let Some(clause) = &sig.generics.where_clause {
            self.emit_where_clause(clause);
        }
    }

    fn emit_parameters(&mut self, inputs: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>) {
        self.node("parameters");
        self.punctuation("(");
        for (i, arg) in inputs.iter().enumerate() {
            if i > 0 {
                self.plain(", ");
            }
            self.node("parameter");
            match arg {
                FnArg::Receiver(r) => {
                    if r.reference.is_some() {
                        self.punctuation("&");
                        if let Some((_, Some(lifetime))) = &r.reference {
                            self.storage(&format!("'{}", lifetime.ident));
                            self.plain(" ");
                        }
                    }
                    if r.mutability.is_some() {
                        self.keyword("mut");
                        self.plain(" ");
                    }
                    self.keyword("self");
                }
                FnArg::Typed(t) => {
                    self.emit_pat(&t.pat);
                    self.punctuation(":");
                    self.plain(" ");
                    self.emit_type(&t.ty);
                }
            }
            self.end();
        }
        self.punctuation(")");
        self.end();
    }

    fn emit_item(&mut self, item: &Item) {
        match item {
            Item::Use(u) => {
                self.node("use");
                self.emit_attrs(&u.attrs);
                self.emit_visibility(&u.vis);
                self.keyword("use");
                self.plain(" ");
                self.emit_use_tree(&u.tree);
                self.punctuation(";");
                self.output.push('\n');
                self.end();
            }
            Item::Fn(f) => {
                self.node("function");
                self.emit_attrs(&f.attrs);
                self.emit_visibility(&f.vis);
                self.emit_signature(&f.sig);
                self.node("block");
                self.plain(" {\n");
                for stmt in &f.block.stmts {
                    self.emit_statement(stmt, "    ");
                }
                self.plain("}\n");
                self.end();
                self.end();
            }
            Item::Struct(s) => {
                self.node("struct");
                self.emit_attrs(&s.attrs);
                self.emit_visibility(&s.vis);
                self.keyword("struct");
                self.plain(" ");
                self.entity(&s.ident.to_string());
                self.emit_generics(&s.generics);
                match &s.fields {
                    syn::Fields::Named(fields) => {
                        if let Some(clause) = &s.generics.where_clause {
                            self.emit_where_clause(clause);
                        }
                        self.plain(" {\n");
                        for field in &fields.named {
                            self.node("field");
                            self.emit_attrs(&field.attrs);
                            self.plain("    ");
                            self.emit_visibility(&field.vis);
                            self.variable(&field.ident.as_ref().unwrap().to_string());
                            self.punctuation(":");
                            self.plain(" ");
                            self.emit_type(&field.ty);
                            self.plain(",\n");
                            self.end();
                        }
                        self.plain("}\n");
                    }
                    syn::Fields::Unnamed(fields) => {
                        self.punctuation("(");
                        for (i, field) in fields.unnamed.iter().enumerate() {
                            if i > 0 {
                                self.plain(", ");
                            }
                            self.emit_visibility(&field.vis);
                            self.emit_type(&field.ty);
                        }
                        self.punctuation(")");
                        self.punctuation(";");
                        self.output.push('\n');
                    }
                    syn::Fields::Unit => {
                        self.punctuation(";");
                        self.output.push('\n');
                    }
                }
                self.end();
            }
            Item::Enum(e) => {
                self.node("enum");
                self.emit_attrs(&e.attrs);
                self.emit_visibility(&e.vis);
                self.keyword("enum");
                self.plain(" ");
                self.entity(&e.ident.to_string());
                self.emit_generics(&e.generics);
                self.plain(" {\n");
                for variant in &e.variants {
                    self.node("variant");
                    self.emit_attrs(&variant.attrs);
                    self.plain("    ");
                    self.entity(&variant.ident.to_string());
                    match &variant.fields {
                        syn::Fields::Named(fields) => {
                            self.plain(" {\n");
                            for field in &fields.named {
                                self.node("field");
                                self.plain("        ");
                                self.variable(&field.ident.as_ref().unwrap().to_string());
                                self.punctuation(":");
                                self.plain(" ");
                                self.emit_type(&field.ty);
                                self.plain(",\n");
                                self.end();
                            }
                            self.plain("    }");
                        }
                        syn::Fields::Unnamed(fields) => {
                            self.punctuation("(");
                            for (i, field) in fields.unnamed.iter().enumerate() {
                                if i > 0 {
                                    self.plain(", ");
                                }
                                self.node("field");
                                self.emit_type(&field.ty);
                                self.end();
                            }
                            self.punctuation(")");
                        }
                        syn::Fields::Unit => {}
                    }
                    self.plain(",\n");
                    self.end();
                }
                self.plain("}\n");
                self.end();
            }
            Item::Impl(imp) => {
                self.node("impl");
                self.emit_attrs(&imp.attrs);
                if imp.unsafety.is_some() {
                    self.storage("unsafe");
                    self.plain(" ");
                }
                self.keyword("impl");
                self.emit_generics(&imp.generics);
                self.plain(" ");
                if let Some((_, path, _)) = &imp.trait_ {
                    self.emit_path(path);
                    self.plain(" ");
                    self.keyword("for");
                    self.plain(" ");
                }
                self.emit_type(&imp.self_ty);
                if let Some(clause) = &imp.generics.where_clause {
                    self.emit_where_clause(clause);
                }
                self.plain(" {\n");
                for item in &imp.items {
                    match item {
                        syn::ImplItem::Fn(f) => {
                            self.node("function");
                            self.emit_attrs(&f.attrs);
                            self.plain("    ");
                            self.emit_visibility(&f.vis);
                            self.emit_signature(&f.sig);
                            self.node("block");
                            self.plain(" {\n");
                            for stmt in &f.block.stmts {
                                self.plain("    ");
                                self.emit_statement(stmt, "    ");
                            }
                            self.plain("    }\n");
                            self.end();
                            self.end();
                        }
                        syn::ImplItem::Type(t) => {
                            self.node("alias");
                            self.plain("    ");
                            self.keyword("type");
                            self.plain(" ");
                            self.entity(&t.ident.to_string());
                            self.plain(" ");
                            self.operator("=");
                            self.plain(" ");
                            self.emit_type(&t.ty);
                            self.punctuation(";");
                            self.output.push('\n');
                            self.end();
                        }
                        syn::ImplItem::Const(c) => {
                            self.node("const");
                            self.plain("    ");
                            self.storage("const");
                            self.plain(" ");
                            self.constant(&c.ident.to_string());
                            self.punctuation(":");
                            self.plain(" ");
                            self.emit_type(&c.ty);
                            self.plain(" ");
                            self.operator("=");
                            self.plain(" ");
                            self.emit_expr(&c.expr);
                            self.punctuation(";");
                            self.output.push('\n');
                            self.end();
                        }
                        _ => {}
                    }
                }
                self.plain("}\n");
                self.end();
            }
            Item::Trait(t) => {
                self.node("trait");
                self.emit_attrs(&t.attrs);
                self.emit_visibility(&t.vis);
                if t.unsafety.is_some() {
                    self.storage("unsafe");
                    self.plain(" ");
                }
                self.keyword("trait");
                self.plain(" ");
                self.entity(&t.ident.to_string());
                self.emit_generics(&t.generics);
                self.plain(" {\n");
                for item in &t.items {
                    match item {
                        syn::TraitItem::Fn(f) => {
                            self.node("function");
                            self.plain("    ");
                            self.keyword("fn");
                            self.plain(" ");
                            self.entity(&f.sig.ident.to_string());
                            self.emit_generics(&f.sig.generics);
                            self.node("parameters");
                            self.punctuation("(");
                            for (i, arg) in f.sig.inputs.iter().enumerate() {
                                if i > 0 {
                                    self.plain(", ");
                                }
                                self.node("parameter");
                                match arg {
                                    FnArg::Receiver(r) => {
                                        if r.reference.is_some() {
                                            self.punctuation("&");
                                        }
                                        if r.mutability.is_some() {
                                            self.keyword("mut");
                                            self.plain(" ");
                                        }
                                        self.keyword("self");
                                    }
                                    FnArg::Typed(t) => {
                                        self.emit_pat(&t.pat);
                                        self.punctuation(":");
                                        self.plain(" ");
                                        self.emit_type(&t.ty);
                                    }
                                }
                                self.end();
                            }
                            self.punctuation(")");
                            self.end();
                            if let ReturnType::Type(_, ty) = &f.sig.output {
                                self.plain(" ");
                                self.operator("->");
                                self.plain(" ");
                                self.emit_type(ty);
                            }
                            self.punctuation(";");
                            self.output.push('\n');
                            self.end();
                        }
                        syn::TraitItem::Type(t) => {
                            self.node("alias");
                            self.plain("    ");
                            self.keyword("type");
                            self.plain(" ");
                            self.entity(&t.ident.to_string());
                            self.punctuation(";");
                            self.output.push('\n');
                            self.end();
                        }
                        _ => {}
                    }
                }
                self.plain("}\n");
                self.end();
            }
            Item::Const(c) => {
                self.node("const");
                self.emit_attrs(&c.attrs);
                self.emit_visibility(&c.vis);
                self.storage("const");
                self.plain(" ");
                self.constant(&c.ident.to_string());
                self.punctuation(":");
                self.plain(" ");
                self.emit_type(&c.ty);
                self.plain(" ");
                self.operator("=");
                self.plain(" ");
                self.emit_expr(&c.expr);
                self.punctuation(";");
                self.output.push('\n');
                self.end();
            }
            Item::Static(s) => {
                self.node("static");
                self.emit_attrs(&s.attrs);
                self.emit_visibility(&s.vis);
                self.storage("static");
                self.plain(" ");
                if s.mutability == syn::StaticMutability::Mut(syn::token::Mut::default()) {
                    self.keyword("mut");
                    self.plain(" ");
                }
                self.constant(&s.ident.to_string());
                self.punctuation(":");
                self.plain(" ");
                self.emit_type(&s.ty);
                self.plain(" ");
                self.operator("=");
                self.plain(" ");
                self.emit_expr(&s.expr);
                self.punctuation(";");
                self.output.push('\n');
                self.end();
            }
            Item::Type(t) => {
                self.node("alias");
                self.emit_attrs(&t.attrs);
                self.emit_visibility(&t.vis);
                self.keyword("type");
                self.plain(" ");
                self.entity(&t.ident.to_string());
                self.emit_generics(&t.generics);
                self.plain(" ");
                self.operator("=");
                self.plain(" ");
                self.emit_type(&t.ty);
                self.punctuation(";");
                self.output.push('\n');
                self.end();
            }
            Item::Mod(m) => {
                self.node("mod");
                self.emit_attrs(&m.attrs);
                self.emit_visibility(&m.vis);
                self.keyword("mod");
                self.plain(" ");
                self.plain(&m.ident.to_string());
                if let Some((_, items)) = &m.content {
                    self.plain(" {\n");
                    for item in items {
                        self.emit_item(item);
                    }
                    self.plain("}\n");
                } else {
                    self.punctuation(";");
                    self.output.push('\n');
                }
                self.end();
            }
            Item::Macro(m) => {
                self.node("macro");
                self.emit_path(&m.mac.path);
                self.macro_name("!");
                self.plain(" {\n");
                self.plain(&m.mac.tokens.to_string());
                self.plain("\n}\n");
                self.end();
            }
            _ => {
                self.plain(&item.to_token_stream().to_string());
                self.output.push('\n');
            }
        }
    }

    fn emit_use_tree(&mut self, tree: &UseTree) {
        match tree {
            UseTree::Path(path) => {
                let name = path.ident.to_string();
                if is_type_name(&name) {
                    self.entity(&name);
                } else {
                    self.plain(&name);
                }
                self.punctuation("::");
                self.emit_use_tree(&path.tree);
            }
            UseTree::Name(name) => {
                let n = name.ident.to_string();
                if is_type_name(&n) {
                    self.entity(&n);
                } else {
                    self.plain(&n);
                }
            }
            UseTree::Rename(rename) => {
                let n = rename.ident.to_string();
                if is_type_name(&n) {
                    self.entity(&n);
                } else {
                    self.plain(&n);
                }
                self.plain(" ");
                self.keyword("as");
                self.plain(" ");
                let r = rename.rename.to_string();
                if is_type_name(&r) {
                    self.entity(&r);
                } else {
                    self.plain(&r);
                }
            }
            UseTree::Glob(_) => {
                self.punctuation("*");
            }
            UseTree::Group(group) => {
                self.plain("{");
                for (i, item) in group.items.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.emit_use_tree(item);
                }
                self.plain("}");
            }
        }
    }
}

impl<'ast> Visit<'ast> for Visitor {
    fn visit_file(&mut self, node: &'ast syn::File) {
        for attr in &node.attrs {
            self.comment(&attr.to_token_stream().to_string());
            self.output.push('\n');
        }
        for (i, item) in node.items.iter().enumerate() {
            if i > 0 {
                self.output.push('\n');
            }
            self.emit_item(item);
        }
    }
}

fn chain(expression: &Expr) -> usize {
    match expression {
        Expr::MethodCall(mc) => 1 + chain(&mc.receiver),
        Expr::Await(a) => 1 + chain(&a.base),
        Expr::Try(t) => chain(&t.expr),
        Expr::Field(f) => 1 + chain(&f.base),
        _ => 0,
    }
}

enum Link<'a> {
    Method(&'a syn::ExprMethodCall),
    Field(&'a syn::ExprField),
    Await,
}

fn flatten(expression: &Expr) -> (&Expr, Vec<Link<'_>>, bool) {
    let mut links = Vec::new();
    let mut current = expression;
    let mut trailing = false;
    if let Expr::Try(t) = current {
        trailing = true;
        current = &t.expr;
    }
    loop {
        match current {
            Expr::MethodCall(mc) => {
                links.push(Link::Method(mc));
                current = &mc.receiver;
            }
            Expr::Await(a) => {
                links.push(Link::Await);
                current = &a.base;
            }
            Expr::Field(f) => {
                links.push(Link::Field(f));
                current = &f.base;
            }
            _ => break,
        }
    }
    links.reverse();
    (current, links, trailing)
}

fn is_type_name(name: &str) -> bool {
    name.chars()
        .next()
        .is_some_and(|c| c.is_uppercase() || c == '_')
}

fn is_keyword_value(name: &str) -> bool {
    matches!(name, "true" | "false" | "None" | "Some")
}

use quote::ToTokens;
