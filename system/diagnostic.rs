use record::{error, info};

#[must_use]
pub fn handler() -> miette::MietteHandler {
    miette::MietteHandlerOpts::new()
        .terminal_links(true)
        .unicode(true)
        .context_lines(3)
        .tab_width(2)
        .color(true)
        .force_graphical(true)
        .with_syntax_highlighting(highlight::Syntax)
        .build()
}

#[ctor::ctor]
fn initialize() {
    if let Err(e) = miette::set_hook(Box::new(|_| Box::new(handler()))) {
        error!("Failed to initialize miette error reporting system: {}", e);
        info!("This will affect error display quality but the program can continue.");
        info!("Consider checking your terminal capabilities or miette configuration.");
    }
}
