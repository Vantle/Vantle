use std::collections::HashMap;

use syn::{
    File, Ident, ItemFn, ItemMod, Signature,
    visit::{self, Visit},
};

use component::generation::rust::error::Error;
use component::generation::rust::schema::Cases;

pub fn generate(
    template: &str,
    data: &Cases,
    content: &str,
    path: &str,
    source: &str,
) -> Result<String, Box<Error>> {
    let mut ast = syn::parse_file(template).map_err(Error::from)?;

    let context = Context::from_ast(&ast);
    let registrations = process(data, &context, content, path)?;

    inject(&mut ast, registrations, source, path);
    Ok(prettyplease::unparse(&ast))
}

struct Context {
    functions: HashMap<String, Signature>,
}

impl Context {
    fn from_ast(ast: &File) -> Self {
        let mut collector = Collector::default();
        collector.visit_file(ast);

        Self {
            functions: collector.functions,
        }
    }
}

fn process(
    data: &Cases,
    context: &Context,
    content: &str,
    path: &str,
) -> Result<Vec<test::Registration>, Box<Error>> {
    let mut registrations = Vec::new();

    for function in &data.functions {
        if !context.functions.contains_key(&function.function.qualified) {
            let functions = context.functions.keys().cloned().collect::<Vec<String>>();

            let name = &function.function.qualified;
            let pattern = format!("\"function\": \"{name}\"");

            let (start, length) = if let Some(position) = content.find(&pattern) {
                let offset = position + pattern.find(name).unwrap_or(0);
                (offset, name.len())
            } else {
                let position = content.find(name).unwrap_or(0);
                (position, name.len())
            };

            let suggestion = similarity::nearest(name, &functions).unwrap_or_default();

            let functions = functions
                .iter()
                .map(|f| format!("  • {f}"))
                .collect::<Vec<_>>()
                .join("\n");
            let message = format!(
                "Available functions in template:\n{functions}{suggestion}\n\nTip: Check that the function name matches exactly with a function in your template file."
            );

            return Err(Box::new(Error::cases(
                path,
                content.to_string(),
                Some((start, length)),
                message,
            )));
        }

        for case in &function.cases {
            let inputs = test::Inputs {
                parameters: &function.parameters,
                returns: &function.returns,
                functions: &context.functions,
            };

            let registration = test::build(
                case,
                &function.function,
                &function.tags,
                &inputs,
                content,
                path,
            )?;

            registrations.push(registration);
        }
    }

    Ok(registrations)
}

fn inject(ast: &mut File, registrations: Vec<test::Registration>, source: &str, cases: &str) {
    let statements = registrations
        .into_iter()
        .map(registration)
        .collect::<Vec<syn::Stmt>>();

    let entry: syn::ItemFn = syn::parse_quote! {
        fn main() -> miette::Result<()> {
            use miette::IntoDiagnostic as _;

            vantle::system::command::execute(
                |arguments: &vantle::test::report::Arguments| {
                    vantle::system::observation::initialize(&arguments.sink.sink)
                },
                |arguments, runtime| {
                    let mut executor: vantle::test::report::Executor = vantle::test::report::Executor::new(arguments, #source, #cases);
                    #(#statements)*
                    executor.wait(runtime)
                },
            )
        }
    };

    ast.items.push(syn::Item::Fn(entry));
}

fn registration(reg: test::Registration) -> syn::Stmt {
    let name = &reg.name;
    let tags = reg
        .tags
        .iter()
        .map(|t| -> syn::Expr {
            syn::parse_quote! { #t }
        })
        .collect::<Vec<_>>();
    let parameters = serde_json::to_string(&reg.parameters).unwrap();
    let expected = serde_json::to_string(&reg.expected).unwrap();
    let statements = reg.statements;
    let actuals = reg.actuals;
    let comparisons = reg.comparisons;

    syn::parse_quote! {
        {
            let parameters: serde_json::Value = serde_json::from_str(#parameters).into_diagnostic()?;
            let expected: serde_json::Value = serde_json::from_str(#expected).into_diagnostic()?;
            executor.register(
                #name,
                &[#(#tags),*],
                parameters,
                expected,
                || {
                    #(#statements)*
                    let actuals: serde_json::Map<String, serde_json::Value> = #actuals.into();
                    #(#comparisons)*
                    Ok(actuals)
                },
            );
        }
    }
}

#[derive(Default)]
struct Collector {
    functions: HashMap<String, Signature>,
    module: Vec<Ident>,
}

impl Visit<'_> for Collector {
    fn visit_item_fn(&mut self, item: &ItemFn) {
        let name = item.sig.ident.to_string();
        let qualified = self.qualified(&name);

        self.functions.insert(qualified, item.sig.clone());

        visit::visit_item_fn(self, item);
    }

    fn visit_item_mod(&mut self, item: &ItemMod) {
        self.module.push(item.ident.clone());
        visit::visit_item_mod(self, item);
        self.module.pop();
    }
}

impl Collector {
    fn qualified(&self, name: &str) -> String {
        if self.module.is_empty() {
            name.to_string()
        } else {
            let path = self
                .module
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("::");
            format!("{path}::{name}")
        }
    }
}
