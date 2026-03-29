#[must_use]
pub fn interpolate<T>(
    sorted: &[T],
    x: f64,
    point: impl Fn(&T) -> f64,
    value: impl Fn(&T) -> f64,
) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    if sorted.len() == 1 || x <= point(&sorted[0]) {
        return value(&sorted[0]);
    }
    let last = &sorted[sorted.len() - 1];
    if x >= point(last) {
        return value(last);
    }
    let index = sorted
        .binary_search_by(|s| point(s).total_cmp(&x))
        .unwrap_or_else(|i| i);
    let right = index.min(sorted.len() - 1);
    let left = right.saturating_sub(1);
    let span = point(&sorted[right]) - point(&sorted[left]);
    let t = if span > 0.0 {
        (x - point(&sorted[left])) / span
    } else {
        0.5
    };
    value(&sorted[left]) * (1.0 - t) + value(&sorted[right]) * t
}
