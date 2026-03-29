use element::Element;
use viewport::Viewport;

#[must_use]
pub fn render(viewport: &Viewport, label: &str, vertical: &str) -> Vec<Element> {
    let domain = ticks(viewport.domain.minimum, viewport.domain.maximum);
    let range = ticks(viewport.range.minimum, viewport.range.maximum);
    let mut children = grid(viewport, &domain, &range);
    children.extend(horizontal(viewport, label, &domain));
    children.extend(rotated(viewport, vertical, &range));
    vec![Element::Tag {
        name: "g".into(),
        attributes: vec![("class".into(), class::chart::axis().to_string())],
        children,
    }]
}

fn horizontal(viewport: &Viewport, label: &str, ticks: &[f64]) -> Vec<Element> {
    let area = viewport.plot();
    let bottom = viewport.margin.top + area.height;
    let offset = bottom + proportion::scale(6);
    let mut elements = Vec::new();
    for value in ticks {
        let sx = viewport.project(*value, viewport.range.minimum).x;
        let formatted = format::notation(*value);
        elements.push(Element::leaf(
            "line",
            vec![
                ("x1".into(), format!("{sx}")),
                ("y1".into(), format!("{bottom}")),
                ("x2".into(), format!("{sx}")),
                ("y2".into(), format!("{}", bottom + proportion::scale(3))),
                ("stroke".into(), "var(--text-secondary)".into()),
                ("stroke-width".into(), "0.5".into()),
                ("opacity".into(), "0.3".into()),
            ],
        ));
        elements.push(Element::Tag {
            name: "text".into(),
            attributes: vec![
                ("x".into(), format!("{sx}")),
                ("y".into(), format!("{offset}")),
                ("text-anchor".into(), "middle".into()),
                ("dominant-baseline".into(), "central".into()),
                ("class".into(), class::chart::tick().to_string()),
            ],
            children: vec![Element::text(&formatted)],
        });
    }
    let anchor = bottom + proportion::scale(7) + proportion::scale(4);
    let center = viewport.margin.left + area.width / 2.0;
    elements.push(Element::Tag {
        name: "text".into(),
        attributes: vec![
            ("x".into(), format!("{center}")),
            ("y".into(), format!("{anchor}")),
            ("text-anchor".into(), "middle".into()),
            ("dominant-baseline".into(), "central".into()),
            ("class".into(), class::chart::label().to_string()),
        ],
        children: vec![Element::text(label)],
    });
    elements
}

fn rotated(viewport: &Viewport, label: &str, ticks: &[f64]) -> Vec<Element> {
    let area = viewport.plot();
    let left = viewport.margin.left;
    let mut elements = Vec::new();
    for value in ticks {
        let sy = viewport.project(viewport.domain.minimum, *value).y;
        let formatted = format::notation(*value);
        elements.push(Element::leaf(
            "line",
            vec![
                ("x1".into(), format!("{}", left - proportion::scale(3))),
                ("y1".into(), format!("{sy}")),
                ("x2".into(), format!("{left}")),
                ("y2".into(), format!("{sy}")),
                ("stroke".into(), "var(--text-secondary)".into()),
                ("stroke-width".into(), "0.5".into()),
                ("opacity".into(), "0.3".into()),
            ],
        ));
        elements.push(Element::Tag {
            name: "text".into(),
            attributes: vec![
                ("x".into(), format!("{}", left - proportion::scale(5))),
                ("y".into(), format!("{sy}")),
                ("text-anchor".into(), "end".into()),
                ("dominant-baseline".into(), "central".into()),
                ("class".into(), class::chart::tick().to_string()),
            ],
            children: vec![Element::text(&formatted)],
        });
    }
    let center = viewport.margin.top + area.height / 2.0;
    let anchor = proportion::scale(5);
    elements.push(Element::Tag {
        name: "text".into(),
        attributes: vec![
            ("x".into(), format!("{anchor}")),
            ("y".into(), format!("{center}")),
            ("text-anchor".into(), "middle".into()),
            ("dominant-baseline".into(), "central".into()),
            (
                "transform".into(),
                format!("rotate(-90, {anchor}, {center})"),
            ),
            ("class".into(), class::chart::label().to_string()),
        ],
        children: vec![Element::text(label)],
    });
    elements
}

fn grid(viewport: &Viewport, domain: &[f64], range: &[f64]) -> Vec<Element> {
    let area = viewport.plot();
    let left = viewport.margin.left;
    let right = viewport.margin.left + area.width;
    let top = viewport.margin.top;
    let bottom = viewport.margin.top + area.height;
    let mut elements = Vec::new();
    for value in domain {
        let sx = viewport.project(*value, viewport.range.minimum).x;
        elements.push(Element::leaf(
            "line",
            vec![
                ("x1".into(), format!("{sx}")),
                ("y1".into(), format!("{top}")),
                ("x2".into(), format!("{sx}")),
                ("y2".into(), format!("{bottom}")),
                ("class".into(), class::chart::grid().to_string()),
                ("opacity".into(), "0.08".into()),
            ],
        ));
    }
    for value in range {
        let sy = viewport.project(viewport.domain.minimum, *value).y;
        elements.push(Element::leaf(
            "line",
            vec![
                ("x1".into(), format!("{left}")),
                ("y1".into(), format!("{sy}")),
                ("x2".into(), format!("{right}")),
                ("y2".into(), format!("{sy}")),
                ("class".into(), class::chart::grid().to_string()),
                ("opacity".into(), "0.10".into()),
            ],
        ));
    }
    elements
}

fn ticks(minimum: f64, maximum: f64) -> Vec<f64> {
    let span = maximum - minimum;
    if span <= 0.0 {
        return vec![minimum];
    }
    let target = 6;
    let rough = span / f64::from(target);
    let magnitude = 10.0_f64.powf(rough.log10().floor());
    let residual = rough / magnitude;
    let step = if residual <= 1.5 {
        magnitude
    } else if residual <= 3.5 {
        2.0 * magnitude
    } else if residual <= 7.5 {
        5.0 * magnitude
    } else {
        10.0 * magnitude
    };
    let start = (minimum / step).ceil() * step;
    let mut result = Vec::new();
    let mut value = start;
    while value <= maximum + step * 0.001 {
        result.push(value);
        value += step;
    }
    result
}
