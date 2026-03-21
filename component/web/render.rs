use std::collections::HashSet;

use element::{Element, Location};
use language::Language;

pub struct Heading {
    pub depth: u8,
    pub identifier: String,
    pub text: String,
}

pub trait Emitter {
    fn open(&mut self, name: &str, attributes: &[(String, String)]) -> miette::Result<()>;
    fn close(&mut self, name: &str) -> miette::Result<()>;
    fn void(&mut self, name: &str, attributes: &[(String, String)]) -> miette::Result<()>;
    fn text(&mut self, content: &str) -> miette::Result<()>;
    fn raw(&mut self, content: &str) -> miette::Result<()>;
    fn code(
        &mut self,
        content: &str,
        language: Language,
        location: Option<&Location>,
    ) -> miette::Result<()>;
    fn indent(&mut self, _depth: usize) -> miette::Result<()> {
        Ok(())
    }
    fn newline(&mut self) -> miette::Result<()> {
        Ok(())
    }
}

#[must_use]
pub fn process(elements: &mut [Element]) -> Vec<Heading> {
    let mut headings = analyze(elements);
    let mut identifiers = HashSet::new();
    index(&mut headings, &mut identifiers);
    mutate(elements, &headings);
    headings
}

#[must_use]
pub fn analyze(elements: &[Element]) -> Vec<Heading> {
    let mut entries = Vec::new();
    scan(&mut entries, elements);
    entries
}

pub fn index<S: std::hash::BuildHasher>(
    headings: &mut [Heading],
    identifiers: &mut HashSet<String, S>,
) {
    for heading in headings.iter_mut() {
        if heading.identifier.is_empty() {
            heading.identifier = deduplicate(slugify(&heading.text), identifiers);
        }
        identifiers.insert(heading.identifier.clone());
    }
}

pub fn mutate(elements: &mut [Element], headings: &[Heading]) {
    let mut index = 0;
    inject(elements, headings, &mut index);
}

pub fn outline(elements: &mut [Element], headings: &[Heading]) {
    if headings.is_empty() {
        return;
    }
    let Some(target) = query::id_mut(elements, "outline") else {
        return;
    };
    let Element::Tag { children, .. } = target else {
        return;
    };
    let minimum = headings.iter().map(|h| h.depth).min().unwrap_or(1);
    *children = headings
        .iter()
        .map(|heading| {
            let level = heading.depth - minimum;
            Element::Tag {
                name: "a".into(),
                attributes: vec![
                    ("href".into(), format!("#{}", heading.identifier)),
                    (attribute::depth().name().into(), level.to_string()),
                ],
                children: vec![Element::Text(heading.text.clone())],
            }
        })
        .collect::<Vec<_>>();
}

pub fn render(emitter: &mut impl Emitter, elements: &[Element]) -> miette::Result<()> {
    emit(emitter, elements, 0)
}

fn scan(entries: &mut Vec<Heading>, elements: &[Element]) {
    for element in elements {
        if let Element::Tag {
            name,
            attributes,
            children,
        } = element
        {
            if let Some(depth) = level(name) {
                let text = textual(children);
                if !text.is_empty() {
                    let existing = attributes
                        .iter()
                        .find(|(k, _)| k == "id")
                        .map(|(_, v)| v.clone())
                        .unwrap_or_default();
                    entries.push(Heading {
                        depth,
                        identifier: existing,
                        text,
                    });
                }
            }
            scan(entries, children);
        }
    }
}

fn inject(elements: &mut [Element], headings: &[Heading], index: &mut usize) {
    for element in elements.iter_mut() {
        if let Element::Tag {
            name,
            attributes,
            children,
        } = element
        {
            if level(name).is_some() {
                let text = textual(children);
                if !text.is_empty() && *index < headings.len() {
                    if !attributes.iter().any(|(k, _)| k == "id") {
                        attributes.push(("id".into(), headings[*index].identifier.clone()));
                    }
                    *index += 1;
                }
            }
            inject(children, headings, index);
        }
    }
}

fn emit(emitter: &mut impl Emitter, elements: &[Element], depth: usize) -> miette::Result<()> {
    for element in elements {
        dispatch(emitter, element, depth)?;
    }
    Ok(())
}

fn dispatch(emitter: &mut impl Emitter, element: &Element, depth: usize) -> miette::Result<()> {
    match element {
        Element::Tag {
            name,
            attributes,
            children,
        } => tag(emitter, name, attributes, children, depth),
        Element::Void { name, attributes } => {
            emitter.indent(depth)?;
            emitter.void(name, attributes)?;
            emitter.newline()
        }
        Element::Text(content) => emitter.text(content),
        Element::Raw(content) => emitter.raw(content),
        Element::Code {
            content,
            language,
            location,
        } => {
            emitter.indent(depth)?;
            emitter.code(content, *language, location.as_ref())?;
            emitter.newline()
        }
    }
}

fn tag(
    emitter: &mut impl Emitter,
    name: &str,
    attributes: &[(String, String)],
    children: &[Element],
    depth: usize,
) -> miette::Result<()> {
    let nested = children
        .iter()
        .any(|c| matches!(c, Element::Tag { .. } | Element::Code { .. }));

    emitter.indent(depth)?;
    emitter.open(name, attributes)?;

    if nested && name != "pre" {
        emitter.newline()?;
        emit(emitter, children, depth + 1)?;
        emitter.indent(depth)?;
    } else {
        inline(emitter, children)?;
    }

    emitter.close(name)?;
    emitter.newline()
}

fn inline(emitter: &mut impl Emitter, elements: &[Element]) -> miette::Result<()> {
    for element in elements {
        match element {
            Element::Tag {
                name,
                attributes,
                children,
            } => {
                emitter.open(name, attributes)?;
                inline(emitter, children)?;
                emitter.close(name)?;
            }
            Element::Void { name, attributes } => emitter.void(name, attributes)?,
            Element::Text(content) => emitter.text(content)?,
            Element::Raw(content) => emitter.raw(content)?,
            Element::Code {
                content,
                language,
                location,
            } => emitter.code(content, *language, location.as_ref())?,
        }
    }
    Ok(())
}

fn level(name: &str) -> Option<u8> {
    match name {
        "h1" => Some(1),
        "h2" => Some(2),
        "h3" => Some(3),
        "h4" => Some(4),
        "h5" => Some(5),
        "h6" => Some(6),
        _ => None,
    }
}

#[must_use]
pub fn textual(elements: &[Element]) -> String {
    let mut text = String::new();
    for element in elements {
        match element {
            Element::Text(t) => text.push_str(t),
            Element::Tag { children, .. } => text.push_str(&textual(children)),
            _ => {}
        }
    }
    text
}

#[must_use]
pub fn identify<S: std::hash::BuildHasher>(
    text: &str,
    explicit: Option<&str>,
    identifiers: &mut HashSet<String, S>,
) -> String {
    let id = match explicit {
        Some(id) => id.to_string(),
        None => deduplicate(slugify(text), identifiers),
    };
    identifiers.insert(id.clone());
    id
}

#[must_use]
pub fn slugify(text: &str) -> String {
    let mut output = text
        .chars()
        .fold(String::with_capacity(text.len()), |mut output, c| {
            if c.is_alphanumeric() {
                output.extend(c.to_lowercase());
            } else if !output.ends_with('-') && !output.is_empty() {
                output.push('-');
            }
            output
        });
    while output.ends_with('-') {
        output.pop();
    }
    output
}

#[must_use]
pub fn deduplicate<S: std::hash::BuildHasher>(
    base: String,
    identifiers: &HashSet<String, S>,
) -> String {
    if !identifiers.contains(&base) {
        return base;
    }
    let mut suffix = 2;
    loop {
        let candidate = format!("{base}-{suffix}");
        if !identifiers.contains(&candidate) {
            return candidate;
        }
        suffix += 1;
    }
}
