use vector::Vector;

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    Vector::from(a).dot(&Vector::from(b))
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    Vector::from(a).cross(&Vector::from(b)).array()
}

fn magnitude(a: [f32; 3]) -> f32 {
    Vector::from(a).magnitude()
}

fn normalize(a: [f32; 3]) -> [f32; 3] {
    Vector::from(a).normalize().array()
}

fn lerp(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    Vector::from(a).lerp(&Vector::from(b), t).array()
}

fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    (Vector::from(a) + Vector::from(b)).array()
}
