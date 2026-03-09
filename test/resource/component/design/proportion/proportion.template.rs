use proportion::{PHI, half, scale};

fn phi() -> f64 {
    PHI
}

fn proportion(k: i32) -> f64 {
    scale(k)
}

fn midpoint(k: i32) -> f64 {
    half(k)
}
