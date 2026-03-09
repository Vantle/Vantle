use color::Color;

fn hex(value: u32) -> [f32; 4] {
    Color::hex(value).array()
}

fn lerp(a: [f32; 4], b: [f32; 4], t: f32) -> [f32; 4] {
    Color::from(a).lerp(Color::from(b), t).array()
}

fn transparent() -> [f32; 4] {
    Color::transparent().array()
}
