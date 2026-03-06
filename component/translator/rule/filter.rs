#[must_use]
pub fn none<Element>() -> Box<dyn Fn(Element) -> bool + 'static> {
    Box::new(|_: Element| true)
}
