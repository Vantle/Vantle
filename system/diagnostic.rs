use log::{error, info};
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSetBuilder;

pub fn handler() -> miette::MietteHandler {
    let mut builder = SyntaxSetBuilder::new();
    builder.add(highlighter::syntax());
    let syntax = builder.build();

    let set = ThemeSet::load_defaults();
    let theme = set
        .themes
        .get("base16-mocha.dark")
        .cloned()
        .unwrap_or_else(|| set.themes.values().next().unwrap().clone());

    let molten = miette::highlighters::SyntectHighlighter::new(syntax, theme, false);

    miette::MietteHandlerOpts::new()
        .terminal_links(true)
        .unicode(true)
        .context_lines(3)
        .tab_width(2)
        .color(true)
        .force_graphical(true)
        .with_syntax_highlighting(molten)
        .build()
}

#[ctor::ctor]
fn initialize() {
    miette::set_hook(Box::new(|_| Box::new(handler()))).unwrap_or_else(|e| {
        error!("Failed to initialize miette error reporting system: {}", e);
        info!("This will affect error display quality but the program can continue.");
        info!("Consider checking your terminal capabilities or miette configuration.");
    });
}
