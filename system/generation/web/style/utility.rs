use observe::trace;

#[trace(channels = [document])]
#[must_use]
pub fn scale(k: i32) -> String {
    format!("{}rem", proportion::scale(k))
}

#[must_use]
pub fn half(k: i32) -> String {
    format!("{}rem", proportion::half(k))
}

fn channel(hex: &str, offset: usize) -> u8 {
    u8::from_str_radix(&hex[offset..offset + 2], 16).unwrap_or(0)
}

#[must_use]
pub fn tint(hex: &str, alpha: f32) -> String {
    let r = channel(hex, 1);
    let g = channel(hex, 3);
    let b = channel(hex, 5);
    format!("rgba({r}, {g}, {b}, {alpha})")
}

#[must_use]
pub fn glow(hex: &str) -> String {
    let r = channel(hex, 1);
    let g = channel(hex, 3);
    let b = channel(hex, 5);
    format!("0 2px 8px rgba({r}, {g}, {b}, 0.25)")
}

#[must_use]
pub fn grid() -> String {
    columns(-3)
}

#[must_use]
pub fn narrow() -> String {
    columns(-4)
}

fn columns(power: i32) -> String {
    let side = proportion::scale(power);
    format!("{side}fr 1fr {side}fr")
}
