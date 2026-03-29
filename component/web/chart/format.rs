#[must_use]
pub fn notation(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    let magnitude = value.abs().log10().floor();
    if (-2.0..=4.0).contains(&magnitude) {
        #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let precision = (2.0 - magnitude).clamp(0.0, 4.0) as usize;
        format!("{value:.precision$}")
    } else {
        let power = 10_f64.powf(magnitude);
        let mantissa = value / power;
        format!("{mantissa:.1}e{magnitude:.0}")
    }
}

pub struct Duration {
    pub value: String,
    pub unit: &'static str,
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

#[must_use]
pub fn duration(seconds: f64) -> Duration {
    if seconds <= 0.0 {
        return Duration {
            value: "0".to_string(),
            unit: "s",
        };
    }
    if seconds < 1e-6 {
        Duration {
            value: format!("{:.1}", seconds * 1e9),
            unit: "ns",
        }
    } else if seconds < 1e-3 {
        Duration {
            value: format!("{:.1}", seconds * 1e6),
            unit: "\u{00b5}s",
        }
    } else if seconds < 1.0 {
        Duration {
            value: format!("{:.2}", seconds * 1e3),
            unit: "ms",
        }
    } else {
        Duration {
            value: format!("{seconds:.3}"),
            unit: "s",
        }
    }
}
