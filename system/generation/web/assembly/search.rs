use wasm_bindgen::prelude::*;
use web_sys::Document;

enum Symbol {
    Term(String),
    Not,
    And,
    Or,
    Open,
    Close,
}

fn tokenize(input: &str) -> Vec<Symbol> {
    let mut symbols = Vec::new();
    let mut iterator = input.chars().peekable();
    while let Some(&character) = iterator.peek() {
        match character {
            ' ' | '\t' => {
                iterator.next();
            }
            '!' | '.' | ',' | '(' | ')' => {
                symbols.push(match character {
                    '!' => Symbol::Not,
                    '.' => Symbol::And,
                    ',' => Symbol::Or,
                    '(' => Symbol::Open,
                    ')' => Symbol::Close,
                    _ => unreachable!(),
                });
                iterator.next();
            }
            _ => {
                let mut term = String::new();
                while let Some(&c) = iterator.peek() {
                    if "!.,() \t".contains(c) {
                        break;
                    }
                    term.push(c);
                    iterator.next();
                }
                symbols.push(Symbol::Term(term));
            }
        }
    }
    symbols
}

fn disjunction(tags: &[&str], symbols: &[Symbol], position: usize) -> (bool, usize) {
    let (mut result, mut position) = conjunction(tags, symbols, position);
    while matches!(symbols.get(position), Some(Symbol::Or)) {
        let (right, next) = conjunction(tags, symbols, position + 1);
        result = result || right;
        position = next;
    }
    (result, position)
}

fn conjunction(tags: &[&str], symbols: &[Symbol], position: usize) -> (bool, usize) {
    let (mut result, mut position) = negation(tags, symbols, position);
    while matches!(symbols.get(position), Some(Symbol::And)) {
        let (right, next) = negation(tags, symbols, position + 1);
        result = result && right;
        position = next;
    }
    (result, position)
}

fn negation(tags: &[&str], symbols: &[Symbol], position: usize) -> (bool, usize) {
    if matches!(symbols.get(position), Some(Symbol::Not)) {
        let (result, next) = negation(tags, symbols, position + 1);
        return (!result, next);
    }
    atom(tags, symbols, position)
}

fn atom(tags: &[&str], symbols: &[Symbol], position: usize) -> (bool, usize) {
    if position >= symbols.len() {
        return (true, position);
    }
    match &symbols[position] {
        Symbol::Open => {
            let (result, next) = disjunction(tags, symbols, position + 1);
            let end = if matches!(symbols.get(next), Some(Symbol::Close)) {
                next + 1
            } else {
                next
            };
            (result, end)
        }
        Symbol::Term(term) => {
            let found = tags.iter().any(|tag| tag.contains(term.as_str()));
            (found, position + 1)
        }
        _ => (true, position + 1),
    }
}

fn evaluate(tags: &[&str], query: &str) -> bool {
    let query = query.trim();
    if query.is_empty() {
        return true;
    }
    let symbols = tokenize(query);
    let (result, _) = disjunction(tags, &symbols, 0);
    result
}

pub fn initialize(document: &Document) {
    let search = attribute::search().selector();
    let Ok(Some(input)) = document.query_selector(&search) else {
        return;
    };

    let functions = attribute::function().selector();
    let counter = attribute::counter().selector();
    let empty = attribute::empty().selector();

    let filtered = document.clone();
    let callback = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
        let Some(target) = event.target() else {
            return;
        };

        let query = js_sys::Reflect::get(&target, &JsValue::from_str("value"))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default()
            .to_lowercase();

        let Ok(cards) = filtered.query_selector_all(&functions) else {
            return;
        };

        let total = cards.length();
        let mut visible: u32 = 0;

        for index in 0..total {
            let Some(node) = cards.get(index) else {
                continue;
            };
            let element: &web_sys::Element = node.unchecked_ref();

            if query.is_empty() {
                let _ = element.remove_attribute(attribute::hidden().name());
                visible += 1;
                continue;
            }

            let raw = element
                .get_attribute(attribute::tags().name())
                .unwrap_or_default();
            let tags = raw
                .split_whitespace()
                .map(str::to_lowercase)
                .collect::<Vec<_>>();
            let refs = tags.iter().map(String::as_str).collect::<Vec<_>>();

            if evaluate(&refs, &query) {
                let _ = element.remove_attribute(attribute::hidden().name());
                visible += 1;
            } else {
                let _ = element.set_attribute(attribute::hidden().name(), "");
            }
        }

        if let Ok(Some(count)) = filtered.query_selector(&counter) {
            if query.is_empty() {
                count.set_text_content(None);
            } else {
                count.set_text_content(Some(&format!("{visible} / {total}")));
            }
        }

        if let Ok(Some(empty)) = filtered.query_selector(&empty) {
            if visible == 0 && !query.is_empty() {
                let _ = empty.remove_attribute(attribute::hidden().name());
            } else {
                let _ = empty.set_attribute(attribute::hidden().name(), "");
            }
        }
    });

    let _ = input.add_event_listener_with_callback("input", callback.as_ref().unchecked_ref());
    callback.forget();

    let bound = document
        .document_element()
        .is_some_and(|html| html.get_attribute(attribute::keyboard().name()).is_some());

    if !bound {
        if let Some(html) = document.document_element() {
            let _ = html.set_attribute(attribute::keyboard().name(), "");
        }

        let keyed = document.clone();
        let keyboard = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
            move |event: web_sys::KeyboardEvent| {
                if event.key() == "/" {
                    if editing::active(&event) {
                        return;
                    }
                    event.prevent_default();
                    if let Ok(Some(search)) = keyed.query_selector(&search) {
                        let html: &web_sys::HtmlElement = search.unchecked_ref();
                        let _ = html.focus();
                    }
                }

                if event.key() == "Escape"
                    && let Ok(Some(search)) = keyed.query_selector(&search)
                {
                    let input: &web_sys::HtmlInputElement = search.unchecked_ref();
                    if !input.value().is_empty() {
                        input.set_value("");
                        if let Ok(event) = web_sys::Event::new("input") {
                            let _ = input.dispatch_event(&event);
                        }
                    }
                }
            },
        );

        let _ =
            document.add_event_listener_with_callback("keydown", keyboard.as_ref().unchecked_ref());
        keyboard.forget();
    }
}
