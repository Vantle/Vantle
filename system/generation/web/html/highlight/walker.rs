use std::fmt::Write;

#[must_use]
pub fn highlight(
    tree: &tree_sitter::Tree,
    source: &str,
    classify: fn(&tree_sitter::Node, &str) -> Option<&'static str>,
) -> String {
    let mut output = String::with_capacity(source.len() * 2);
    let mut position = 0;
    traverse(
        tree.root_node(),
        source,
        &mut output,
        &mut position,
        classify,
    );
    if position < source.len() {
        output.push_str(&escape::escape(&source[position..]));
    }
    output
}

fn traverse(
    node: tree_sitter::Node,
    source: &str,
    output: &mut String,
    position: &mut usize,
    classify: fn(&tree_sitter::Node, &str) -> Option<&'static str>,
) {
    if node.child_count() == 0 {
        let start = node.start_byte();
        let end = node.end_byte();
        if start > *position {
            output.push_str(&escape::escape(&source[*position..start]));
        }
        if start < end && end <= source.len() {
            let text = &source[start..end];
            match classify(&node, text) {
                Some(class) => {
                    write!(
                        output,
                        "<span class=\"{class}\">{}</span>",
                        escape::escape(text)
                    )
                    .unwrap();
                }
                None => output.push_str(&escape::escape(text)),
            }
        }
        *position = end;
        return;
    }

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            traverse(child, source, output, position, classify);
        }
    }
}
