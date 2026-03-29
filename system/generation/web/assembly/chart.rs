use std::cell::{Cell, RefCell};
use std::fmt::Write;
use std::rc::Rc;

use proto::chart as wire;
use viewport::Viewport;
use wasm_bindgen::prelude::*;
use web_sys::Document;

const NAMESPACE: &str = "http://www.w3.org/2000/svg";
const RESOLUTION: u32 = 200;
const CARD_PADDING: f64 = 8.0;
const CARD_GAP: f64 = 10.0;
const CARD_HEIGHT: f64 = 55.0;
const CARD_EXPANDED: f64 = 77.0;
const ROW_ORIGIN: f64 = 14.0;
const TICK_LENGTH: f64 = 8.0;
const DURATION: f64 = 618.0;
const SLICES: u32 = 80;
const STOPS: u32 = 24;
const PEAK: f64 = 0.28;

#[derive(Clone)]
struct Term {
    exponent: u32,
    coefficient: f64,
}

struct Sample {
    point: f64,
    mean: f64,
    deviation: f64,
    predicted: f64,
    count: u32,
}

struct Nearest {
    domain: f64,
    predicted: f64,
}

struct Overlay {
    group: web_sys::Element,
    indicator: web_sys::Element,
    tangent: web_sys::Element,
    band: web_sys::Element,
    marker: web_sys::Element,
    pins: web_sys::Element,
    frozen: web_sys::Element,
    vector: web_sys::Element,
    perpendicular: web_sys::Element,
    parallel: web_sys::Element,
    cursor: web_sys::Element,
    background: web_sys::Element,
    density: web_sys::Element,
    position: web_sys::Element,
    mean: web_sys::Element,
    slope: web_sys::Element,
    label: web_sys::Element,
    count: web_sys::Element,
    merged: web_sys::Element,
}

struct State {
    document: Document,
    svg: web_sys::Element,
    viewport: Viewport,
    normalization: f64,
    label: String,
    terms: Vec<Term>,
    samples: Vec<Sample>,
    overlay: Overlay,
    anchor: Cell<f64>,
    locked: Cell<bool>,
    cursor: Cell<(f64, f64)>,
    excluded: RefCell<Vec<bool>>,
    active: RefCell<Vec<Term>>,
    generation: Cell<u32>,
    stops: Vec<Vec<web_sys::Element>>,
    line: Option<web_sys::Element>,
}

impl State {
    fn predicted(&self, x: f64) -> f64 {
        {
            let active = self.active.borrow();
            if !active.is_empty() {
                let normalized = x / self.normalization;
                return active
                    .iter()
                    .map(|t| t.coefficient * normalized.powi(t.exponent.cast_signed()))
                    .sum();
            }
        }
        let excluded = self.excluded.borrow();
        let filtered = self
            .samples
            .iter()
            .enumerate()
            .filter(|(i, _)| !excluded[*i])
            .map(|(_, s)| s)
            .collect::<Vec<_>>();
        if filtered.len() < 2 {
            return estimate::interpolate(&self.samples, x, |s| s.point, |s| s.predicted);
        }
        estimate::interpolate(&filtered, x, |s| s.point, |s| s.predicted)
    }

    fn deviation(&self, x: f64) -> f64 {
        let excluded = self.excluded.borrow();
        let filtered = self
            .samples
            .iter()
            .enumerate()
            .filter(|(i, _)| !excluded[*i])
            .map(|(_, s)| s)
            .collect::<Vec<_>>();
        if filtered.len() < 2 {
            return estimate::interpolate(
                &self.samples,
                x,
                |s| s.point,
                |s| s.deviation.max(s.predicted * 0.05),
            );
        }
        estimate::interpolate(
            &filtered,
            x,
            |s| s.point,
            |s| s.deviation.max(s.predicted * 0.05),
        )
    }

    fn derivative(&self, x: f64) -> f64 {
        {
            let active = self.active.borrow();
            if !active.is_empty() {
                let normalized = x / self.normalization;
                let raw: f64 = active
                    .iter()
                    .map(|t| {
                        if t.exponent == 0 {
                            return 0.0;
                        }
                        let e = t.exponent;
                        t.coefficient
                            * f64::from(e)
                            * normalized.powi(e.wrapping_sub(1).cast_signed())
                    })
                    .sum();
                return raw / self.normalization;
            }
        }
        let dx = self.viewport.domain.span() * 0.001;
        let left = self.predicted(x - dx);
        let right = self.predicted(x + dx);
        (right - left) / (2.0 * dx)
    }

    fn nearest(&self, sx: f64, sy: f64) -> Nearest {
        let previous = self.anchor.get();
        if self.locked.get() {
            return Nearest {
                domain: previous,
                predicted: self.predicted(previous),
            };
        }
        let ds = self.viewport.domain.span();
        let mut result = Nearest {
            domain: self.viewport.domain.minimum,
            predicted: 0.0,
        };
        let mut closest = f64::MAX;
        for i in 0..=RESOLUTION {
            let x = self.viewport.domain.minimum + ds * f64::from(i) / f64::from(RESOLUTION);
            let y = self.predicted(x);
            let projected = self.viewport.project(x, y);
            let d = (sx - projected.x).powi(2) + (sy - projected.y).powi(2);
            if d < closest {
                closest = d;
                result.domain = x;
                result.predicted = y;
            }
        }
        self.anchor.set(result.domain);
        result
    }
}

pub fn initialize(document: &Document) {
    let Ok(charts) = document.query_selector_all(&attribute::chart().selector()) else {
        return;
    };
    for index in 0..charts.length() {
        let Some(node) = charts.get(index) else {
            continue;
        };
        let element: web_sys::Element = node.unchecked_ref::<web_sys::Element>().clone();
        if element.get_attribute(attribute::bound().name()).is_some() {
            continue;
        }
        let _ = element.set_attribute(attribute::bound().name(), "");
        bind(document, element);
    }
}

fn bind(document: &Document, svg: web_sys::Element) {
    let Some(data) = svg.get_attribute(attribute::chart().name()) else {
        return;
    };
    let Some((vp, normalization, label, terms, samples)) = decode(&data) else {
        return;
    };
    let Some(overlay) = assemble(document, &svg) else {
        return;
    };
    let midpoint = f64::midpoint(vp.domain.minimum, vp.domain.maximum);
    let excluded = vec![false; samples.len()];
    let active = terms.clone();
    #[expect(clippy::cast_possible_truncation)]
    let duration = DURATION as i32;
    let transition = format!("transition:stop-opacity {duration}ms cubic-bezier(0.382,0,0.618,1)");
    let stops = (0..SLICES)
        .map(|column| {
            let selector = format!("#h{column}");
            svg.query_selector(&selector)
                .ok()
                .flatten()
                .map_or_else(Vec::new, |gradient| {
                    let children = gradient.children();
                    (0..=STOPS)
                        .filter_map(|row| {
                            let stop = children.get_with_index(row)?;
                            let _ = stop.set_attribute("style", &transition);
                            Some(stop)
                        })
                        .collect::<Vec<_>>()
                })
        })
        .collect::<Vec<_>>();
    let line = svg
        .query_selector(&class::chart::winner().selector())
        .ok()
        .flatten();
    let state = Rc::new(RefCell::new(State {
        document: document.clone(),
        svg: svg.clone(),
        viewport: vp,
        normalization,
        label,
        terms,
        samples,
        overlay,
        anchor: Cell::new(midpoint),
        locked: Cell::new(false),
        cursor: Cell::new((0.0, 0.0)),
        excluded: RefCell::new(excluded),
        active: RefCell::new(active),
        generation: Cell::new(0),
        stops,
        line,
    }));
    {
        let state = Rc::clone(&state);
        let callback = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let borrowed = state.borrow();
            let cx = property(event.as_ref(), "clientX");
            let cy = property(event.as_ref(), "clientY");
            let mouse = client(&borrowed, cx, cy);
            borrowed.cursor.set((mouse.x, mouse.y));
            flush(&borrowed, mouse.x, mouse.y);
        });
        let _ =
            svg.add_event_listener_with_callback("mousemove", callback.as_ref().unchecked_ref());
        callback.forget();
    }
    {
        let state = Rc::clone(&state);
        let callback = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let cx = property(event.as_ref(), "clientX");
            let cy = property(event.as_ref(), "clientY");
            let scatter = {
                let borrowed = state.borrow();
                let mouse = client(&borrowed, cx, cy);
                let proximity = proportion::scale(6);
                borrowed
                    .samples
                    .iter()
                    .enumerate()
                    .filter_map(|(i, s)| {
                        let projected = borrowed.viewport.project(s.point, s.mean);
                        let d = (mouse.x - projected.x).powi(2) + (mouse.y - projected.y).powi(2);
                        (d < proximity * proximity).then_some((i, d))
                    })
                    .min_by(|a, b| a.1.total_cmp(&b.1))
                    .map(|(i, _)| i)
            };
            if let Some(index) = scatter {
                {
                    let borrowed = state.borrow();
                    let mut excluded = borrowed.excluded.borrow_mut();
                    excluded[index] = !excluded[index];
                    toggle(&borrowed.svg, index, excluded[index]);
                }
                exclude(&state);
                return;
            }
            let borrowed = state.borrow();
            let mouse = client(&borrowed, cx, cy);
            let domain = borrowed.anchor.get();
            let predicted = borrowed.predicted(domain);
            let anchor = borrowed.viewport.project(domain, predicted);
            let proximate = distance(mouse.x, mouse.y, anchor.x, anchor.y) < TICK_LENGTH * 2.0;
            if let Some(hit) = probe(&borrowed.overlay.pins, mouse.x, mouse.y, TICK_LENGTH * 2.0) {
                if borrowed.locked.get() && proximate {
                    if !connected(&borrowed.overlay.frozen, &hit) {
                        let _ = borrowed.overlay.pins.remove_child(&hit);
                    }
                    borrowed.locked.set(false);
                } else {
                    defrost(&borrowed.overlay.frozen, &borrowed.overlay.pins, &hit);
                    let _ = borrowed.overlay.pins.remove_child(&hit);
                }
            } else if borrowed.locked.get() && proximate {
                borrowed.locked.set(false);
            } else if borrowed.locked.get() {
                let color = palette(borrowed.overlay.frozen.children().length());
                freeze(
                    &borrowed.document,
                    &borrowed.overlay,
                    anchor.x,
                    anchor.y,
                    mouse.x,
                    mouse.y,
                    &color,
                );
                place(
                    &borrowed.document,
                    &borrowed.overlay.pins,
                    mouse.x,
                    mouse.y,
                    &color,
                );
            } else {
                place(
                    &borrowed.document,
                    &borrowed.overlay.pins,
                    anchor.x,
                    anchor.y,
                    "var(--accent)",
                );
                borrowed.locked.set(true);
            }
        });
        let _ = svg.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref());
        callback.forget();
    }
    {
        let state = Rc::clone(&state);
        let callback = Closure::<dyn FnMut(web_sys::Event)>::new(move |_: web_sys::Event| {
            let borrowed = state.borrow();
            if !borrowed.locked.get() {
                let _ = borrowed.overlay.group.set_attribute("opacity", "0");
            }
        });
        let _ =
            svg.add_event_listener_with_callback("mouseleave", callback.as_ref().unchecked_ref());
        callback.forget();
    }
    {
        let state = Rc::clone(&state);
        let callback = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let borrowed = state.borrow();
            if !borrowed.locked.get() {
                return;
            }
            let target = event
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Node>().ok());
            let inside = target.is_some_and(|t| borrowed.svg.contains(Some(&t)));
            if !inside {
                borrowed.locked.set(false);
                let _ = borrowed.overlay.group.set_attribute("opacity", "0");
            }
        });
        let _ = document
            .add_event_listener_with_callback("pointerdown", callback.as_ref().unchecked_ref());
        callback.forget();
    }
}

type Decoded = (Viewport, f64, String, Vec<Term>, Vec<Sample>);

fn decode(data: &str) -> Option<Decoded> {
    use base64::Engine;
    use prost::Message;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(data)
        .ok()?;
    let chart = wire::Chart::decode(&bytes[..]).ok()?;
    let v = chart.viewport?;
    let m = v.margin?;
    let d = v.domain?;
    let r = v.range?;
    let vp = Viewport {
        width: v.width,
        height: v.height,
        margin: viewport::Margin {
            top: m.top,
            right: m.right,
            bottom: m.bottom,
            left: m.left,
        },
        domain: viewport::Extent {
            minimum: d.minimum,
            maximum: d.maximum,
        },
        range: viewport::Extent {
            minimum: r.minimum,
            maximum: r.maximum,
        },
    };
    let terms = chart
        .terms
        .into_iter()
        .map(|t| Term {
            exponent: t.exponent,
            coefficient: t.coefficient,
        })
        .collect::<Vec<_>>();
    let mut samples = chart
        .samples
        .into_iter()
        .map(|s| Sample {
            point: s.point,
            mean: s.mean,
            deviation: s.deviation,
            predicted: s.predicted,
            count: s.count,
        })
        .collect::<Vec<_>>();
    samples.sort_by(|a, b| a.point.total_cmp(&b.point));
    if samples.is_empty() {
        return None;
    }
    Some((vp, chart.normalization, chart.label, terms, samples))
}

fn element(
    document: &Document,
    tag: &str,
    attributes: &[(&str, &str)],
) -> Option<web_sys::Element> {
    let node = document.create_element_ns(Some(NAMESPACE), tag).ok()?;
    for &(key, value) in attributes {
        let _ = node.set_attribute(key, value);
    }
    Some(node)
}

fn assemble(document: &Document, svg: &web_sys::Element) -> Option<Overlay> {
    let overlay = class::chart::overlay().to_string();
    let fade = class::chart::fade().to_string();
    let group = element(
        document,
        "g",
        &[
            ("opacity", "0"),
            ("pointer-events", "none"),
            ("class", &overlay),
        ],
    )?;

    let indicator = element(
        document,
        "line",
        &[
            ("stroke", "var(--text)"),
            ("stroke-width", "0.75"),
            ("opacity", "0.25"),
            ("stroke-dasharray", "3 3"),
        ],
    )?;

    let tangent = element(
        document,
        "line",
        &[
            ("stroke", "var(--accent)"),
            ("stroke-width", "0.75"),
            ("opacity", "0.25"),
            ("stroke-dasharray", "5 3"),
        ],
    )?;

    let band = element(
        document,
        "line",
        &[
            ("stroke", "var(--accent)"),
            ("stroke-width", "10"),
            ("opacity", "0.12"),
            ("stroke-linecap", "round"),
        ],
    )?;

    let marker = element(
        document,
        "line",
        &[
            ("stroke", "var(--accent)"),
            ("stroke-width", "1.5"),
            ("opacity", "0.6"),
            ("stroke-linecap", "round"),
        ],
    )?;

    let pins = element(document, "g", &[])?;
    let frozen = element(document, "g", &[])?;

    let vector = element(
        document,
        "line",
        &[
            ("stroke", "var(--accent)"),
            ("stroke-width", "1"),
            ("opacity", "0"),
            ("stroke-dasharray", "4 3"),
            ("class", &fade),
        ],
    )?;

    let projection: &[(&str, &str)] = &[
        ("stroke", "var(--text-secondary)"),
        ("stroke-width", "0.5"),
        ("opacity", "0"),
        ("stroke-dasharray", "2 2"),
        ("class", &fade),
    ];
    let perpendicular = element(document, "line", projection)?;
    let parallel = element(document, "line", projection)?;

    let cursor = element(
        document,
        "circle",
        &[
            ("r", "3.5"),
            ("fill", "var(--accent)"),
            ("stroke", "var(--code-background)"),
            ("stroke-width", "2"),
        ],
    )?;

    let backing = class::chart::backing().to_string();
    let background = element(
        document,
        "rect",
        &[
            ("rx", "6"),
            ("fill", "var(--code-background)"),
            ("filter", "url(#shadow)"),
            ("class", &backing),
        ],
    )?;

    let heading = class::chart::heading().to_string();
    let density = element(document, "text", &[("class", &heading)])?;

    let body = class::chart::body().to_string();
    let position = element(document, "text", &[("class", &body)])?;
    let mean = element(document, "text", &[("class", &body)])?;
    let slope = element(document, "text", &[("class", &body)])?;

    let dim = class::chart::dim().to_string();
    let label = element(document, "text", &[("class", &dim)])?;
    let count = element(document, "text", &[("class", &dim)])?;
    let merged = element(document, "g", &[])?;

    for child in [
        &indicator,
        &tangent,
        &band,
        &marker,
        &vector,
        &perpendicular,
        &parallel,
        &cursor,
        &background,
        &density,
        &position,
        &mean,
        &slope,
        &label,
        &count,
        &merged,
    ] {
        let _ = group.append_child(child);
    }
    let _ = svg.append_child(&pins);
    let _ = svg.append_child(&frozen);
    let _ = svg.append_child(&group);

    Some(Overlay {
        group,
        indicator,
        tangent,
        band,
        marker,
        pins,
        frozen,
        vector,
        perpendicular,
        parallel,
        cursor,
        background,
        density,
        position,
        mean,
        slope,
        label,
        count,
        merged,
    })
}

fn client(state: &State, cx: f64, cy: f64) -> viewport::Point {
    let this: &JsValue = state.svg.as_ref();
    let rect: JsValue = js_sys::Reflect::get(this, &"getBoundingClientRect".into())
        .ok()
        .and_then(|f| {
            let func: &js_sys::Function = f.unchecked_ref();
            func.call0(this).ok()
        })
        .unwrap_or(JsValue::NULL);
    let left = property(&rect, "left");
    let top = property(&rect, "top");
    let width = property(&rect, "width").max(1.0);
    let height = property(&rect, "height").max(1.0);
    viewport::Point {
        x: (cx - left) * state.viewport.width / width,
        y: (cy - top) * state.viewport.height / height,
    }
}

fn palette(index: u32) -> String {
    let hue = (f64::from(index) * 137.5) % 360.0;
    format!("hsl({hue:.0} 65% 55%)")
}

fn promote(target: &web_sys::Element, color: &str) {
    let _ = target.set_attribute("class", &class::chart::heading().to_string());
    let _ = target.set_attribute("style", &format!("fill: {color}"));
}

fn place(document: &Document, container: &web_sys::Element, sx: f64, sy: f64, color: &str) {
    let Some(dot) = element(
        document,
        "circle",
        &[("r", "3"), ("fill", color), ("opacity", "0.8")],
    ) else {
        return;
    };
    coordinate(&dot, "cx", sx);
    coordinate(&dot, "cy", sy);
    let _ = container.append_child(&dot);
}

fn freeze(document: &Document, overlay: &Overlay, x1: f64, y1: f64, x2: f64, y2: f64, color: &str) {
    let Some(wrapper) = element(document, "g", &[]) else {
        return;
    };
    let _ = wrapper.set_attribute("data-x1", &format!("{x1:.1}"));
    let _ = wrapper.set_attribute("data-y1", &format!("{y1:.1}"));
    let _ = wrapper.set_attribute("data-x2", &format!("{x2:.1}"));
    let _ = wrapper.set_attribute("data-y2", &format!("{y2:.1}"));
    let _ = wrapper.set_attribute("data-color", color);

    let Some(line) = element(
        document,
        "line",
        &[
            ("stroke", color),
            ("stroke-width", "1"),
            ("opacity", "0.6"),
            ("stroke-dasharray", "4 3"),
        ],
    ) else {
        return;
    };
    segment(&line, x1, y1, x2, y2);
    let _ = wrapper.append_child(&line);

    if let Some(bg) = element(
        document,
        "rect",
        &[
            ("rx", "6"),
            ("fill", "var(--code-background)"),
            ("opacity", &format!("{:.3}", 1.0 / proportion::PHI)),
            ("filter", "url(#shadow)"),
        ],
    ) {
        for attr in ["x", "y", "width", "height"] {
            if let Some(value) = overlay.background.get_attribute(attr) {
                let _ = bg.set_attribute(attr, &value);
            }
        }
        let _ = wrapper.append_child(&bg);
    }

    let sources = [
        &overlay.density,
        &overlay.position,
        &overlay.mean,
        &overlay.slope,
        &overlay.label,
        &overlay.count,
    ];
    for (index, source) in sources.iter().enumerate() {
        if let Some(clone) = snapshot(document, source) {
            if index == 0 {
                promote(&clone, color);
            }
            let _ = wrapper.append_child(&clone);
        }
    }

    let _ = overlay.frozen.append_child(&wrapper);
}

fn snapshot(document: &Document, source: &web_sys::Element) -> Option<web_sys::Element> {
    let class = source.get_attribute("class").unwrap_or_default();
    let style = source.get_attribute("style").unwrap_or_default();
    let clone = element(document, "text", &[("class", &class), ("style", &style)])?;
    clone.set_text_content(source.text_content().as_deref());
    for attr in ["x", "y", "opacity"] {
        if let Some(value) = source.get_attribute(attr) {
            let _ = clone.set_attribute(attr, &value);
        }
    }
    Some(clone)
}

fn defrost(frozen: &web_sys::Element, pins: &web_sys::Element, pin: &web_sys::Element) {
    let px = numeric(pin, "cx");
    let py = numeric(pin, "cy");
    let children = frozen.children();
    let mut removals = vec![];
    let mut orphans = vec![];
    let mut origins = vec![];
    for i in 0..children.length() {
        let Some(child) = children.get_with_index(i) else {
            continue;
        };
        let x1 = numeric(&child, "data-x1");
        let y1 = numeric(&child, "data-y1");
        let x2 = numeric(&child, "data-x2");
        let y2 = numeric(&child, "data-y2");
        if distance(px, py, x1, y1) < 1.0 {
            removals.push(child);
            orphans.push((x2, y2));
        } else if distance(px, py, x2, y2) < 1.0 {
            removals.push(child);
            origins.push((x1, y1));
        }
    }
    for child in removals {
        let _ = frozen.remove_child(&child);
    }
    let mut seen = std::collections::HashSet::new();
    for (ox, oy) in orphans.into_iter().chain(origins) {
        if !seen.insert((ox.to_bits(), oy.to_bits())) {
            continue;
        }
        if let Some(orphan) = probe(pins, ox, oy, 1.0)
            && !connected(frozen, &orphan)
        {
            let _ = pins.remove_child(&orphan);
        }
    }
}

fn probe(
    container: &web_sys::Element,
    sx: f64,
    sy: f64,
    threshold: f64,
) -> Option<web_sys::Element> {
    let children = container.children();
    for i in 0..children.length() {
        let Some(child) = children.get_with_index(i) else {
            continue;
        };
        if distance(sx, sy, numeric(&child, "cx"), numeric(&child, "cy")) < threshold {
            return Some(child);
        }
    }
    None
}

fn connected(frozen: &web_sys::Element, pin: &web_sys::Element) -> bool {
    let px = numeric(pin, "cx");
    let py = numeric(pin, "cy");
    let children = frozen.children();
    for i in 0..children.length() {
        let Some(child) = children.get_with_index(i) else {
            continue;
        };
        if distance(
            px,
            py,
            numeric(&child, "data-x1"),
            numeric(&child, "data-y1"),
        ) < 1.0
            || distance(
                px,
                py,
                numeric(&child, "data-x2"),
                numeric(&child, "data-y2"),
            ) < 1.0
        {
            return true;
        }
    }
    false
}

fn reach(
    origin: &viewport::Point,
    direction: &viewport::Point,
    minimum: &viewport::Point,
    maximum: &viewport::Point,
) -> (f64, f64) {
    let mut forward = f64::MAX;
    let mut backward = f64::MAX;
    for &(d, o, lo, hi) in &[
        (direction.x, origin.x, minimum.x, maximum.x),
        (direction.y, origin.y, minimum.y, maximum.y),
    ] {
        if d.abs() > 1e-12 {
            let lower = (lo - o) / d;
            let upper = (hi - o) / d;
            if lower > 0.0 {
                forward = forward.min(lower);
            } else {
                backward = backward.min(lower.abs());
            }
            if upper > 0.0 {
                forward = forward.min(upper);
            } else {
                backward = backward.min(upper.abs());
            }
        }
    }
    (forward.max(0.0), backward.max(0.0))
}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

fn segment(line: &web_sys::Element, x1: f64, y1: f64, x2: f64, y2: f64) {
    coordinate(line, "x1", x1);
    coordinate(line, "y1", y1);
    coordinate(line, "x2", x2);
    coordinate(line, "y2", y2);
}

struct Cursor {
    sx: f64,
    sy: f64,
    anchor: viewport::Point,
    px: f64,
    py: f64,
    tx: f64,
    ty: f64,
    domain: f64,
    observed: f64,
    mean: f64,
    deviation: f64,
    slope: f64,
    spread: f64,
    density: f64,
}

fn flush(state: &State, mx: f64, my: f64) {
    if !state.viewport.contains(mx, my) && !state.locked.get() {
        let _ = state.overlay.group.set_attribute("opacity", "0");
        return;
    }
    let _ = state.overlay.group.set_attribute("opacity", "1");

    let mouse = viewport::Point { x: mx, y: my };
    let cursor = compute(state, &mouse);
    position(state, &cursor);
    populate(state, &cursor);
}

fn compute(state: &State, mouse: &viewport::Point) -> Cursor {
    let nearest = state.nearest(mouse.x, mouse.y);
    let deviation = state.deviation(nearest.domain);
    let anchor = state.viewport.project(nearest.domain, nearest.predicted);

    let slope = state.derivative(nearest.domain);
    let visual = -slope * state.viewport.aspect();
    let length = (1.0 + visual * visual).sqrt();
    let px = -visual / length;
    let py = 1.0 / length;
    let tx = py;
    let ty = -px;

    let area = state.viewport.plot();
    let rs = state.viewport.range.span();
    let sigma = deviation.max(nearest.predicted.abs() * 0.05).max(rs * 0.01);
    let spread = sigma * area.height / rs;
    let perpendicular = (mouse.x - anchor.x) * px + (mouse.y - anchor.y) * py;
    let z = if spread > 0.0 {
        perpendicular / spread
    } else {
        0.0
    };

    let observed = state.viewport.unproject(mouse.x, mouse.y).y;

    Cursor {
        sx: mouse.x,
        sy: mouse.y,
        anchor,
        px,
        py,
        tx,
        ty,
        domain: nearest.domain,
        observed,
        mean: nearest.predicted,
        deviation,
        slope,
        spread,
        density: (-0.5 * z * z).exp(),
    }
}

fn position(state: &State, cursor: &Cursor) {
    let area = state.viewport.plot();
    let extent = area.height * 0.8;
    segment(
        &state.overlay.indicator,
        cursor.anchor.x - cursor.px * extent,
        cursor.anchor.y - cursor.py * extent,
        cursor.anchor.x + cursor.px * extent,
        cursor.anchor.y + cursor.py * extent,
    );
    segment(
        &state.overlay.band,
        cursor.anchor.x - cursor.px * cursor.spread,
        cursor.anchor.y - cursor.py * cursor.spread,
        cursor.anchor.x + cursor.px * cursor.spread,
        cursor.anchor.y + cursor.py * cursor.spread,
    );

    let minimum = viewport::Point {
        x: state.viewport.margin.left,
        y: state.viewport.margin.top,
    };
    let maximum = viewport::Point {
        x: minimum.x + area.width,
        y: minimum.y + area.height,
    };
    let direction = viewport::Point {
        x: cursor.tx,
        y: cursor.ty,
    };
    let (forward, backward) = reach(&cursor.anchor, &direction, &minimum, &maximum);
    segment(
        &state.overlay.tangent,
        cursor.anchor.x - cursor.tx * backward,
        cursor.anchor.y - cursor.ty * backward,
        cursor.anchor.x + cursor.tx * forward,
        cursor.anchor.y + cursor.ty * forward,
    );

    segment(
        &state.overlay.marker,
        cursor.anchor.x - cursor.tx * TICK_LENGTH,
        cursor.anchor.y - cursor.ty * TICK_LENGTH,
        cursor.anchor.x + cursor.tx * TICK_LENGTH,
        cursor.anchor.y + cursor.ty * TICK_LENGTH,
    );

    coordinate(&state.overlay.cursor, "cx", cursor.sx);
    coordinate(&state.overlay.cursor, "cy", cursor.sy);

    let locked = state.locked.get();
    let opacity = if locked { "1" } else { "0" };
    for line in [
        &state.overlay.vector,
        &state.overlay.perpendicular,
        &state.overlay.parallel,
    ] {
        let _ = line.set_attribute("opacity", opacity);
    }
    if locked {
        let dx = cursor.sx - cursor.anchor.x;
        let dy = cursor.sy - cursor.anchor.y;
        let normal = dx * cursor.px + dy * cursor.py;
        let tangential = dx * cursor.tx + dy * cursor.ty;
        let perpendicular = viewport::Point {
            x: cursor.anchor.x + normal * cursor.px,
            y: cursor.anchor.y + normal * cursor.py,
        };
        let parallel = viewport::Point {
            x: cursor.anchor.x + tangential * cursor.tx,
            y: cursor.anchor.y + tangential * cursor.ty,
        };
        segment(
            &state.overlay.vector,
            cursor.anchor.x,
            cursor.anchor.y,
            cursor.sx,
            cursor.sy,
        );
        segment(
            &state.overlay.perpendicular,
            cursor.sx,
            cursor.sy,
            perpendicular.x,
            perpendicular.y,
        );
        segment(
            &state.overlay.parallel,
            cursor.sx,
            cursor.sy,
            parallel.x,
            parallel.y,
        );
    }
}

fn populate(state: &State, cursor: &Cursor) {
    let percent = format!("{:.0}%", cursor.density * 100.0);
    let mean = format::duration(cursor.mean);
    let deviation = format::duration(cursor.deviation);

    let value = format::duration(cursor.observed);

    state.overlay.density.set_text_content(Some(&percent));
    state.overlay.position.set_text_content(Some(&format!(
        "{}: {}  y: {value}",
        state.label,
        format::notation(cursor.domain),
    )));
    state
        .overlay
        .mean
        .set_text_content(Some(&format!("\u{03bc} = {mean} \u{00b1} {deviation}")));
    state.overlay.slope.set_text_content(Some(&format!(
        "tangent = {}",
        format::notation(cursor.slope)
    )));

    let proximity = proportion::scale(6);
    let sample = closest(state, cursor.sx, cursor.sy, proximity);
    let expanded = if let Some(sample) = sample {
        state.overlay.label.set_text_content(Some(&format!(
            "{}: {}",
            state.label,
            format::notation(sample.point)
        )));
        state
            .overlay
            .count
            .set_text_content(Some(&format!("samples: {}", sample.count)));
        let _ = state.overlay.label.set_attribute("opacity", "1");
        let _ = state.overlay.count.set_attribute("opacity", "1");
        true
    } else {
        let _ = state.overlay.label.set_attribute("opacity", "0");
        let _ = state.overlay.count.set_attribute("opacity", "0");
        false
    };

    let content = measure(&state.overlay.density)
        .max(measure(&state.overlay.position))
        .max(measure(&state.overlay.mean))
        .max(measure(&state.overlay.slope));
    let width = content + CARD_PADDING * 2.0;
    let area = state.viewport.plot();
    let spacing = proportion::scale(5);
    let card = layout(state, cursor.sx, cursor.sy, area.height, expanded, width);

    coordinate(&state.overlay.background, "x", card.x);
    coordinate(&state.overlay.background, "y", card.y);
    coordinate(&state.overlay.background, "width", card.width);
    coordinate(&state.overlay.background, "height", card.height);

    let left = card.x + CARD_PADDING;
    let rows = [
        (&state.overlay.density, ROW_ORIGIN),
        (&state.overlay.position, ROW_ORIGIN + spacing),
        (&state.overlay.mean, ROW_ORIGIN + spacing * 2.0),
        (&state.overlay.slope, ROW_ORIGIN + spacing * 3.0),
        (&state.overlay.label, ROW_ORIGIN + spacing * 4.0),
        (&state.overlay.count, ROW_ORIGIN + spacing * 5.0),
    ];
    for (text, offset) in rows {
        coordinate(text, "x", left);
        coordinate(text, "y", card.y + offset);
    }

    let extra = merge(state, &card, spacing);
    if extra > 0.0 {
        coordinate(&state.overlay.background, "height", card.height + extra);
    }
}

fn merge(state: &State, card: &Card, spacing: f64) -> f64 {
    state.overlay.merged.set_text_content(None);

    let frozen = &state.overlay.frozen;
    let wrappers = frozen.children();

    let mut offset = card.height;
    let mut height = card.height;
    let left = card.x + CARD_PADDING;

    for i in 0..wrappers.length() {
        let Some(wrapper) = wrappers.get_with_index(i) else {
            continue;
        };
        let children = wrapper.children();
        let Some(bg) = children.get_with_index(1) else {
            continue;
        };
        let fx = numeric(&bg, "x");
        let fy = numeric(&bg, "y");
        let fw = numeric(&bg, "width");
        let fh = numeric(&bg, "height");

        let horizontal = (card.x + card.width).min(fx + fw) > card.x.max(fx);
        let vertical = (card.y + height).min(fy + fh) > card.y.max(fy);
        if horizontal && vertical {
            for j in 1..children.length() {
                if let Some(child) = children.get_with_index(j) {
                    let _ = child.set_attribute("visibility", "hidden");
                }
            }

            let color = wrapper
                .get_attribute("data-color")
                .unwrap_or_else(|| "var(--text-secondary)".into());
            let sep = class::chart::dim().to_string();
            if let Some(divider) = element(
                &state.document,
                "text",
                &[
                    ("class", &sep),
                    ("style", &format!("fill: {color}; font-weight: 600")),
                ],
            ) {
                divider.set_text_content(Some("\u{2014}\u{2014}"));
                coordinate(&divider, "x", left);
                coordinate(&divider, "y", card.y + offset);
                let _ = state.overlay.merged.append_child(&divider);
            }
            offset += spacing;

            for j in 2..children.length() {
                let Some(source) = children.get_with_index(j) else {
                    continue;
                };
                let opacity = source
                    .get_attribute("opacity")
                    .unwrap_or_else(|| "1".into());
                if opacity == "0" {
                    continue;
                }
                let text = source.text_content().unwrap_or_default();
                if text.is_empty() {
                    continue;
                }
                if let Some(clone) = snapshot(&state.document, &source) {
                    if j == 2 {
                        promote(&clone, &color);
                    }
                    coordinate(&clone, "x", left);
                    coordinate(&clone, "y", card.y + offset);
                    let _ = state.overlay.merged.append_child(&clone);
                }
                offset += spacing;
            }
            height = offset;
        } else {
            for j in 1..children.length() {
                if let Some(child) = children.get_with_index(j) {
                    let _ = child.remove_attribute("visibility");
                }
            }
        }
    }

    if offset > card.height {
        offset - spacing + CARD_PADDING - card.height
    } else {
        0.0
    }
}

fn closest(state: &State, sx: f64, sy: f64, proximity: f64) -> Option<&Sample> {
    state
        .samples
        .iter()
        .filter_map(|s| {
            let projected = state.viewport.project(s.point, s.mean);
            let distance = (sx - projected.x).powi(2) + (sy - projected.y).powi(2);
            (distance < proximity * proximity).then_some((s, distance))
        })
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(sample, _)| sample)
}

struct Card {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

fn measure(target: &web_sys::Element) -> f64 {
    js_sys::Reflect::get(target, &"getComputedTextLength".into())
        .ok()
        .and_then(|f| {
            let func: &js_sys::Function = f.unchecked_ref();
            func.call0(target).ok()
        })
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0)
}

fn layout(state: &State, sx: f64, sy: f64, ah: f64, expanded: bool, width: f64) -> Card {
    let height = if expanded { CARD_EXPANDED } else { CARD_HEIGHT };
    let area = state.viewport.plot();
    let left = state.viewport.margin.left;
    let top = state.viewport.margin.top;
    let right = left + area.width;
    let bottom = top + ah;

    let candidates: [(f64, f64); 4] = [
        (sx + CARD_GAP, sy - CARD_HEIGHT / 2.0),
        (sx - width - CARD_GAP, sy - CARD_HEIGHT / 2.0),
        (sx - width / 2.0, sy - CARD_HEIGHT - CARD_GAP),
        (sx - width / 2.0, sy + CARD_GAP),
    ];

    candidates
        .iter()
        .map(|&(cx, cy)| {
            let clamped = Card {
                x: cx.clamp(left, right - width),
                y: cy.clamp(top, bottom - height),
                width,
                height,
            };
            let horizontal = (clamped.x + clamped.width).min(sx + 4.0) - clamped.x.max(sx - 4.0);
            let vertical = (clamped.y + clamped.height).min(sy + 4.0) - clamped.y.max(sy - 4.0);
            let occlusion = if horizontal > 0.0 && vertical > 0.0 {
                horizontal * vertical
            } else {
                0.0
            };
            let penalty = (cx - clamped.x).abs() + (cy - clamped.y).abs();
            (clamped, occlusion + penalty)
        })
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(card, _)| card)
        .unwrap_or(Card {
            x: sx + CARD_GAP,
            y: (sy - height / 2.0).clamp(top, bottom - height),
            width,
            height,
        })
}

fn numeric(target: &web_sys::Element, attribute: &str) -> f64 {
    target
        .get_attribute(attribute)
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(0.0)
}

fn coordinate(target: &web_sys::Element, attribute: &str, value: f64) {
    let _ = target.set_attribute(attribute, &format!("{value:.1}"));
}

fn property(target: &JsValue, name: &str) -> f64 {
    js_sys::Reflect::get(target, &name.into())
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0)
}

fn toggle(svg: &web_sys::Element, index: usize, excluded: bool) {
    let selector = format!("circle[data-index=\"{index}\"]");
    let Some(circle) = svg.query_selector(&selector).ok().flatten() else {
        return;
    };
    let list = circle.class_list();
    if excluded {
        let _ = list.add_1("excluded");
    } else {
        let _ = list.remove_1("excluded");
    }
}

fn exclude(state: &Rc<RefCell<State>>) {
    let (previous, updated) = {
        let borrowed = state.borrow();
        let previous = borrowed.active.borrow().clone();
        let updated = refit(&borrowed);
        (previous, updated)
    };
    animate(state, previous, updated);
}

fn refit(state: &State) -> Vec<Term> {
    if state.terms.is_empty() {
        return Vec::new();
    }
    let excluded = state.excluded.borrow();
    let input = state
        .samples
        .iter()
        .enumerate()
        .filter(|(i, _)| !excluded[*i])
        .map(|(_, s)| regression::Sample {
            point: vec![s.point / state.normalization],
            observation: s.mean,
        })
        .collect::<Vec<_>>();
    if input.len() < 2 {
        return state.terms.clone();
    }
    let degree = state.terms.iter().map(|t| t.exponent).max().unwrap_or(1) as usize;
    let Some(selection) = regression::select(&input, 1, degree) else {
        return state.terms.clone();
    };
    let winner = selection.winner();
    winner
        .polynomial
        .terms
        .iter()
        .map(|t| {
            #[expect(clippy::cast_possible_truncation)]
            let exponent = t.monomial.exponent.first().copied().unwrap_or(0) as u32;
            Term {
                exponent,
                coefficient: t.coefficient,
            }
        })
        .collect::<Vec<_>>()
}

fn animate(state: &Rc<RefCell<State>>, previous: Vec<Term>, target: Vec<Term>) {
    let dur = format!("{:.3}s", DURATION / 1000.0);
    {
        let borrowed = state.borrow();
        let grid = opacity_grid(&borrowed, &target);
        for (column, column_stops) in borrowed.stops.iter().enumerate() {
            for (row, stop) in column_stops.iter().enumerate() {
                let _ = stop.set_attribute("stop-opacity", &format!("{:.3}", grid[column][row]));
            }
        }
        if let Some(line) = &borrowed.line {
            while let Ok(Some(child)) = line.query_selector("animate") {
                let _ = line.remove_child(&child);
            }
            let from = points_string(&borrowed, &previous);
            let to = points_string(&borrowed, &target);
            let _ = line.set_attribute("points", &from);
            if let Some(anim) = element(
                &borrowed.document,
                "animate",
                &[
                    ("attributeName", "points"),
                    ("from", &from),
                    ("to", &to),
                    ("dur", &dur),
                    ("fill", "freeze"),
                    ("calcMode", "spline"),
                    ("keyTimes", "0;1"),
                    ("keySplines", "0.382 0 0.618 1"),
                ],
            ) {
                let _ = line.append_child(&anim);
            }
        }
    }
    let epoch = {
        let borrowed = state.borrow();
        let next = borrowed.generation.get().wrapping_add(1);
        borrowed.generation.set(next);
        next
    };
    let start = js_sys::Date::now();
    #[expect(clippy::type_complexity)]
    let callback: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let looped = Rc::clone(&callback);
    let state = Rc::clone(state);
    *callback.borrow_mut() = Some(Closure::new(move || {
        let borrowed = state.borrow();
        if borrowed.generation.get() != epoch {
            let _ = looped.borrow_mut().take();
            return;
        }
        let elapsed = js_sys::Date::now() - start;
        let raw = (elapsed / DURATION).min(1.0);
        let t = raw * raw * (3.0 - 2.0 * raw);
        let blended = blend(&previous, &target, t);
        borrowed.active.borrow_mut().clone_from(&blended);
        if raw >= 1.0 {
            if let Some(line) = &borrowed.line {
                let final_points = points_string(&borrowed, &target);
                let _ = line.set_attribute("points", &final_points);
                while let Ok(Some(child)) = line.query_selector("animate") {
                    let _ = line.remove_child(&child);
                }
            }
            let (mx, my) = borrowed.cursor.get();
            if borrowed.viewport.contains(mx, my) {
                flush(&borrowed, mx, my);
            }
            let _ = looped.borrow_mut().take();
        } else {
            let window = web_sys::window().unwrap();
            let _ = window.request_animation_frame(
                looped.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
            );
        }
    }));
    let window = web_sys::window().unwrap();
    let _ = window
        .request_animation_frame(callback.borrow().as_ref().unwrap().as_ref().unchecked_ref());
}

fn blend(previous: &[Term], target: &[Term], t: f64) -> Vec<Term> {
    let span = previous
        .iter()
        .chain(target.iter())
        .map(|t| t.exponent)
        .max()
        .unwrap_or(0);
    (0..=span)
        .map(|e| {
            let p = previous
                .iter()
                .find(|t| t.exponent == e)
                .map_or(0.0, |t| t.coefficient);
            let q = target
                .iter()
                .find(|t| t.exponent == e)
                .map_or(0.0, |t| t.coefficient);
            Term {
                exponent: e,
                coefficient: p + (q - p) * t,
            }
        })
        .collect::<Vec<_>>()
}

fn points_string(state: &State, terms: &[Term]) -> String {
    let mut points = String::new();
    if terms.is_empty() {
        let excluded = state.excluded.borrow();
        let filtered = state
            .samples
            .iter()
            .enumerate()
            .filter(|(i, _)| !excluded[*i])
            .map(|(_, s)| s)
            .collect::<Vec<_>>();
        for sample in &filtered {
            let y = sample
                .predicted
                .clamp(state.viewport.range.minimum, state.viewport.range.maximum);
            let projected = state.viewport.project(sample.point, y);
            if !points.is_empty() {
                points.push(' ');
            }
            let _ = write!(points, "{:.1},{:.1}", projected.x, projected.y);
        }
    } else {
        let span = state.viewport.domain.span();
        for i in 0..=RESOLUTION {
            let x = state.viewport.domain.minimum + span * f64::from(i) / f64::from(RESOLUTION);
            let normalized = x / state.normalization;
            let y: f64 = terms
                .iter()
                .map(|t| t.coefficient * normalized.powi(t.exponent.cast_signed()))
                .sum();
            let y = y.clamp(state.viewport.range.minimum, state.viewport.range.maximum);
            let projected = state.viewport.project(x, y);
            if !points.is_empty() {
                points.push(' ');
            }
            let _ = write!(points, "{:.1},{:.1}", projected.x, projected.y);
        }
    }
    points
}

fn opacity_grid(state: &State, terms: &[Term]) -> Vec<Vec<f64>> {
    let aspect = state.viewport.aspect();
    let excluded = state.excluded.borrow();
    let filtered = state
        .samples
        .iter()
        .enumerate()
        .filter(|(i, _)| !excluded[*i])
        .map(|(_, s)| s)
        .collect::<Vec<_>>();
    let range_floor = state.viewport.range.span() * 0.01;
    (0..SLICES)
        .map(|column| {
            let fraction = (f64::from(column) + 0.5) / f64::from(SLICES);
            let x = state.viewport.domain.minimum + state.viewport.domain.span() * fraction;
            let center = if terms.is_empty() {
                if filtered.len() < 2 {
                    estimate::interpolate(&state.samples, x, |s| s.point, |s| s.predicted)
                } else {
                    estimate::interpolate(&filtered, x, |s| s.point, |s| s.predicted)
                }
            } else {
                evaluate_terms(terms, x / state.normalization)
            };
            let raw = if filtered.len() < 2 {
                estimate::interpolate(&state.samples, x, |s| s.point, |s| s.deviation)
            } else {
                estimate::interpolate(&filtered, x, |s| s.point, |s| s.deviation)
            };
            let base = raw.max(center.abs() * 0.05).max(range_floor);
            let visual = if terms.is_empty() {
                0.0
            } else {
                derive_terms(terms, x / state.normalization) / state.normalization * aspect
            };
            let sigma = base * (1.0 + visual * visual).sqrt();
            (0..=STOPS)
                .map(|row| {
                    let t = f64::from(row) / f64::from(STOPS);
                    let y = state.viewport.range.maximum - state.viewport.range.span() * t;
                    let z = if sigma > 0.0 {
                        (y - center) / sigma
                    } else {
                        0.0
                    };
                    PEAK * (-z * z / 2.0).exp()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn evaluate_terms(terms: &[Term], normalized: f64) -> f64 {
    terms
        .iter()
        .map(|t| t.coefficient * normalized.powi(t.exponent.cast_signed()))
        .sum()
}

fn derive_terms(terms: &[Term], normalized: f64) -> f64 {
    terms
        .iter()
        .map(|t| {
            if t.exponent == 0 {
                return 0.0;
            }
            let e = t.exponent;
            t.coefficient * f64::from(e) * normalized.powi(e.wrapping_sub(1).cast_signed())
        })
        .sum()
}
