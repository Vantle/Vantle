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
        let mut byte_start = usize::MAX;
        let mut byte_end = 0usize;
        let mut row_start = usize::MAX;
        let mut row_end = 0usize;

        for capture in each.captures {
            if capture.index as usize == capture_index {
                let node = &capture.node;
                byte_start = byte_start.min(node.start_byte());
                byte_end = byte_end.max(node.end_byte());
                row_start = row_start.min(node.start_position().row);
                row_end = row_end.max(node.end_position().row);
            }
        }

        if byte_start < byte_end {
            results.push(Extraction {
                content: source[byte_start..byte_end].to_string(),
                start: row_start + 1,
                end: row_end + 1,
            });
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
