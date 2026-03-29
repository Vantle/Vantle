use body::Body;
use element::Element;
use proto::chart as wire;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Report {
    pub functions: Vec<Function>,
}

#[derive(Deserialize)]
pub struct Function {
    pub name: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub dimension: Vec<String>,
    pub observation: Observation,
}

#[derive(Deserialize)]
pub struct Observation {
    pub time: Timed,
}

#[derive(Deserialize)]
pub struct Timed {
    pub unit: String,
    pub sample: Vec<analysis::Observation>,
}

#[must_use]
pub fn groups(report: Report) -> Vec<card::Group> {
    report
        .functions
        .into_iter()
        .map(|function| {
            let html = visualization(&function);
            let total = function.observation.time.sample.len();
            card::Group {
                function: function.name,
                tags: function.tags,
                source: None,
                input: String::new(),
                output: String::new(),
                cases: vec![card::Case {
                    parameters: serde_json::json!({ "chart": html, "samples": total }),
                    returns: serde_json::json!({ "status": "measured" }),
                    unexpected: None,
                }],
            }
        })
        .collect::<Vec<_>>()
}

#[must_use]
pub fn render(body: Body, group: &card::Group) -> Body {
    let html = group
        .cases
        .first()
        .and_then(|c| c.parameters.get("chart"))
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    body.html(html)
}

fn visualization(function: &Function) -> String {
    let timed = &function.observation.time;
    let dimensions = function.dimension.len().max(1);

    let result = fitting::perform(&timed.sample, dimensions);

    let normalization = result.fit.as_ref().map_or(1.0, |f| f.normalization);

    let fitted = result.fit.as_ref().map(|f| regression::Candidate {
        terms: f
            .terms
            .iter()
            .map(|t| regression::Term {
                exponent: t.exponent.clone(),
                coefficient: t.coefficient,
            })
            .collect::<Vec<_>>(),
    });

    let mut entries = result
        .samples
        .iter()
        .map(|s| {
            let x = s.point.first().copied().unwrap_or(0.0);
            scatter::Entry {
                point: x,
                mean: s.mean,
                deviation: s.deviation,
                interval: s.interval,
                count: s.count,
                predicted: s.predicted,
            }
        })
        .collect::<Vec<_>>();
    entries.sort_by(|a, b| a.point.total_cmp(&b.point));

    if entries.is_empty() {
        return String::new();
    }

    let (abscissa, ordinate) = entries.iter().fold(
        ((f64::MAX, f64::MIN), (f64::MAX, f64::MIN)),
        |((xlo, xhi), (ylo, yhi)), e| {
            (
                (xlo.min(e.point), xhi.max(e.point)),
                (ylo.min(e.mean), yhi.max(e.mean)),
            )
        },
    );

    let domain = viewport::Extent::padded(abscissa.0, abscissa.1, 0.05);
    let ordinate = expand(ordinate, &domain, fitted.as_ref(), normalization);
    let range = viewport::Extent::padded(ordinate.0.max(0.0), ordinate.1, 0.10);
    let vp = viewport::Viewport::new(domain, range);

    let horizontal = function.dimension.first().map_or("input", String::as_str);

    let mut content = gradient::heatmap(&vp, fitted.as_ref(), normalization, &entries);
    let vertical = format!("time ({})", timed.unit);
    content.extend(axis::render(&vp, horizontal, &vertical));
    content.extend(regression::render(
        &vp,
        fitted.as_ref(),
        normalization,
        &entries,
    ));
    content.extend(scatter::render(&vp, &entries));

    let mut svg = canvas::render(&vp, content);
    if let Element::Tag {
        ref mut attributes, ..
    } = svg
    {
        attributes.push((
            attribute::chart().name().into(),
            encode(&vp, normalization, fitted.as_ref(), &entries, horizontal),
        ));
    }

    let mut children = vec![svg];
    children.extend(breakdown(&entries, horizontal, &timed.unit));

    let root = Element::Tag {
        name: "div".into(),
        attributes: vec![("class".into(), class::chart::chart().to_string())],
        children,
    };

    fragment::fragment(&[root]).unwrap_or_default()
}

fn expand(
    ordinate: (f64, f64),
    domain: &viewport::Extent,
    fitted: Option<&regression::Candidate>,
    normalization: f64,
) -> (f64, f64) {
    let Some(candidate) = fitted else {
        return ordinate;
    };
    let steps = 200;
    let span = domain.span();
    (0..=steps).fold(ordinate, |(lo, hi), i| {
        let x = domain.minimum + span * f64::from(i) / f64::from(steps);
        let y = candidate.evaluate(x / normalization);
        (lo.min(y), hi.max(y))
    })
}

fn encode(
    vp: &viewport::Viewport,
    normalization: f64,
    fitted: Option<&regression::Candidate>,
    entries: &[scatter::Entry],
    label: &str,
) -> String {
    use base64::Engine;
    use prost::Message;
    let message = wire::Chart {
        viewport: Some(wire::Viewport {
            width: vp.width,
            height: vp.height,
            margin: Some(wire::Margin {
                top: vp.margin.top,
                right: vp.margin.right,
                bottom: vp.margin.bottom,
                left: vp.margin.left,
            }),
            domain: Some(wire::Extent {
                minimum: vp.domain.minimum,
                maximum: vp.domain.maximum,
            }),
            range: Some(wire::Extent {
                minimum: vp.range.minimum,
                maximum: vp.range.maximum,
            }),
        }),
        normalization,
        label: label.to_string(),
        terms: fitted.map_or_else(Vec::new, |c| {
            c.terms
                .iter()
                .map(|t| wire::Term {
                    #[expect(clippy::cast_possible_truncation)]
                    exponent: t.exponent.first().copied().unwrap_or(0) as u32,
                    coefficient: t.coefficient,
                })
                .collect::<Vec<_>>()
        }),
        samples: entries
            .iter()
            .map(|e| wire::Sample {
                point: e.point,
                mean: e.mean,
                deviation: e.deviation,
                predicted: e.predicted,
                lower: e.interval[0],
                upper: e.interval[1],
                #[expect(clippy::cast_possible_truncation)]
                count: e.count as u32,
            })
            .collect::<Vec<_>>(),
    };
    base64::engine::general_purpose::STANDARD.encode(message.encode_to_vec())
}

fn cell(tag: &'static str, content: &str) -> Element {
    Element::Tag {
        name: tag.into(),
        attributes: Vec::new(),
        children: vec![Element::text(content)],
    }
}

fn breakdown(entries: &[scatter::Entry], dimension: &str, unit: &str) -> Vec<Element> {
    let header = Element::Tag {
        name: "tr".into(),
        attributes: Vec::new(),
        children: vec![
            cell("th", dimension),
            cell("th", "mean"),
            cell("th", &format!("\u{00b1} dev ({unit})")),
            cell("th", "samples"),
        ],
    };

    let rows = entries
        .iter()
        .map(|entry| {
            let mean = format::duration(entry.mean);
            let deviation = format::duration(entry.deviation);
            Element::Tag {
                name: "tr".into(),
                attributes: Vec::new(),
                children: vec![
                    cell("td", &format!("{:.0}", entry.point)),
                    cell("td", &mean.to_string()),
                    cell("td", &deviation.to_string()),
                    cell("td", &entry.count.to_string()),
                ],
            }
        })
        .collect::<Vec<_>>();

    vec![Element::Tag {
        name: "div".into(),
        attributes: Vec::new(),
        children: vec![Element::Tag {
            name: "table".into(),
            attributes: Vec::new(),
            children: vec![
                Element::Tag {
                    name: "thead".into(),
                    attributes: Vec::new(),
                    children: vec![header],
                },
                Element::Tag {
                    name: "tbody".into(),
                    attributes: Vec::new(),
                    children: rows,
                },
            ],
        }],
    }]
}
