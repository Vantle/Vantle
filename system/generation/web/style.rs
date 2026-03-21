use observe::trace;
use style::Style;

pub use base;
pub use code;
pub use dashboard;
pub use layout;
pub use navigation;
pub use utility;

#[trace(channels = [document])]
fn variables(style: Style) -> Style {
    let style = palette::PALETTE
        .iter()
        .chain(palette::SYNTAX)
        .fold(style, |s, token| {
            s.variable(&format!("--{}", token.role), token.light)
        });
    style
        .variable("--phi", &proportion::PHI.to_string())
        .variable("--scale-n2", &utility::scale(-2))
        .variable("--scale-n1", &utility::scale(-1))
        .variable("--scale-n0h", &utility::half(0))
        .variable("--scale-0", &utility::scale(0))
        .variable("--scale-1", &utility::scale(1))
        .variable("--scale-2", &utility::scale(2))
        .variable("--scale-3", &utility::scale(3))
        .variable("--duration-micro", &format!("{}s", proportion::scale(-5)))
        .variable("--duration-fast", &format!("{}s", proportion::scale(-4)))
        .variable("--duration-base", &format!("{}s", proportion::scale(-3)))
        .variable("--ease-out", "cubic-bezier(0.22, 1, 0.36, 1)")
}

#[trace(channels = [document])]
#[must_use]
pub fn theme() -> Style {
    variables(Style::new())
        .extend(base::foundation())
        .extend(base::typography())
        .extend(code::highlighting())
        .extend(navigation::navigation())
        .extend(layout::layout())
        .extend(base::content())
        .extend(base::footer())
        .extend(code::toolbar())
        .extend(dashboard::dashboard())
        .extend(layout::animation())
        .extend(layout::responsive())
}
