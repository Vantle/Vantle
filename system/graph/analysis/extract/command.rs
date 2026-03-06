use clap::Parser;
use language::Language;

#[derive(Parser)]
#[command(
    name = "extract",
    about = "Extract code snippets via tree-sitter queries"
)]
struct Arguments {
    #[arg(long, help = "Source file to extract from")]
    source: std::path::PathBuf,

    #[arg(long, help = "Tree-sitter query with @capture")]
    query: String,

    #[arg(long, help = "Source language")]
    language: String,

    #[arg(long, help = "Output file path")]
    output: std::path::PathBuf,

    #[arg(long, help = "Workspace-relative source path for metadata")]
    label: String,
}

fn raw(content: &str) -> String {
    let mut hashes = 0;
    loop {
        let delimiter = "#".repeat(hashes);
        let closing = format!("\"{delimiter}");
        if !content.contains(&closing) {
            return format!("r{delimiter}\"{content}\"{delimiter}");
        }
        hashes += 1;
    }
}

fn main() -> miette::Result<()> {
    let arguments = Arguments::parse();

    let source = std::fs::read_to_string(&arguments.source).map_err(|source| {
        extract::error::Error::Read {
            path: arguments.source.display().to_string(),
            source,
        }
    })?;

    let parsed = Language::parse(&arguments.language)?;
    let variant = parsed.variant();

    let extractions = if arguments.query.is_empty() {
        let end = source.lines().count();
        vec![extract::Extraction {
            content: source,
            start: 1,
            end,
        }]
    } else {
        extract::extract(&source, &arguments.query, parsed)?
    };

    let mut output = String::from("pub static EXTRACTIONS: &[extraction::Extraction] = &[\n");

    for each in &extractions {
        let content = raw(&each.content);
        std::fmt::Write::write_fmt(
            &mut output,
            format_args!(
                "    extraction::Extraction {{\n        \
                 name: \"{label}\",\n        \
                 content: {content},\n        \
                 start: {start},\n        \
                 end: {end},\n        \
                 language: language::Language::{variant},\n    \
                 }},\n",
                label = arguments.label,
                start = each.start,
                end = each.end,
            ),
        )
        .unwrap();
    }

    output.push_str("];\n");

    if let Some(parent) = arguments.output.parent() {
        std::fs::create_dir_all(parent).map_err(|source| extract::error::Error::Output {
            path: parent.display().to_string(),
            source,
        })?;
    }

    std::fs::write(&arguments.output, &output).map_err(|source| extract::error::Error::Output {
        path: arguments.output.display().to_string(),
        source,
    })?;

    Ok(())
}
