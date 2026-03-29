use collapse::Collapse;
use overflow::Overflow;
use selector::{Parity, Pseudo, Selector, Tag, tag};
use style::Style;
use transform::Transform;
use value::{Concrete, Keyword, Palette, Token};
use weight::Weight;

const PHI_EASE: &str = "cubic-bezier(0.382, 0, 0.618, 1)";

#[must_use]
pub fn chart() -> Style {
    Style::new()
        .extend(canvas())
        .extend(axis())
        .extend(scatter())
        .extend(regression())
        .extend(table())
        .extend(overlay())
        .extend(animation())
}

fn canvas() -> Style {
    Style::new()
        .rule(
            Selector::from(class::chart::chart()).descendant(tag(Tag::Svg)),
            |r| {
                r.width(Concrete::percent(100.0))
                    .height(Keyword::Auto)
                    .display(display::Display::Block)
                    .overflow(Overflow::Visible)
            },
        )
        .rule(
            Selector::from(class::chart::chart()).and(tag(Tag::Div)),
            |r| {
                r.width(Concrete::percent(100.0))
                    .overflow(Overflow::Visible)
            },
        )
}

fn axis() -> Style {
    Style::new()
        .rule(class::chart::tick(), |r| {
            tabular(r)
                .font_size(Concrete::px(11))
                .fill(Token::palette(Palette::Secondary))
        })
        .rule(class::chart::label(), |r| {
            r.font_size(Concrete::px(11))
                .font_family("inherit")
                .font_weight(Weight::W400)
                .letter_spacing(Concrete::em(0.04))
                .text_transform(Transform::Uppercase)
                .fill(Token::palette(Palette::Secondary))
        })
        .rule(class::chart::grid(), |r| {
            r.stroke(Token::palette(Palette::Border))
                .stroke_width("0.3")
        })
}

fn scatter() -> Style {
    Style::new()
        .rule(
            Selector::from(class::chart::point()).descendant(Selector::Class("visible".into())),
            |r| {
                r.fill(Token::palette(Palette::Accent))
                    .stroke(Keyword::None)
                    .stroke_width("0")
                    .cursor(cursor::Cursor::Pointer)
                    .transition(format!(
                        "transform 200ms ease-out, fill 618ms {PHI_EASE}, stroke 618ms {PHI_EASE}, stroke-width 618ms {PHI_EASE}"
                    ))
            },
        )
        .rule(
            Selector::from(class::chart::point()).descendant(
                Selector::Class("visible".into()).and(Selector::from(class::chart::excluded())),
            ),
            |r| {
                r.fill(Keyword::None)
                    .stroke(Token::palette(Palette::Accent))
                    .stroke_width("1.5")
            },
        )
}

fn regression() -> Style {
    Style::new().rule(class::chart::winner(), |r| {
        r.fill(Keyword::None)
            .stroke(Token::palette(Palette::Text))
            .stroke_width("2")
            .opacity(0.8)
            .stroke_linecap("round")
            .stroke_linejoin("round")
    })
}

fn tabular(properties: style::Properties) -> style::Properties {
    properties
        .font_family("inherit")
        .font_variant_numeric("tabular-nums")
}

fn overlay() -> Style {
    Style::new()
        .rule(class::chart::heading(), |r| {
            tabular(r)
                .font_size(Concrete::px(11))
                .font_weight(Weight::W600)
                .fill(Token::palette(Palette::Accent))
        })
        .rule(class::chart::body(), |r| {
            tabular(r)
                .font_size(Concrete::px(9))
                .fill(Token::palette(Palette::Text))
        })
        .rule(class::chart::dim(), |r| {
            tabular(r)
                .font_size(Concrete::px(8))
                .fill(Token::palette(Palette::Secondary))
                .transition(format!("opacity 200ms {PHI_EASE}"))
        })
        .rule(class::chart::overlay(), |r| {
            r.transition("opacity 80ms ease-out")
        })
        .rule(class::chart::fade(), |r| {
            r.transition("opacity 200ms ease-out")
        })
        .rule(class::chart::backing(), |r| {
            r.opacity(0.618)
                .transition(format!("width 200ms {PHI_EASE}, height 200ms {PHI_EASE}"))
        })
        .rule(class::chart::emerge(), |r| {
            r.animation("emerge 0.4s ease-out both")
        })
}

fn table() -> Style {
    Style::new()
        .rule(
            Selector::from(class::chart::chart()).descendant(tag(Tag::Table)),
            |r| {
                r.width(Concrete::percent(100.0))
                    .border_collapse(Collapse::Collapse)
                    .font_size(Concrete::rem(0.8))
            },
        )
        .rule(
            Selector::from(class::chart::chart()).descendant(tag(Tag::Th)),
            |r| {
                r.color(Token::palette(Palette::Secondary))
                    .font_size(Concrete::rem(0.75))
                    .letter_spacing(Concrete::em(0.04))
                    .text_transform(Transform::Uppercase)
                    .font_weight(Weight::W400)
                    .border_bottom(format!("1px solid {}", Token::palette(Palette::Border)))
            },
        )
        .rule(
            Selector::from(class::chart::chart())
                .descendant(tag(Tag::Tr).pseudo(Pseudo::NthChild(Parity::Odd))),
            |r| r.background(Token::palette(Palette::Stripe)),
        )
        .rule(
            selector::group(vec![
                Selector::from(class::chart::chart()).descendant(tag(Tag::Td)),
                Selector::from(class::chart::chart()).descendant(tag(Tag::Th)),
            ]),
            |r| {
                r.padding((Concrete::rem(0.4), Concrete::rem(0.6)))
                    .font_variant_numeric("tabular-nums")
            },
        )
}

fn animation() -> Style {
    Style::new().keyframe("emerge", |k| {
        k.step("from", |r| r.transform("scale(0)").opacity(0.0))
    })
}
