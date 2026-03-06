pub use error;

use language::Language;
use tree_sitter::StreamingIterator;

pub struct Extraction {
    pub content: String,
    pub start: usize,
    pub end: usize,
}

pub fn extract(
    source: impl AsRef<str>,
    query: impl AsRef<str>,
    language: Language,
) -> miette::Result<Vec<Extraction>> {
    let source = source.as_ref();
    let tree = constructor::treesitter(source, language)?;
    let query_source = query.as_ref();

    let query = tree_sitter::Query::new(&tree.language(), query_source).map_err(|e| {
        error::Error::Invalid {
            detail: e.to_string(),
        }
    })?;

    let capture_index = query
        .capture_names()
        .iter()
        .position(|name| *name == "capture")
        .ok_or(error::Error::Capture)?;

    let mut cursor = tree_sitter::QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    let mut results = Vec::new();

    while let Some(each) = matches.next() {
        for capture in each.captures {
            if capture.index as usize == capture_index {
                let text = capture
                    .node
                    .utf8_text(source.as_bytes())
                    .map_err(|_| error::Error::Parse)?;
                results.push(Extraction {
                    content: text.to_string(),
                    start: capture.node.start_position().row + 1,
                    end: capture.node.end_position().row + 1,
                });
            }
        }
    }

    if results.is_empty() {
        let names = query
            .capture_names()
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>();
        return Err(error::Error::empty(&names).into());
    }

    Ok(results)
}
