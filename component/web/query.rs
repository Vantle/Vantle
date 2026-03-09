use element::Element;

#[must_use]
pub fn id<'a>(elements: &'a [Element], target: &str) -> Option<&'a Element> {
    for element in elements {
        if matched(element, target) {
            return Some(element);
        }
        if let Element::Tag { children, .. } = element
            && let Some(found) = id(children, target)
        {
            return Some(found);
        }
    }
    None
}

pub fn id_mut<'a>(elements: &'a mut [Element], target: &str) -> Option<&'a mut Element> {
    for element in elements.iter_mut() {
        if matched(element, target) {
            return Some(element);
        }
        if let Element::Tag { children, .. } = element
            && let Some(found) = id_mut(children, target)
        {
            return Some(found);
        }
    }
    None
}

#[must_use]
pub fn class<'a>(elements: &'a [Element], name: &str) -> Vec<&'a Element> {
    let mut results = Vec::new();
    collect(elements, name, &mut results);
    results
}

pub fn class_mut<'a>(elements: &'a mut [Element], name: &str) -> Vec<&'a mut Element> {
    let mut results = Vec::new();
    collect_mut(elements, name, &mut results);
    results
}

#[must_use]
pub fn attribute<'a>(elements: &'a [Element], key: &str, value: &str) -> Vec<&'a Element> {
    let mut results = Vec::new();
    filter(elements, key, value, &mut results);
    results
}

pub fn attribute_mut<'a>(
    elements: &'a mut [Element],
    key: &str,
    value: &str,
) -> Vec<&'a mut Element> {
    let mut results = Vec::new();
    filter_mut(elements, key, value, &mut results);
    results
}

pub fn visit(elements: &[Element], f: &mut impl FnMut(&Element)) {
    for element in elements {
        f(element);
        if let Element::Tag { children, .. } = element {
            visit(children, f);
        }
    }
}

pub fn visit_mut(elements: &mut [Element], f: &mut impl FnMut(&mut Element)) {
    for element in elements.iter_mut() {
        f(element);
        if let Element::Tag { children, .. } = element {
            visit_mut(children, f);
        }
    }
}

fn matched(element: &Element, target: &str) -> bool {
    matches!(
        element,
        Element::Tag { attributes, .. }
            if attributes.iter().any(|(k, v)| k == "id" && v == target)
    )
}

fn classed(element: &Element, name: &str) -> bool {
    matches!(
        element,
        Element::Tag { attributes, .. }
            if attributes
                .iter()
                .any(|(k, v)| k == "class" && v.split_whitespace().any(|c| c == name))
    )
}

fn tagged(element: &Element, key: &str, value: &str) -> bool {
    matches!(
        element,
        Element::Tag { attributes, .. }
            if attributes.iter().any(|(k, v)| k == key && v == value)
    )
}

fn collect<'a>(elements: &'a [Element], name: &str, results: &mut Vec<&'a Element>) {
    for element in elements {
        if classed(element, name) {
            results.push(element);
        } else if let Element::Tag { children, .. } = element {
            collect(children, name, results);
        }
    }
}

fn collect_mut<'a>(elements: &'a mut [Element], name: &str, results: &mut Vec<&'a mut Element>) {
    for element in elements.iter_mut() {
        if classed(element, name) {
            results.push(element);
        } else if let Element::Tag { children, .. } = element {
            collect_mut(children, name, results);
        }
    }
}

fn filter<'a>(elements: &'a [Element], key: &str, value: &str, results: &mut Vec<&'a Element>) {
    for element in elements {
        if tagged(element, key, value) {
            results.push(element);
        } else if let Element::Tag { children, .. } = element {
            filter(children, key, value, results);
        }
    }
}

fn filter_mut<'a>(
    elements: &'a mut [Element],
    key: &str,
    value: &str,
    results: &mut Vec<&'a mut Element>,
) {
    for element in elements.iter_mut() {
        if tagged(element, key, value) {
            results.push(element);
        } else if let Element::Tag { children, .. } = element {
            filter_mut(children, key, value, results);
        }
    }
}
