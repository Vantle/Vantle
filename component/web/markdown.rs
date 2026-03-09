use element::Element;

#[must_use]
pub fn parse(source: &str) -> Vec<Element> {
    let Ok(tree) = constructor::markdown(source) else {
        return vec![Element::Text(source.into())];
    };
    let mut elements = Vec::new();
    let mut cursor = tree.walk();
    if cursor.goto_first_child() {
        visit(&mut cursor, source, &mut elements);
    }
    elements
}

fn visit(cursor: &mut tree_sitter_md::MarkdownCursor, source: &str, elements: &mut Vec<Element>) {
    loop {
        match cursor.node().kind() {
            "atx_heading" => heading(cursor, source, elements),
            "paragraph" => paragraph(cursor, source, elements),
            "list" => list(cursor, source, elements),
            "block_quote" => blockquote(cursor, source, elements),
            "fenced_code_block" => code(cursor, source, elements),
            "thematic_break" => elements.push(Element::Void {
                name: "hr".into(),
                attributes: Vec::new(),
            }),
            "section" => {
                if cursor.goto_first_child() {
                    visit(cursor, source, elements);
                    cursor.goto_parent();
                }
            }
            _ => {}
        }
        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

fn heading(cursor: &mut tree_sitter_md::MarkdownCursor, source: &str, elements: &mut Vec<Element>) {
    let mut level = 1u8;
    let mut text = String::new();
    if cursor.goto_first_child() {
        loop {
            match cursor.node().kind() {
                "atx_h1_marker" => level = 1,
                "atx_h2_marker" => level = 2,
                "atx_h3_marker" => level = 3,
                "atx_h4_marker" => level = 4,
                "atx_h5_marker" => level = 5,
                "atx_h6_marker" => level = 6,
                "inline" => {
                    let range = cursor.node().byte_range();
                    if cursor.goto_first_child() {
                        collect_text(cursor, source, &mut text, range.start, range.end);
                        cursor.goto_parent();
                    } else {
                        text.push_str(&source[range]);
                    }
                }
                _ => {}
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
    elements.push(Element::Tag {
        name: element::HEADINGS[(level.clamp(1, 6) - 1) as usize].into(),
        attributes: Vec::new(),
        children: vec![Element::Text(text)],
    });
}

fn collect_text(
    cursor: &mut tree_sitter_md::MarkdownCursor,
    source: &str,
    text: &mut String,
    start: usize,
    end: usize,
) {
    let mut position = start;
    loop {
        let node = cursor.node();
        let gap = &source[position..node.start_byte()];
        if !gap.is_empty() {
            text.push_str(gap);
        }
        if node.child_count() == 0 {
            if !matches!(node.kind(), "emphasis_delimiter") {
                text.push_str(&source[node.byte_range()]);
            }
        } else if cursor.goto_first_child() {
            collect_text(cursor, source, text, node.start_byte(), node.end_byte());
            cursor.goto_parent();
        }
        position = node.end_byte();
        if !cursor.goto_next_sibling() {
            break;
        }
    }
    let trailing = &source[position..end];
    if !trailing.is_empty() {
        text.push_str(trailing);
    }
}

fn paragraph(
    cursor: &mut tree_sitter_md::MarkdownCursor,
    source: &str,
    elements: &mut Vec<Element>,
) {
    let children = inline_elements(cursor, source);
    if !children.is_empty() {
        elements.push(Element::Tag {
            name: "p".into(),
            attributes: Vec::new(),
            children,
        });
    }
}

fn list(cursor: &mut tree_sitter_md::MarkdownCursor, source: &str, elements: &mut Vec<Element>) {
    let ordered = cursor.node().child(0).is_some_and(|first| {
        let mut c = first.walk();
        first.children(&mut c).any(|child| {
            child.kind() == "list_marker_dot" || child.kind() == "list_marker_parenthesis"
        })
    });
    let tag = if ordered { "ol" } else { "ul" };
    let mut items = Vec::new();
    if cursor.goto_first_child() {
        loop {
            if cursor.node().kind() == "list_item" {
                items.push(Element::Tag {
                    name: "li".into(),
                    attributes: Vec::new(),
                    children: list_item_content(cursor, source),
                });
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
    elements.push(Element::Tag {
        name: tag.into(),
        attributes: Vec::new(),
        children: items,
    });
}

fn list_item_content(cursor: &mut tree_sitter_md::MarkdownCursor, source: &str) -> Vec<Element> {
    let mut elements = Vec::new();
    if cursor.goto_first_child() {
        loop {
            match cursor.node().kind() {
                "paragraph" => elements.extend(inline_elements(cursor, source)),
                "list" => list(cursor, source, &mut elements),
                _ => {}
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
    elements
}

fn blockquote(
    cursor: &mut tree_sitter_md::MarkdownCursor,
    source: &str,
    elements: &mut Vec<Element>,
) {
    let mut children = Vec::new();
    if cursor.goto_first_child() {
        loop {
            match cursor.node().kind() {
                "paragraph" => paragraph(cursor, source, &mut children),
                "block_quote" => blockquote(cursor, source, &mut children),
                _ => {}
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
    elements.push(Element::Tag {
        name: "blockquote".into(),
        attributes: Vec::new(),
        children,
    });
}

fn code(cursor: &mut tree_sitter_md::MarkdownCursor, source: &str, elements: &mut Vec<Element>) {
    let node = cursor.node();
    let mut content = String::new();
    let mut child_cursor = node.walk();
    for child in node.children(&mut child_cursor) {
        if child.kind() == "code_fence_content" {
            content = source[child.byte_range()].to_string();
        }
    }
    if content.ends_with('\n') {
        content.pop();
    }
    elements.push(Element::Tag {
        name: "pre".into(),
        attributes: Vec::new(),
        children: vec![Element::Tag {
            name: "code".into(),
            attributes: Vec::new(),
            children: vec![Element::Text(content)],
        }],
    });
}

fn inline_elements(cursor: &mut tree_sitter_md::MarkdownCursor, source: &str) -> Vec<Element> {
    let mut elements = Vec::new();
    if cursor.goto_first_child() {
        loop {
            if cursor.node().kind() == "inline" {
                let range = cursor.node().byte_range();
                if cursor.goto_first_child() {
                    walk_inline(cursor, source, &mut elements, range.start, range.end);
                    cursor.goto_parent();
                } else {
                    let text = &source[range];
                    if !text.is_empty() {
                        elements.push(Element::Text(text.into()));
                    }
                }
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
    elements
}

fn walk_inline(
    cursor: &mut tree_sitter_md::MarkdownCursor,
    source: &str,
    elements: &mut Vec<Element>,
    start: usize,
    end: usize,
) {
    let mut position = start;
    loop {
        let node = cursor.node();
        let gap = &source[position..node.start_byte()];
        if !gap.is_empty() {
            elements.push(Element::Text(gap.into()));
        }
        match node.kind() {
            "emphasis" => elements.push(Element::Tag {
                name: "em".into(),
                attributes: Vec::new(),
                children: emphasis_children(cursor, source),
            }),
            "strong_emphasis" => elements.push(Element::Tag {
                name: "strong".into(),
                attributes: Vec::new(),
                children: emphasis_children(cursor, source),
            }),
            "code_span" => elements.push(Element::Tag {
                name: "code".into(),
                attributes: Vec::new(),
                children: vec![Element::Text(code_span_text(node, source).into())],
            }),
            "inline_link" => link(cursor, source, elements),
            "shortcut_link" | "full_reference_link" | "collapsed_reference_link" => {
                elements.push(Element::Text(source[node.byte_range()].into()));
            }
            _ => {
                let text = &source[node.byte_range()];
                if !text.is_empty() {
                    elements.push(Element::Text(text.into()));
                }
            }
        }
        position = node.end_byte();
        if !cursor.goto_next_sibling() {
            break;
        }
    }
    let trailing = &source[position..end];
    if !trailing.is_empty() {
        elements.push(Element::Text(trailing.into()));
    }
}

fn emphasis_children(cursor: &mut tree_sitter_md::MarkdownCursor, source: &str) -> Vec<Element> {
    let mut elements = Vec::new();
    if cursor.goto_first_child() {
        let mut position = cursor.node().start_byte();
        loop {
            let node = cursor.node();
            if node.kind() == "emphasis_delimiter" {
                let gap = &source[position..node.start_byte()];
                if !gap.is_empty() {
                    elements.push(Element::Text(gap.into()));
                }
                position = node.end_byte();
                if !cursor.goto_next_sibling() {
                    break;
                }
                continue;
            }
            let gap = &source[position..node.start_byte()];
            if !gap.is_empty() {
                elements.push(Element::Text(gap.into()));
            }
            match node.kind() {
                "emphasis" => elements.push(Element::Tag {
                    name: "em".into(),
                    attributes: Vec::new(),
                    children: emphasis_children(cursor, source),
                }),
                "strong_emphasis" => elements.push(Element::Tag {
                    name: "strong".into(),
                    attributes: Vec::new(),
                    children: emphasis_children(cursor, source),
                }),
                _ => {
                    let text = &source[node.byte_range()];
                    if !text.is_empty() {
                        elements.push(Element::Text(text.into()));
                    }
                }
            }
            position = node.end_byte();
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
    elements
}

fn code_span_text<'a>(node: tree_sitter::Node, source: &'a str) -> &'a str {
    let full = &source[node.byte_range()];
    let trimmed = full.trim_start_matches('`');
    let trimmed = trimmed.trim_end_matches('`');
    if trimmed.starts_with(' ') && trimmed.ends_with(' ') && trimmed.len() > 1 {
        return &trimmed[1..trimmed.len() - 1];
    }
    trimmed
}

fn link(cursor: &mut tree_sitter_md::MarkdownCursor, source: &str, elements: &mut Vec<Element>) {
    let mut text_children = Vec::new();
    let mut href = String::new();
    if cursor.goto_first_child() {
        loop {
            match cursor.node().kind() {
                "link_text" => {
                    let content = &source[cursor.node().byte_range()];
                    let inner = content
                        .strip_prefix('[')
                        .and_then(|s| s.strip_suffix(']'))
                        .unwrap_or(content);
                    if !inner.is_empty() {
                        text_children.push(Element::Text(inner.into()));
                    }
                }
                "link_destination" => {
                    let content = &source[cursor.node().byte_range()];
                    let inner = content
                        .strip_prefix('(')
                        .and_then(|s| s.strip_suffix(')'))
                        .unwrap_or(content);
                    href = inner.into();
                }
                _ => {}
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
    elements.push(Element::Tag {
        name: "a".into(),
        attributes: vec![("href".into(), href)],
        children: text_children,
    });
}
