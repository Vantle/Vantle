use element::Element;
use viewport::Viewport;

#[must_use]
pub fn render(viewport: &Viewport, content: Vec<Element>) -> Element {
    let area = viewport.plot();
    let mut children = vec![Element::Tag {
        name: "defs".into(),
        attributes: Vec::new(),
        children: vec![
            Element::Tag {
                name: "clipPath".into(),
                attributes: vec![("id".into(), "plot".into())],
                children: vec![Element::leaf(
                    "rect",
                    vec![
                        ("x".into(), format!("{}", viewport.margin.left)),
                        ("y".into(), format!("{}", viewport.margin.top)),
                        ("width".into(), format!("{}", area.width)),
                        ("height".into(), format!("{}", area.height)),
                    ],
                )],
            },
            Element::Tag {
                name: "filter".into(),
                attributes: vec![("id".into(), "smooth".into())],
                children: vec![Element::leaf(
                    "feGaussianBlur",
                    vec![("stdDeviation".into(), "8 3".into())],
                )],
            },
            shadow(),
        ],
    }];
    children.extend(content);
    Element::Tag {
        name: "svg".into(),
        attributes: vec![
            (
                "viewBox".into(),
                format!("0 0 {} {}", viewport.width, viewport.height),
            ),
            ("xmlns".into(), "http://www.w3.org/2000/svg".into()),
            ("preserveAspectRatio".into(), "xMidYMid meet".into()),
        ],
        children,
    }
}

fn shadow() -> Element {
    Element::Tag {
        name: "filter".into(),
        attributes: vec![
            ("id".into(), "shadow".into()),
            ("x".into(), "-20%".into()),
            ("y".into(), "-20%".into()),
            ("width".into(), "140%".into()),
            ("height".into(), "140%".into()),
        ],
        children: vec![
            Element::leaf(
                "feGaussianBlur",
                vec![
                    ("in".into(), "SourceAlpha".into()),
                    ("stdDeviation".into(), "3".into()),
                    ("result".into(), "blur".into()),
                ],
            ),
            Element::leaf(
                "feOffset",
                vec![
                    ("in".into(), "blur".into()),
                    ("dx".into(), "0".into()),
                    ("dy".into(), "2".into()),
                    ("result".into(), "shifted".into()),
                ],
            ),
            Element::leaf(
                "feFlood",
                vec![
                    ("flood-color".into(), "black".into()),
                    ("flood-opacity".into(), "0.12".into()),
                    ("result".into(), "color".into()),
                ],
            ),
            Element::Tag {
                name: "feComposite".into(),
                attributes: vec![
                    ("in".into(), "color".into()),
                    ("in2".into(), "shifted".into()),
                    ("operator".into(), "in".into()),
                    ("result".into(), "shadow".into()),
                ],
                children: Vec::new(),
            },
            Element::Tag {
                name: "feMerge".into(),
                attributes: Vec::new(),
                children: vec![
                    Element::leaf("feMergeNode", vec![("in".into(), "shadow".into())]),
                    Element::leaf("feMergeNode", vec![("in".into(), "SourceGraphic".into())]),
                ],
            },
        ],
    }
}
