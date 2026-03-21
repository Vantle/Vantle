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
        visitor.statement(stmt, "");
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
            "<span class=\"syntax {class}\">{}</span>",
            escape::escape(text)
        )
        .unwrap();
    }

    fn literal(&mut self, class: &str, word: &str) {
        write!(
            self.output,
            "<span class=\"syntax {class}\">{}</span>",
            escape::escape(word)
        )
        .unwrap();
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
    fn macros(&mut self, text: &str) {
        self.token("macro", text);
    }
    fn comment(&mut self, text: &str) {
        self.token("comment", text);
    }

    fn plain(&mut self, text: &str) {
        escape::stream(&mut self.output, text);
    }

    fn node(&mut self, class: &str) {
        self.output.push_str("<span class=\"node-");
        escape::stream(&mut self.output, class);
        self.output.push_str("\">");
    }

    fn end(&mut self) {
        self.output.push_str("</span>");
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

    fn chained(&mut self, expression: &Expr) {
        let (root, links, trailing) = flatten(expression);
        let indent = self.leading();
        self.expression(root);
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
                                GenericArgument::Type(t) => self.typed(t),
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
                        self.expression(arg);
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

    fn attributes(&mut self, attrs: &[syn::Attribute]) {
        if attrs.is_empty() {
            return;
        }
        self.node("attributes");
        for attr in attrs {
            self.macros(&attr.to_token_stream().to_string());
            self.output.push('\n');
        }
        self.end();
    }

    fn visibility(&mut self, vis: &syn::Visibility) {
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

    fn generics(&mut self, generics: &syn::Generics) {
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
                            self.bound(bound);
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
                        self.typed(&c.ty);
                    }
                }
            }
            self.punctuation(">");
            self.end();
        }
    }

    fn bound(&mut self, bound: &syn::TypeParamBound) {
        match bound {
            syn::TypeParamBound::Trait(t) => {
                self.path(&t.path);
            }
            syn::TypeParamBound::Lifetime(l) => {
                self.storage(&format!("'{}", l.ident));
            }
            _ => {
                self.plain(&bound.to_token_stream().to_string());
            }
        }
    }

    fn clause(&mut self, clause: &syn::WhereClause) {
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
                    self.typed(&t.bounded_ty);
                    self.punctuation(": ");
                    for (j, bound) in t.bounds.iter().enumerate() {
                        if j > 0 {
                            self.plain(" + ");
                        }
                        self.bound(bound);
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

    fn arguments(&mut self, args: &PathArguments, turbofish: bool) {
        match args {
            PathArguments::AngleBracketed(args) => {
                self.punctuation(if turbofish { "::<" } else { "<" });
                for (i, arg) in args.args.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    match arg {
                        GenericArgument::Type(t) => self.typed(t),
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
                for (i, input) in args.inputs.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.typed(input);
                }
                self.punctuation(")");
                if let ReturnType::Type(_, ty) = &args.output {
                    self.plain(" ");
                    self.operator("->");
                    self.plain(" ");
                    self.typed(ty);
                }
            }
            PathArguments::None => {}
        }
    }

    fn path(&mut self, path: &syn::Path) {
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
            self.arguments(&segment.arguments, true);
        }
        self.end();
    }

    fn typed(&mut self, ty: &Type) {
        self.node("type");
        match ty {
            Type::Path(path) => {
                if let Some(qself) = &path.qself {
                    self.punctuation("<");
                    self.typed(&qself.ty);
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
                        self.arguments(&segment.arguments, false);
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
                self.typed(&r.elem);
            }
            Type::Slice(s) => {
                self.punctuation("[");
                self.typed(&s.elem);
                self.punctuation("]");
            }
            Type::Array(a) => {
                self.punctuation("[");
                self.typed(&a.elem);
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
                    self.typed(elem);
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
                    self.bound(bound);
                }
            }
            Type::TraitObject(obj) => {
                self.keyword("dyn");
                self.plain(" ");
                for (i, bound) in obj.bounds.iter().enumerate() {
                    if i > 0 {
                        self.plain(" + ");
                    }
                    self.bound(bound);
                }
            }
            Type::Infer(_) => self.plain("_"),
            Type::Never(_) => self.punctuation("!"),
            _ => self.plain(&ty.to_token_stream().to_string()),
        }
        self.end();
    }

    fn expression(&mut self, expr: &Expr) {
        self.node("expression");
        match expr {
            Expr::Lit(lit) => self.value(&lit.lit),
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
                    self.path(&path.path);
                }
            }
            Expr::Call(call) => {
                self.expression(&call.func);
                self.punctuation("(");
                for (i, arg) in call.args.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.expression(arg);
                }
                self.punctuation(")");
            }
            Expr::MethodCall(_) | Expr::Field(_) | Expr::Try(_) | Expr::Await(_)
                if chain(expr) >= 3 =>
            {
                self.chained(expr);
            }
            Expr::MethodCall(mc) => {
                self.expression(&mc.receiver);
                self.punctuation(".");
                self.function(&mc.method.to_string());
                if let Some(turbofish) = &mc.turbofish {
                    self.punctuation("::<");
                    for (i, arg) in turbofish.args.iter().enumerate() {
                        if i > 0 {
                            self.plain(", ");
                        }
                        match arg {
                            GenericArgument::Type(t) => self.typed(t),
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
                    self.expression(arg);
                }
                self.punctuation(")");
            }
            Expr::Reference(r) => {
                self.punctuation("&");
                if r.mutability.is_some() {
                    self.keyword("mut");
                    self.plain(" ");
                }
                self.expression(&r.expr);
            }
            Expr::Block(block) => {
                let indent = self.leading();
                let outer = &indent[..indent.len().saturating_sub(4)];
                self.plain(" {\n");
                for stmt in &block.block.stmts {
                    self.statement(stmt, &indent);
                }
                self.output.push_str(outer);
                self.plain("}");
            }
            Expr::If(expr_if) => {
                let indent = self.leading();
                let outer = &indent[..indent.len().saturating_sub(4)];
                self.keyword("if");
                self.plain(" ");
                self.expression(&expr_if.cond);
                self.plain(" {\n");
                for stmt in &expr_if.then_branch.stmts {
                    self.statement(stmt, &indent);
                }
                self.output.push_str(outer);
                self.plain("}");
                if let Some((_, else_branch)) = &expr_if.else_branch {
                    self.plain(" ");
                    self.keyword("else");
                    self.plain(" ");
                    self.expression(else_branch);
                }
            }
            Expr::Match(m) => {
                let indent = self.leading();
                let outer = &indent[..indent.len().saturating_sub(4)];
                self.keyword("match");
                self.plain(" ");
                self.expression(&m.expr);
                self.plain(" {\n");
                for arm in &m.arms {
                    self.output.push_str(&indent);
                    self.pattern(&arm.pat);
                    self.plain(" ");
                    self.operator("=>");
                    self.plain(" ");
                    self.expression(&arm.body);
                    self.plain(",\n");
                }
                self.output.push_str(outer);
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
                    self.pattern(input);
                }
                self.punctuation("|");
                self.plain(" ");
                if let Expr::Block(block) = &*c.body {
                    let indent = self.leading();
                    let outer = &indent[..indent.len().saturating_sub(4)];
                    self.plain("{\n");
                    for stmt in &block.block.stmts {
                        self.statement(stmt, &indent);
                    }
                    self.output.push_str(outer);
                    self.plain("}");
                } else {
                    self.expression(&c.body);
                }
            }
            Expr::Struct(s) => {
                let indent = self.leading();
                let outer = &indent[..indent.len().saturating_sub(4)];
                self.path(&s.path);
                self.plain(" {\n");
                for (i, field) in s.fields.iter().enumerate() {
                    if i > 0 {
                        self.plain(",\n");
                    }
                    self.output.push_str(&indent);
                    if field.colon_token.is_some() {
                        self.variable(&field.member.to_token_stream().to_string());
                        self.punctuation(":");
                        self.plain(" ");
                    }
                    self.expression(&field.expr);
                }
                self.output.push('\n');
                self.output.push_str(outer);
                self.plain("}");
            }
            Expr::Field(f) => {
                self.expression(&f.base);
                self.punctuation(".");
                self.variable(&f.member.to_token_stream().to_string());
            }
            Expr::Index(idx) => {
                self.expression(&idx.expr);
                self.punctuation("[");
                self.expression(&idx.index);
                self.punctuation("]");
            }
            Expr::Unary(u) => {
                self.operator(&u.op.to_token_stream().to_string());
                self.expression(&u.expr);
            }
            Expr::Binary(b) => {
                self.expression(&b.left);
                self.plain(" ");
                self.operator(&b.op.to_token_stream().to_string());
                self.plain(" ");
                self.expression(&b.right);
            }
            Expr::Let(l) => {
                self.keyword("let");
                self.plain(" ");
                self.pattern(&l.pat);
                self.plain(" ");
                self.operator("=");
                self.plain(" ");
                self.expression(&l.expr);
            }
            Expr::Return(r) => {
                self.keyword("return");
                if let Some(expr) = &r.expr {
                    self.plain(" ");
                    self.expression(expr);
                }
            }
            Expr::Try(t) => {
                self.expression(&t.expr);
                self.operator("?");
            }
            Expr::Await(a) => {
                self.expression(&a.base);
                self.punctuation(".");
                self.keyword("await");
            }
            Expr::Paren(p) => {
                self.punctuation("(");
                self.expression(&p.expr);
                self.punctuation(")");
            }
            Expr::Tuple(t) => {
                self.punctuation("(");
                for (i, elem) in t.elems.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.expression(elem);
                }
                self.punctuation(")");
            }
            Expr::Array(a) => {
                self.punctuation("[");
                for (i, elem) in a.elems.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.expression(elem);
                }
                self.punctuation("]");
            }
            Expr::Cast(c) => {
                self.expression(&c.expr);
                self.plain(" ");
                self.keyword("as");
                self.plain(" ");
                self.typed(&c.ty);
            }
            Expr::Range(r) => {
                if let Some(start) = &r.start {
                    self.expression(start);
                }
                self.operator(&r.limits.to_token_stream().to_string());
                if let Some(end) = &r.end {
                    self.expression(end);
                }
            }
            Expr::Macro(m) => {
                self.path(&m.mac.path);
                self.macros("!");
                self.punctuation("(");
                self.plain(&m.mac.tokens.to_string());
                self.punctuation(")");
            }
            _ => self.plain(&expr.to_token_stream().to_string()),
        }
        self.end();
    }

    fn value(&mut self, lit: &Lit) {
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

    fn pattern(&mut self, pat: &Pat) {
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
            Pat::Lit(lit) => self.value(&lit.lit),
            Pat::Path(path) => self.path(&path.path),
            Pat::Tuple(t) => {
                self.punctuation("(");
                for (i, elem) in t.elems.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.pattern(elem);
                }
                self.punctuation(")");
            }
            Pat::TupleStruct(ts) => {
                self.path(&ts.path);
                self.punctuation("(");
                for (i, elem) in ts.elems.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.pattern(elem);
                }
                self.punctuation(")");
            }
            Pat::Struct(s) => {
                self.path(&s.path);
                self.plain(" { ");
                for (i, field) in s.fields.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.variable(&field.member.to_token_stream().to_string());
                    if field.colon_token.is_some() {
                        self.punctuation(": ");
                        self.pattern(&field.pat);
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
                self.pattern(&r.pat);
            }
            Pat::Slice(s) => {
                self.punctuation("[");
                for (i, elem) in s.elems.iter().enumerate() {
                    if i > 0 {
                        self.plain(", ");
                    }
                    self.pattern(elem);
                }
                self.punctuation("]");
            }
            Pat::Or(o) => {
                for (i, case) in o.cases.iter().enumerate() {
                    if i > 0 {
                        self.plain(" | ");
                    }
                    self.pattern(case);
                }
            }
            Pat::Rest(_) => self.plain(".."),
            Pat::Type(t) => {
                self.pattern(&t.pat);
                self.punctuation(": ");
                self.typed(&t.ty);
            }
            _ => self.plain(&pat.to_token_stream().to_string()),
        }
        self.end();
    }

    fn statement(&mut self, stmt: &Stmt, indent: &str) {
        match stmt {
            Stmt::Local(local) => {
                self.node("let");
                self.output.push_str(indent);
                self.keyword("let");
                self.plain(" ");
                if let Pat::Ident(ident) = &local.pat
                    && ident.mutability.is_some()
                {
                    self.keyword("mut");
                    self.plain(" ");
                    self.variable(&ident.ident.to_string());
                } else {
                    self.pattern(&local.pat);
                }
                if let Some(init) = &local.init {
                    self.plain(" ");
                    self.operator("=");
                    self.plain(" ");
                    self.expression(&init.expr);
                }
                self.punctuation(";");
                self.output.push('\n');
                self.end();
            }
            Stmt::Expr(expr, semi) => {
                self.node("statement");
                self.output.push_str(indent);
                self.expression(expr);
                if semi.is_some() {
                    self.punctuation(";");
                }
                self.output.push('\n');
                self.end();
            }
            Stmt::Item(item) => self.item(item),
            Stmt::Macro(m) => {
                self.node("statement");
                self.output.push_str(indent);
                self.path(&m.mac.path);
                self.macros("!");
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

    fn signature(&mut self, sig: &syn::Signature) {
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
        self.generics(&sig.generics);
        self.parameters(&sig.inputs);
        if let ReturnType::Type(_, ty) = &sig.output {
            self.plain(" ");
            self.operator("->");
            self.plain(" ");
            self.typed(ty);
        }
        if let Some(clause) = &sig.generics.where_clause {
            self.clause(clause);
        }
    }

    fn parameters(&mut self, inputs: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>) {
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
                    self.pattern(&t.pat);
                    self.punctuation(":");
                    self.plain(" ");
                    self.typed(&t.ty);
                }
            }
            self.end();
        }
        self.punctuation(")");
        self.end();
    }

    fn item(&mut self, item: &Item) {
        match item {
            Item::Use(u) => {
                self.node("use");
                self.attributes(&u.attrs);
                self.visibility(&u.vis);
                self.keyword("use");
                self.plain(" ");
                self.tree(&u.tree);
                self.punctuation(";");
                self.output.push('\n');
                self.end();
            }
            Item::Fn(f) => {
                self.node("function");
                self.attributes(&f.attrs);
                self.visibility(&f.vis);
                self.signature(&f.sig);
                self.node("block");
                self.plain(" {\n");
                for stmt in &f.block.stmts {
                    self.statement(stmt, "    ");
                }
                self.plain("}\n");
                self.end();
                self.end();
            }
            Item::Struct(s) => {
                self.node("struct");
                self.attributes(&s.attrs);
                self.visibility(&s.vis);
                self.keyword("struct");
                self.plain(" ");
                self.entity(&s.ident.to_string());
                self.generics(&s.generics);
                match &s.fields {
                    syn::Fields::Named(fields) => {
                        if let Some(clause) = &s.generics.where_clause {
                            self.clause(clause);
                        }
                        self.plain(" {\n");
                        for field in &fields.named {
                            self.node("field");
                            self.attributes(&field.attrs);
                            self.plain("    ");
                            self.visibility(&field.vis);
                            self.variable(&field.ident.as_ref().unwrap().to_string());
                            self.punctuation(":");
                            self.plain(" ");
                            self.typed(&field.ty);
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
                            self.visibility(&field.vis);
                            self.typed(&field.ty);
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
                self.attributes(&e.attrs);
                self.visibility(&e.vis);
                self.keyword("enum");
                self.plain(" ");
                self.entity(&e.ident.to_string());
                self.generics(&e.generics);
                self.plain(" {\n");
                for variant in &e.variants {
                    self.node("variant");
                    self.attributes(&variant.attrs);
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
                                self.typed(&field.ty);
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
                                self.typed(&field.ty);
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
                self.attributes(&imp.attrs);
                if imp.unsafety.is_some() {
                    self.storage("unsafe");
                    self.plain(" ");
                }
                self.keyword("impl");
                self.generics(&imp.generics);
                self.plain(" ");
                if let Some((_, path, _)) = &imp.trait_ {
                    self.path(path);
                    self.plain(" ");
                    self.keyword("for");
                    self.plain(" ");
                }
                self.typed(&imp.self_ty);
                if let Some(clause) = &imp.generics.where_clause {
                    self.clause(clause);
                }
                self.plain(" {\n");
                for item in &imp.items {
                    match item {
                        syn::ImplItem::Fn(f) => {
                            self.node("function");
                            self.attributes(&f.attrs);
                            self.plain("    ");
                            self.visibility(&f.vis);
                            self.signature(&f.sig);
                            self.node("block");
                            self.plain(" {\n");
                            for stmt in &f.block.stmts {
                                self.plain("    ");
                                self.statement(stmt, "    ");
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
                            self.typed(&t.ty);
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
                            self.typed(&c.ty);
                            self.plain(" ");
                            self.operator("=");
                            self.plain(" ");
                            self.expression(&c.expr);
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
                self.attributes(&t.attrs);
                self.visibility(&t.vis);
                if t.unsafety.is_some() {
                    self.storage("unsafe");
                    self.plain(" ");
                }
                self.keyword("trait");
                self.plain(" ");
                self.entity(&t.ident.to_string());
                self.generics(&t.generics);
                self.plain(" {\n");
                for item in &t.items {
                    match item {
                        syn::TraitItem::Fn(f) => {
                            self.node("function");
                            self.plain("    ");
                            self.keyword("fn");
                            self.plain(" ");
                            self.entity(&f.sig.ident.to_string());
                            self.generics(&f.sig.generics);
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
                                        self.pattern(&t.pat);
                                        self.punctuation(":");
                                        self.plain(" ");
                                        self.typed(&t.ty);
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
                                self.typed(ty);
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
                self.attributes(&c.attrs);
                self.visibility(&c.vis);
                self.storage("const");
                self.plain(" ");
                self.constant(&c.ident.to_string());
                self.punctuation(":");
                self.plain(" ");
                self.typed(&c.ty);
                self.plain(" ");
                self.operator("=");
                self.plain(" ");
                self.expression(&c.expr);
                self.punctuation(";");
                self.output.push('\n');
                self.end();
            }
            Item::Static(s) => {
                self.node("static");
                self.attributes(&s.attrs);
                self.visibility(&s.vis);
                self.storage("static");
                self.plain(" ");
                if s.mutability == syn::StaticMutability::Mut(syn::token::Mut::default()) {
                    self.keyword("mut");
                    self.plain(" ");
                }
                self.constant(&s.ident.to_string());
                self.punctuation(":");
                self.plain(" ");
                self.typed(&s.ty);
                self.plain(" ");
                self.operator("=");
                self.plain(" ");
                self.expression(&s.expr);
                self.punctuation(";");
                self.output.push('\n');
                self.end();
            }
            Item::Type(t) => {
                self.node("alias");
                self.attributes(&t.attrs);
                self.visibility(&t.vis);
                self.keyword("type");
                self.plain(" ");
                self.entity(&t.ident.to_string());
                self.generics(&t.generics);
                self.plain(" ");
                self.operator("=");
                self.plain(" ");
                self.typed(&t.ty);
                self.punctuation(";");
                self.output.push('\n');
                self.end();
            }
            Item::Mod(m) => {
                self.node("mod");
                self.attributes(&m.attrs);
                self.visibility(&m.vis);
                self.keyword("mod");
                self.plain(" ");
                self.plain(&m.ident.to_string());
                if let Some((_, items)) = &m.content {
                    self.plain(" {\n");
                    for item in items {
                        self.item(item);
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
                self.path(&m.mac.path);
                self.macros("!");
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

    fn tree(&mut self, tree: &UseTree) {
        match tree {
            UseTree::Path(path) => {
                let name = path.ident.to_string();
                if is_type_name(&name) {
                    self.entity(&name);
                } else {
                    self.plain(&name);
                }
                self.punctuation("::");
                self.tree(&path.tree);
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
                    self.tree(item);
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
            self.item(item);
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
