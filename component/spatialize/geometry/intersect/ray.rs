#[must_use]
pub fn nearest(a: f32, b: f32, c: f32) -> Option<f32> {
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }
    let t = (-b - discriminant.sqrt()) / (2.0 * a);
    (t > 0.0).then_some(t)
}
