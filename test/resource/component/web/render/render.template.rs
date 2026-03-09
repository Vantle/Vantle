use render::slugify;

fn slug(text: String) -> String {
    slugify(&text)
}

fn textual(elements: Vec<String>) -> String {
    use element::Element;
    let converted = elements.into_iter().map(Element::Text).collect::<Vec<_>>();
    render::textual(&converted)
}
