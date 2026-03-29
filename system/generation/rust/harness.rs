use syn::File;

use context::Context;
use error::Error;
use schema::Cases;

pub struct Harness {
    pub name: String,
    pub dimensions: Vec<String>,
    pub warmup: usize,
    pub iterations: usize,
    pub cases: Vec<Extraction>,
}

pub struct Extraction {
    pub inner: function::Registration,
    pub extractions: Vec<syn::Expr>,
}

pub fn measure(
    data: &Cases,
    specification: &performance::Specification,
    context: &Context,
    content: &str,
    path: &str,
) -> Result<Vec<Harness>, Box<Error>> {
    let mut registrations = Vec::new();

    for entry in &specification.functions {
        let target = &entry.function;

        if !context.functions.contains_key(&target.qualified) {
            let functions = context.functions.keys().cloned().collect::<Vec<String>>();
            let suggestion = similarity::nearest(&target.qualified, &functions).unwrap_or_default();
            let functions = functions
                .iter()
                .map(|f| format!("  • {f}"))
                .collect::<Vec<_>>()
                .join("\n");
            return Err(Box::new(Error::cases(
                path,
                content.to_string(),
                None,
                format!("Available functions in template:\n{functions}{suggestion}"),
            )));
        }

        let definition = data
            .functions
            .iter()
            .find(|f| f.function.qualified == target.qualified)
            .ok_or_else(|| {
                Box::new(Error::Missing {
                    field: target.qualified.clone(),
                    context: "cases.json - function required by performance specification"
                        .to_string(),
                })
            })?;

        let selector = expression::parse(&entry.select).map_err(|e| {
            Box::new(Error::Missing {
                field: entry.select.clone(),
                context: format!("invalid select expression: {e}"),
            })
        })?;

        let mut cases = Vec::new();
        for case in &definition.cases {
            let tags: Vec<String> = {
                let mut t = definition.tags.clone();
                t.extend_from_slice(&case.tags);
                t
            };

            if !selector.evaluate(&tags) {
                continue;
            }

            let inputs = function::Inputs {
                parameters: &definition.parameters,
                returns: &definition.returns,
                functions: &context.functions,
            };

            let registration = function::build(
                case,
                &definition.function,
                &definition.tags,
                &inputs,
                content,
                path,
            )?;

            let mut extractions = Vec::new();
            for (parameter, measured) in &entry.measure {
                let ident = syn::Ident::new(parameter, proc_macro2::Span::call_site());
                let extraction: syn::Expr = match measured {
                    performance::Measure::Length | performance::Measure::Keys => {
                        syn::parse_quote! { performance::dimension(#ident.len()) }
                    }
                    performance::Measure::Value => {
                        syn::parse_quote! { performance::dimension(#ident) }
                    }
                };
                extractions.push(extraction);
            }

            cases.push(Extraction {
                inner: registration,
                extractions,
            });
        }

        let dimensions = entry.measure.keys().cloned().collect::<Vec<_>>();

        registrations.push(Harness {
            name: target.qualified.replace("::", "."),
            dimensions,
            warmup: entry.sampling.warmup,
            iterations: entry.sampling.iterations,
            cases,
        });
    }

    Ok(registrations)
}

pub fn instrument(
    ast: &mut File,
    registrations: Vec<Harness>,
    source: &str,
    cases: &str,
    specification: &str,
) {
    let mut statements = Vec::<syn::Stmt>::new();

    for reg in registrations {
        let name = &reg.name;
        let dimensions = &reg.dimensions;
        let warmup = reg.warmup;
        let iterations = reg.iterations;

        let mut samples = Vec::<syn::Stmt>::new();
        let mut tags = Vec::<String>::new();

        for case in reg.cases {
            let inner = case.inner;
            let extractions = case.extractions;
            let body = inner.statements;

            for tag in &inner.tags {
                if !tags.contains(tag) {
                    tags.push(tag.clone());
                }
            }

            let sample: syn::Stmt = syn::parse_quote! {
                {
                    for iteration in 0..(#warmup + #iterations) {
                        #(#body)*
                        std::hint::black_box(&result);

                        let point: Vec<f64> = vec![#(#extractions),*];

                        let start = std::time::Instant::now();
                        #(#body)*
                        std::hint::black_box(&result);
                        let elapsed = start.elapsed().as_secs_f64();

                        if iteration >= #warmup {
                            timings.push(performance::Timing {
                                point,
                                observation: elapsed,
                            });
                        }
                    }
                }
            };

            samples.push(sample);
        }

        let stmt: syn::Stmt = syn::parse_quote! {
            {
                let mut timings: Vec<performance::Timing> = Vec::new();
                #(#samples)*
                sampler.register(performance::Measured {
                    name: #name.to_string(),
                    tags: vec![#(#tags.to_string()),*],
                    dimensions: vec![#(#dimensions.to_string()),*],
                    timings,
                });
            }
        };

        statements.push(stmt);
    }

    let entry: syn::ItemFn = syn::parse_quote! {
        fn main() -> miette::Result<()> {
            command::execute(
                |arguments: &performance::Arguments| {
                    observation::initialize(&arguments.sink.sink)
                },
                |arguments, runtime| {
                    let mut sampler: performance::Sampler = performance::Sampler::new(arguments, #source, #cases, #specification);
                    #(#statements)*
                    sampler.wait(runtime)
                },
            )
        }
    };

    ast.items.push(syn::Item::Fn(entry));
}
