use element::Element;
use regression::Candidate;
use scatter::Entry;
use viewport::Viewport;

const SLICES: u32 = 80;
const STOPS: u32 = 24;
const PEAK: f64 = 0.28;
const HUE_START: f64 = 220.0;
const HUE_END: f64 = 350.0;

#[must_use]
pub fn heatmap(
    viewport: &Viewport,
    candidate: Option<&Candidate>,
    normalization: f64,
    samples: &[Entry],
) -> Vec<Element> {
    if samples.len() < 2 {
        return Vec::new();
    }
    let area = viewport.plot();
    let width = area.width / f64::from(SLICES);
    let aspect = viewport.aspect();
    let mut definitions = Vec::new();
    let mut strips = Vec::new();
    for column in 0..SLICES {
        let fraction = (f64::from(column) + 0.5) / f64::from(SLICES);
        let x = viewport.domain.minimum + viewport.domain.span() * fraction;
        let center = candidate.map_or_else(
            || interpolated(samples, x),
            |c| c.evaluate(x / normalization),
        );
        let base = deviation(samples, x)
            .max(center.abs() * 0.05)
            .max(viewport.range.span() * 0.01);
        let visual =
            candidate.map_or(0.0, |c| c.derivative(x / normalization) / normalization) * aspect;
        let sigma = base * (1.0 + visual * visual).sqrt();
        let hue = HUE_START + (HUE_END - HUE_START) * fraction;
        let color = format!("hsl({hue:.0} 70% 55%)");
        let identifier = format!("h{column}");
        let stops = (0..=STOPS)
            .map(|row| {
                let t = f64::from(row) / f64::from(STOPS);
                let y = viewport.range.maximum - viewport.range.span() * t;
                let z = (y - center) / sigma;
                let opacity = PEAK * (-z * z / 2.0).exp();
                Element::leaf(
                    "stop",
                    vec![
                        ("offset".into(), format!("{:.1}%", t * 100.0)),
                        ("stop-color".into(), color.clone()),
                        ("stop-opacity".into(), format!("{opacity:.3}")),
                    ],
                )
            })
            .collect::<Vec<_>>();
        definitions.push(Element::Tag {
            name: "linearGradient".into(),
            attributes: vec![
                ("id".into(), identifier.clone()),
                ("x1".into(), "0".into()),
                ("y1".into(), "0".into()),
                ("x2".into(), "0".into()),
                ("y2".into(), "1".into()),
            ],
            children: stops,
        });
        let sx = viewport.margin.left + f64::from(column) * width;
        strips.push(Element::leaf(
            "rect",
            vec![
                ("x".into(), format!("{sx:.1}")),
                ("y".into(), format!("{:.1}", viewport.margin.top)),
                ("width".into(), format!("{:.1}", width + 0.5)),
                ("height".into(), format!("{:.1}", area.height)),
                ("fill".into(), format!("url(#{identifier})")),
            ],
        ));
    }
    vec![
        Element::Tag {
            name: "defs".into(),
            attributes: Vec::new(),
            children: definitions,
        },
        Element::Tag {
            name: "g".into(),
            attributes: vec![
                ("class".into(), class::chart::gradient().to_string()),
                ("clip-path".into(), "url(#plot)".into()),
                ("filter".into(), "url(#smooth)".into()),
            ],
            children: strips,
        },
    ]
}

fn interpolated(samples: &[Entry], x: f64) -> f64 {
    estimate::interpolate(samples, x, |s| s.point, |s| s.predicted)
}

fn deviation(samples: &[Entry], x: f64) -> f64 {
    estimate::interpolate(samples, x, |s| s.point, |s| s.deviation)
}
