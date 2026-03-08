use syn::File;

use component::generation::rust::{error::Error, schema::Cases};

pub fn generate(
    template: &str,
    data: &Cases,
    content: &str,
    path: &str,
    source: &str,
) -> Result<String, Box<Error>> {
    let mut ast = syn::parse_file(template).map_err(Error::from)?;

    let context = context::Context::from(&ast);
    let registrations = process(data, &context, content, path)?;

    inject(&mut ast, registrations, source, path);
    Ok(prettyplease::unparse(&ast))
}

pub fn benchmark(
    template: &str,
    data: &Cases,
    specification: &performance::Specification,
    content: &str,
    path: &str,
    source: &str,
    location: &str,
) -> Result<String, Box<Error>> {
    let mut ast = syn::parse_file(template).map_err(Error::from)?;

    let context = context::Context::from(&ast);
    let registrations = harness::measure(data, specification, &context, content, path)?;

    harness::instrument(&mut ast, registrations, source, path, location);
    Ok(prettyplease::unparse(&ast))
}

fn process(
    data: &Cases,
    context: &context::Context,
    content: &str,
    path: &str,
) -> Result<Vec<function::Registration>, Box<Error>> {
    let mut registrations = Vec::new();

    for function in &data.functions {
        if !context.functions.contains_key(&function.function.qualified) {
            let functions = context.functions.keys().cloned().collect::<Vec<String>>();

            let name = &function.function.qualified;
            let pattern = format!("\"function\": \"{name}\"");

            let span = if let Some(position) = content.find(&pattern) {
                let offset = position + pattern.find(name).unwrap_or(0);
                Some((offset, name.len()))
            } else {
                content.find(name).map(|position| (position, name.len()))
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
                span,
                message,
            )));
        }

        for case in &function.cases {
            let inputs = function::Inputs {
                parameters: &function.parameters,
                returns: &function.returns,
                functions: &context.functions,
            };

            let registration = function::build(
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

fn inject(ast: &mut File, registrations: Vec<function::Registration>, source: &str, cases: &str) {
    let statements = registrations
        .into_iter()
        .map(emit)
        .collect::<Vec<syn::Stmt>>();

    let entry: syn::ItemFn = syn::parse_quote! {
        fn main() -> miette::Result<()> {
            use miette::IntoDiagnostic as _;

            command::execute(
                |arguments: &function::Arguments| {
                    observation::initialize(&arguments.sink.sink)
                },
                |arguments, runtime| {
                    let mut executor: function::Executor = function::Executor::new(arguments, #source, #cases);
                    #(#statements)*
                    executor.wait(runtime)
                },
            )
        }
    };

    ast.items.push(syn::Item::Fn(entry));
}

fn emit(reg: function::Registration) -> syn::Stmt {
    let name = &reg.name;
    let tags = reg
        .tags
        .iter()
        .map(|t| -> syn::Expr {
            syn::parse_quote! { #t }
        })
        .collect::<Vec<_>>();
    let parameters = serde_json::to_string(&reg.parameters).expect("Value is always serializable");
    let expected = serde_json::to_string(&reg.expected).expect("Value is always serializable");
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
