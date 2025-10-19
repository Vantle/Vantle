use cached::proc_macro::once;
use miette::IntoDiagnostic;
use syntect::parsing::SyntaxDefinition;

#[once]
pub fn syntax() -> SyntaxDefinition {
    let content = std::fs::read_to_string("Molten/resource/system/graph/syntax.yaml")
        .into_diagnostic()
        .unwrap_or_else(|error| {
            eprintln!("{:?}", error);
            std::process::exit(1);
        });
    SyntaxDefinition::load_from_str(&content, false, None)
        .into_diagnostic()
        .unwrap_or_else(|error| {
            eprintln!("{:?}", error);
            std::process::exit(1);
        })
}
