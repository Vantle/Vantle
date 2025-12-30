use proportion::scale;

fn base() -> f32 {
    10.0
}

#[must_use]
pub fn glyph() -> f32 {
    scale().base(base()).k(2.0).compute()
}

#[must_use]
pub fn stroke() -> f32 {
    scale().base(base()).k(-2.0).compute()
}

#[must_use]
pub fn offset() -> f32 {
    scale().base(base()).k(1.0).compute()
}

#[must_use]
pub fn horizontal() -> f32 {
    scale().base(base()).compute()
}

#[must_use]
pub fn vertical() -> f32 {
    scale().base(base()).k(-1.0).compute()
}

#[must_use]
pub fn margin() -> f32 {
    scale().base(base()).k(-2.0).compute()
}

#[must_use]
pub fn font() -> f32 {
    scale().base(base()).k(1.0).compute()
}

#[must_use]
pub fn leading() -> f32 {
    scale().base(base()).k(2.0).compute()
}

#[must_use]
#[expect(clippy::cast_precision_loss)]
pub fn repulsion(count: usize) -> f32 {
    scale()
        .base(5000.0)
        .k((count as f32 / 8.0).min(3.0))
        .compute()
}

#[must_use]
#[expect(clippy::cast_precision_loss)]
pub fn attraction(count: usize) -> f32 {
    scale()
        .base(0.01)
        .k(-((count as f32 / 12.0).min(2.0)))
        .compute()
}

#[must_use]
pub fn radius(viewport: f32) -> f32 {
    scale().base(viewport).k(-1.0).compute()
}

#[must_use]
pub fn pivot() -> f32 {
    scale().k(-11.0).compute()
}

#[must_use]
pub fn drift(distance: f32) -> f32 {
    scale().base(distance).k(-17.0).compute()
}

#[must_use]
pub fn tolerance() -> f32 {
    1.0 + 2.0 / scale().k(6.0).compute()
}

#[must_use]
pub fn near() -> f32 {
    scale().k(-4.0).compute()
}

#[must_use]
pub fn far() -> f32 {
    scale().base(1000.0).k(4.0).compute()
}
