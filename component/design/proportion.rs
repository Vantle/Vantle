pub const PHI: f64 = 1.618_033_988_749_895;

#[must_use]
pub fn scale(k: i32) -> f64 {
    PHI.powi(k)
}

#[must_use]
pub fn half(k: i32) -> f64 {
    PHI.powf(f64::from(k) - 0.5)
}
