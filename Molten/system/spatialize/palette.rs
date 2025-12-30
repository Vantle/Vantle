use color::Color;
use proportion::PHI;

const fn scale(base: f32, k: i32) -> f32 {
    if k == 0 {
        base
    } else if k > 0 {
        let mut result = base;
        let mut i = 0;
        while i < k {
            result *= PHI;
            i += 1;
        }
        result
    } else {
        let mut result = base;
        let mut i = 0;
        while i < -k {
            result /= PHI;
            i += 1;
        }
        result
    }
}

const BASE: f32 = 0.055;

pub const BACKGROUND: Color = Color {
    r: scale(BASE, -2),
    g: scale(BASE, -2),
    b: scale(BASE, -1) * 0.7,
    a: 1.0,
};

pub const NODE: Color = Color {
    r: scale(BASE, 2),
    g: scale(BASE, 2),
    b: scale(BASE, 3) * 0.85,
    a: 1.0,
};

pub const EDGE: Color = Color {
    r: scale(BASE, 3),
    g: scale(BASE, 5),
    b: scale(BASE, 6),
    a: 0.85,
};

pub const HIGHLIGHTED: Color = Color {
    r: scale(BASE, 6),
    g: scale(BASE, 5),
    b: scale(BASE, 2),
    a: 1.0,
};

pub const TOOLTIP: Color = Color {
    r: scale(BASE, 1),
    g: scale(BASE, 1),
    b: scale(BASE, 2) * 0.9,
    a: 0.95,
};

pub const TEXT: Color = Color {
    r: scale(BASE, 6),
    g: scale(BASE, 6),
    b: scale(BASE, 6),
    a: 1.0,
};
