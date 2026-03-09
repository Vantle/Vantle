use element::Element;

#[must_use]
pub fn parse(source: &str) -> Vec<Element> {
    let mut elements = Vec::new();
    let mut buffer = String::new();
    let mut heading: Option<(u8, String)> = None;

    for event in pulldown_cmark::Parser::new(source) {
        match event {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Heading { level, .. }) => {
                flush(&mut buffer, &mut elements);
                heading = Some((level as u8, String::new()));
            }
            pulldown_cmark::Event::End(pulldown_cmark::TagEnd::Heading(_)) => {
                if let Some((depth, text)) = heading.take() {
                    elements.push(Element::Tag {
                        name: element::HEADINGS[(depth.clamp(1, 6) - 1) as usize].into(),
                        attributes: Vec::new(),
                        children: vec![Element::Text(text)],
                    });
                }
            }
            _ if heading.is_some() => {
                let content = &mut heading.as_mut().unwrap().1;
                match &event {
                    pulldown_cmark::Event::Text(text) | pulldown_cmark::Event::Code(text) => {
                        content.push_str(text);
                    }
                    pulldown_cmark::Event::SoftBreak | pulldown_cmark::Event::HardBreak => {
                        content.push(' ');
                    }
                    _ => {}
                }
            }
            _ => pulldown_cmark::html::push_html(&mut buffer, std::iter::once(event)),
        }
    }

    flush(&mut buffer, &mut elements);
    elements
}

fn flush(buffer: &mut String, elements: &mut Vec<Element>) {
    if !buffer.is_empty() {
        elements.push(Element::Raw(std::mem::take(buffer)));
    }
}
