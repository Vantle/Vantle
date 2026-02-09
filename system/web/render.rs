use std::collections::HashMap;
use std::fmt::Write;
use std::path::PathBuf;

use clap::Parser;
use element::{Element, Language};
use highlight::Highlighter;
use observe::trace;
use page::Page;
use span::Fragment;
use style::{Media, Properties, Style};

#[derive(Parser)]
#[command(
    name = "document",
    about = "Generate a document from a Rust DSL page definition"
)]
pub struct Arguments {
    #[arg(long)]
    output: PathBuf,

    #[arg(long)]
    destination: String,

    #[arg(long)]
    data: Vec<String>,
}

impl Arguments {
    #[must_use]
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    #[must_use]
    pub fn root(&self) -> String {
        let depth = self.destination.chars().filter(|&c| c == '/').count();
        if depth == 0 {
            "./".into()
        } else {
            "../".repeat(depth)
        }
    }
}

#[trace(channels = [document])]
pub fn render<S: std::hash::BuildHasher>(
    page: &Page,
    data: &HashMap<String, String, S>,
) -> miette::Result<String> {
    let highlighter = Highlighter::new();
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    writeln!(html, "<title>{}</title>", escape(&page.title)).unwrap();

    if let Some(ref path) = page.stylesheet {
        writeln!(html, "<link rel=\"stylesheet\" href=\"{}\">", escape(path)).unwrap();
    }

    html.push_str("</head>\n<body>\n");

    for element in &page.body {
        render_element(&mut html, element, data, &highlighter)?;
    }

    if let Some(ref wasm) = page.wasm {
        write!(
            html,
            "<script type=\"module\">\nimport init from '{wasm}';\nawait init();\n</script>\n"
        )
        .unwrap();
    }

    html.push_str("</body>\n</html>\n");

    Ok(html)
}

#[trace(channels = [document])]
#[must_use]
pub fn css(style: &Style) -> String {
    let mut output = String::new();
    render_style(&mut output, style);
    output
}

#[trace(channels = [document])]
pub fn generate(arguments: &Arguments, page: Page) -> miette::Result<()> {
    let data = load(&arguments.data)?;
    let html = render(&page, &data)?;
    emit(&arguments.output, &html)
}

#[trace(channels = [document])]
pub fn stylesheet(arguments: &Arguments, style: &Style) -> miette::Result<()> {
    emit(&arguments.output, &css(style))
}

#[trace(channels = [document])]
fn emit(path: &std::path::Path, content: &str) -> miette::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|_| error::Error::Output {
            path: parent.display().to_string(),
        })?;
    }

    std::fs::write(path, content).map_err(|_| error::Error::Output {
        path: path.display().to_string(),
    })?;

    Ok(())
}

#[trace(channels = [document])]
fn load(paths: &[String]) -> miette::Result<HashMap<String, String>> {
    let mut data = HashMap::new();
    for path in paths {
        let content = std::fs::read_to_string(path).map_err(|_| {
            let available = data.keys().cloned().collect::<Vec<_>>();
            error::Error::source(path, &available)
        })?;
        let name = std::path::Path::new(path)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        data.insert(name, content);
    }
    Ok(data)
}

#[trace(channels = [document])]
fn render_style(html: &mut String, style: &Style) {
    if !style.variables.is_empty() {
        html.push_str(":root {\n");
        for (name, value) in &style.variables {
            writeln!(html, "  {name}: {value};").unwrap();
        }
        html.push_str("}\n");
    }

    for rule in &style.rules {
        writeln!(html, "{} {{", rule.selector).unwrap();
        render_properties(html, &rule.properties);
        html.push_str("}\n");
    }

    for media in &style.media {
        render_media(html, media);
    }
}

#[trace(channels = [document])]
fn render_properties(html: &mut String, properties: &Properties) {
    for (name, value) in &properties.entries {
        writeln!(html, "  {name}: {value};").unwrap();
    }
}

#[trace(channels = [document])]
fn render_media(html: &mut String, media: &Media) {
    writeln!(html, "@media ({}) {{", media.query).unwrap();

    if !media.style.variables.is_empty() {
        html.push_str("  :root {\n");
        for (name, value) in &media.style.variables {
            writeln!(html, "    {name}: {value};").unwrap();
        }
        html.push_str("  }\n");
    }

    for rule in &media.style.rules {
        writeln!(html, "  {} {{", rule.selector).unwrap();
        for (name, value) in &rule.properties.entries {
            writeln!(html, "    {name}: {value};").unwrap();
        }
        html.push_str("  }\n");
    }

    html.push_str("}\n");
}

#[trace(channels = [document])]
fn render_element<S: std::hash::BuildHasher>(
    html: &mut String,
    element: &Element,
    data: &HashMap<String, String, S>,
    highlighter: &Highlighter,
) -> miette::Result<()> {
    match element {
        Element::Tag {
            name,
            attributes,
            children,
        } => {
            render_tag(html, name, attributes, children, data, highlighter)?;
        }
        Element::Text(text) => {
            html.push_str(&escape(text));
        }
        Element::Span(fragments) => {
            render_fragments(html, fragments);
        }
        Element::Raw(raw) => {
            html.push_str(raw);
        }
        Element::Code { name, language } => {
            render_code(html, name, *language, data, highlighter)?;
        }
        Element::Inject { name } => {
            render_inject(html, name, data)?;
        }
    }
    Ok(())
}

#[trace(channels = [document])]
fn render_tag<S: std::hash::BuildHasher>(
    html: &mut String,
    name: &str,
    attributes: &[(String, String)],
    children: &[Element],
    data: &HashMap<String, String, S>,
    highlighter: &Highlighter,
) -> miette::Result<()> {
    html.push('<');
    html.push_str(name);
    for (key, value) in attributes {
        write!(html, " {key}=\"{}\"", escape(value)).unwrap();
    }

    match name {
        "img" | "br" | "hr" | "input" | "meta" | "link" => {
            html.push('>');
        }
        _ => {
            html.push('>');
            for child in children {
                render_element(html, child, data, highlighter)?;
            }
            write!(html, "</{name}>").unwrap();
        }
    }

    Ok(())
}

#[trace(channels = [document])]
fn render_fragments(html: &mut String, fragments: &[Fragment]) {
    for fragment in fragments {
        match fragment {
            Fragment::Text(text) => html.push_str(&escape(text)),
            Fragment::Bold(text) => write!(html, "<strong>{}</strong>", escape(text)).unwrap(),
            Fragment::Italic(text) => write!(html, "<em>{}</em>", escape(text)).unwrap(),
            Fragment::Code(text) => write!(html, "<code>{}</code>", escape(text)).unwrap(),
            Fragment::Link { href, text } => {
                write!(html, "<a href=\"{}\">{}</a>", escape(href), escape(text)).unwrap();
            }
        }
    }
}

#[trace(channels = [document])]
fn render_code<S: std::hash::BuildHasher>(
    html: &mut String,
    name: &str,
    language: Language,
    data: &HashMap<String, String, S>,
    highlighter: &Highlighter,
) -> miette::Result<()> {
    let source = data.get(name).ok_or_else(|| {
        let available = data.keys().cloned().collect::<Vec<_>>();
        error::Error::source(name, &available)
    })?;

    let highlighted = highlighter.highlight(source, language)?;
    write!(
        html,
        "<div class=\"code-block\" data-language=\"{}\">{}</div>",
        language.name(),
        highlighted
    )
    .unwrap();

    Ok(())
}

#[trace(channels = [document])]
fn render_inject<S: std::hash::BuildHasher>(
    html: &mut String,
    name: &str,
    data: &HashMap<String, String, S>,
) -> miette::Result<()> {
    let source = data.get(name).ok_or_else(|| {
        let available = data.keys().cloned().collect::<Vec<_>>();
        error::Error::source(name, &available)
    })?;

    html.push_str(source);

    Ok(())
}

#[trace(channels = [document])]
fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
