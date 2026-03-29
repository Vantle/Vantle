use element::Element;
use viewport::Viewport;

pub struct Entry {
    pub point: f64,
    pub mean: f64,
    pub deviation: f64,
    pub interval: [f64; 2],
    pub count: usize,
    pub predicted: f64,
}

#[must_use]
pub fn render(viewport: &Viewport, samples: &[Entry]) -> Vec<Element> {
    let class = class::chart::point().to_string();
    let emerge = class::chart::emerge().to_string();
    let peak = samples
        .iter()
        .map(|s| s.deviation)
        .fold(0.0_f64, f64::max)
        .max(f64::EPSILON);
    samples
        .iter()
        .enumerate()
        .map(|(index, sample)| {
            let projected = viewport.project(sample.point, sample.mean);
            #[expect(clippy::cast_precision_loss)]
            let radius = (3.0 + (sample.count as f64).ln()).min(8.0);
            let normalized = (sample.deviation / peak).clamp(0.0, 1.0);
            let opacity = 0.55 + 0.4 * (1.0 - normalized);
            #[expect(clippy::cast_precision_loss)]
            let delay = 0.6 + index as f64 * 0.04;
            let children = vec![Element::leaf(
                "circle",
                vec![
                    ("cx".into(), format!("{:.1}", projected.x)),
                    ("cy".into(), format!("{:.1}", projected.y)),
                    ("r".into(), format!("{radius:.1}")),
                    ("class".into(), format!("visible {emerge}")),
                    ("opacity".into(), format!("{opacity:.2}")),
                    ("data-index".into(), format!("{index}")),
                    (
                        "style".into(),
                        format!(
                            "transform-origin: {:.1}px {:.1}px; animation-delay: {delay:.2}s",
                            projected.x, projected.y
                        ),
                    ),
                ],
            )];
            Element::Tag {
                name: "g".into(),
                attributes: vec![("class".into(), class.clone())],
                children,
            }
        })
        .collect::<Vec<_>>()
}
