use alignment::Alignment;
use cursor::Cursor;
use decoration::Decoration;
use display::Display;
use overflow::Overflow;
use position::Position;
use selector::{Pseudo, Selector, Tag, data, present, tag};
use space::Space;
use style::Style;
use transform::Transform;
use value::{Concrete, Keyword, Palette, Token};
use weight::Weight;

fn status(style: Style, reference: class::reference::Reference, hex: &str) -> Style {
    style
        .rule(Selector::from(reference), |r| {
            r.background(utility::tint(hex, 0.15)).color(hex)
        })
        .rule(Selector::from(reference).pseudo(Pseudo::Hover), |r| {
            r.transform("translateY(-1px)")
                .box_shadow(utility::glow(hex))
                .background(utility::tint(hex, 0.25))
        })
}

#[must_use]
pub fn dashboard() -> Style {
    let style = Style::new()
        .rule(class::dashboard::frame(), |r| {
            r.display(Display::Grid)
                .grid_template_columns(utility::narrow())
                .gap((Concrete::zero(), Token::scale(1)))
                .max_width(Concrete::px(1600))
                .margin((Concrete::zero(), Keyword::Auto))
                .padding((Concrete::zero(), Token::scale(1)))
        })
        .rule(
            Selector::from(class::dashboard::frame()).child(tag(Tag::Main)),
            |r| r.min_width(Concrete::zero()).max_width(Concrete::px(1200)),
        )
        .rule(
            Selector::from(class::dashboard::frame()).child(tag(Tag::Footer)),
            |r| r.custom("grid-column", "1 / -1"),
        )
        .rule(class::dashboard::filter(), |r| {
            r.position(Position::Relative)
                .margin_bottom(Token::scale(1))
        })
        .rule(class::dashboard::search(), |r| {
            r.width(Concrete::percent(100.0))
                .padding((Token::scale(-1), Token::scale(0)))
                .font_size(Token::scale(0))
                .font_family("inherit")
                .background(Token::palette(Palette::Code))
                .color(Token::palette(Palette::Text))
                .border("1px solid var(--border)")
                .border_radius(Concrete::px(6))
                .transition("border-color 0.2s ease, background-color 0.3s ease")
        })
        .rule(
            Selector::from(class::dashboard::search()).pseudo(Pseudo::Focus),
            |r| r.border_color("var(--accent)").outline(Keyword::None),
        )
        .rule(class::dashboard::counter(), |r| {
            r.position(Position::Absolute)
                .right(Token::scale(0))
                .top(Concrete::percent(50.0))
                .transform("translateY(-50%)")
                .font_size(Token::scale(-1))
                .color(Token::palette(Palette::Secondary))
                .pointer_events(Keyword::None)
        })
        .rule(class::dashboard::empty(), |r| {
            r.text_align(align::Align::Center)
                .padding((Token::scale(2), Token::scale(0)))
                .color(Token::palette(Palette::Secondary))
                .font_size(Token::scale(0))
        })
        .rule(class::dashboard::grid(), |r| {
            r.display(Display::Grid)
                .grid_template_columns("1fr")
                .gap(Token::scale(0))
        })
        .rule(class::dashboard::card(), |r| {
            r.background(Token::palette(Palette::Code))
                .border_left("3px solid var(--border)")
                .border_radius(Concrete::px(6))
                .overflow(Overflow::Hidden)
                .transition(
                    "background-color var(--duration-fast) var(--ease-out), \
                     border-left-color var(--duration-fast) var(--ease-out), \
                     box-shadow var(--duration-fast) var(--ease-out), \
                     opacity var(--duration-fast) var(--ease-out), \
                     transform var(--duration-micro) var(--ease-out)",
                )
        })
        .rule(data(attribute::search::hidden()), |r| {
            r.display(Display::None)
        })
        .rule(
            Selector::from(class::dashboard::card()).and(data(attribute::paginate::paged())),
            |r| r.display(Display::None),
        )
        .rule(
            Selector::from(class::dashboard::card()).data(attribute::status::status(), "pass"),
            |r| r.border_left(format!("3px solid {}", palette::PASS)),
        )
        .rule(
            Selector::from(class::dashboard::card()).data(attribute::status::status(), "fail"),
            |r| r.border_left(format!("3px solid {}", palette::FAIL)),
        )
        .rule(
            Selector::from(class::dashboard::card()).pseudo(Pseudo::Hover),
            |r| {
                r.background(Token::palette(Palette::Stripe))
                    .box_shadow("0 2px 8px rgba(0, 0, 0, 0.06)")
                    .transform("translateY(-1px)")
            },
        )
        .rule(
            Selector::from(class::dashboard::card()).descendant(tag(Tag::Summary)),
            |r| {
                r.display(Display::Flex)
                    .align_items(Alignment::Center)
                    .gap(Token::scale(-2))
                    .cursor(Cursor::Pointer)
                    .font_weight(Weight::W500)
                    .padding(Token::scale(0))
            },
        )
        .rule(class::dashboard::badge(), |r| {
            r.display(Display::Inline)
                .padding((Concrete::px(1), Concrete::px(6)))
                .border_radius(Concrete::px(8))
                .font_size(Token::scale(-1))
                .font_weight(Weight::W500)
                .white_space(Space::Nowrap)
                .transition(
                    "transform var(--duration-micro) var(--ease-out), \
                     box-shadow var(--duration-micro) var(--ease-out), \
                     background-color var(--duration-micro) var(--ease-out)",
                )
        });
    let style = status(style, class::dashboard::pass(), palette::PASS);
    let style = status(style, class::dashboard::fail(), palette::FAIL);
    let style = status(style, class::dashboard::panic(), palette::PANIC);
    style
        .rule(class::dashboard::divider(), |r| {
            r.font_size(Token::scale(-1))
                .font_weight(Weight::W600)
                .text_transform(Transform::Uppercase)
                .letter_spacing(Concrete::em(0.05))
                .color(Token::palette(Palette::Secondary))
                .padding((Token::scale(-1), Concrete::zero()))
        })
        .rule(
            Selector::from(class::dashboard::card())
                .descendant(tag(Tag::Summary))
                .pseudo(Pseudo::FocusVisible),
            |r| {
                r.outline("2px solid var(--accent)")
                    .outline_offset(Concrete::px(2))
                    .border_radius(Concrete::px(4))
            },
        )
        .rule(
            Selector::from(class::dashboard::search()).pseudo(Pseudo::FocusVisible),
            |r| {
                r.outline("2px solid var(--accent)")
                    .outline_offset(Concrete::px(2))
                    .box_shadow("0 0 0 3px rgba(212, 93, 0, 0.15)")
            },
        )
        .rule(class::dashboard::margin(), |r| {
            r.position(Position::Sticky)
                .custom(
                    "top",
                    &format!(
                        "calc({} + {} + {})",
                        Token::scale(3),
                        Token::scale(-2),
                        Token::scale(2)
                    ),
                )
                .custom("align-self", "start")
                .display(Display::Flex)
                .custom("flex-direction", "column")
                .gap(Token::scale(-2))
                .padding((Token::scale(-1), Token::scale(0)))
                .border_left("3px solid var(--border)")
                .min_width(Concrete::px(120))
        })
        .rule(
            Selector::from(class::dashboard::margin()).and(class::dashboard::pass().into()),
            |r| r.border_left_color(palette::PASS),
        )
        .rule(
            Selector::from(class::dashboard::margin()).and(class::dashboard::fail().into()),
            |r| r.border_left_color(palette::FAIL),
        )
        .rule(
            Selector::from(class::dashboard::margin())
                .descendant(class::dashboard::metric().into()),
            |r| {
                r.background("transparent")
                    .padding(Concrete::zero())
                    .border_radius(Concrete::zero())
                    .font_size(Token::scale(-1))
                    .color(Token::palette(Palette::Secondary))
            },
        )
        .rule(
            Selector::from(class::dashboard::margin())
                .descendant(class::dashboard::metric().into())
                .descendant(tag(Tag::Span)),
            |r| {
                r.display(Display::Block)
                    .font_size(Token::scale(2))
                    .font_weight(Weight::W700)
                    .custom("line-height", "1.1")
            },
        )
        .rule(
            Selector::from(class::dashboard::margin())
                .descendant(class::dashboard::pass().into())
                .descendant(tag(Tag::Span)),
            |r| r.color(palette::PASS),
        )
        .rule(
            Selector::from(class::dashboard::margin())
                .descendant(class::dashboard::fail().into())
                .descendant(tag(Tag::Span)),
            |r| r.color(palette::FAIL),
        )
        .rule(
            Selector::from(class::dashboard::margin())
                .descendant(class::dashboard::divider().into()),
            |r| r.padding(Concrete::zero()).margin_bottom(Token::scale(-3)),
        )
        .rule(class::dashboard::metric(), |r| {
            r.font_size(Token::scale(-1))
                .font_weight(Weight::W500)
                .padding((Concrete::px(4), Concrete::px(10)))
                .border_radius(Concrete::px(8))
        })
        .rule(
            Selector::from(class::dashboard::metric()).descendant(tag(Tag::Span)),
            |r| r.font_size(Token::scale(0)).font_weight(Weight::W600),
        )
        .rule(class::dashboard::bar(), |r| {
            r.display(Display::Flex)
                .height(Concrete::px(3))
                .border_radius("6px 6px 0 0")
                .overflow(Overflow::Hidden)
        })
        .rule(class::dashboard::segment(), |r| {
            r.transition("width var(--duration-fast) var(--ease-out)")
        })
        .rule(
            Selector::from(class::dashboard::segment()).and(class::dashboard::pass().into()),
            |r| r.background(palette::PASS),
        )
        .rule(
            Selector::from(class::dashboard::segment()).and(class::dashboard::fail().into()),
            |r| r.background(palette::FAIL),
        )
        .rule(class::dashboard::tags(), |r| {
            r.display(Display::Flex)
                .gap(Concrete::px(4))
                .flex_wrap(wrap::Wrap::Wrap)
                .margin_left(Keyword::Auto)
        })
        .rule(class::dashboard::tag(), |r| {
            r.padding((Concrete::px(2), Concrete::px(8)))
                .border_radius(Concrete::px(6))
                .font_size(Token::scale(0))
                .background(Token::palette(Palette::Stripe))
                .color(Token::palette(Palette::Secondary))
        })
        .rule(class::dashboard::detail(), |r| {
            r.padding((Token::scale(-2), Token::scale(-1)))
                .border_left("2px solid var(--border)")
                .margin_left(Token::scale(-1))
                .display(Display::Flex)
                .align_items(Alignment::Center)
                .gap(Token::scale(-2))
                .flex_wrap(wrap::Wrap::Wrap)
        })
        .rule(
            Selector::from(class::dashboard::detail()).child(tag(Tag::Div)),
            |r| r.width(Concrete::percent(100.0)),
        )
        .rule(
            Selector::from(class::dashboard::detail()).descendant(tag(Tag::Pre)),
            |r| {
                r.font_size(Token::scale(-1))
                    .margin_bottom(Token::scale(-2))
            },
        )
        .rule(class::dashboard::expected(), |r| {
            r.background(utility::tint(palette::FAIL, 0.15))
                .text_decoration(Decoration::LineThrough)
                .border_radius(Concrete::px(2))
                .padding((Concrete::zero(), Concrete::px(2)))
                .cursor(Cursor::Pointer)
        })
        .rule(class::dashboard::actual(), |r| {
            r.background(utility::tint(palette::PASS, 0.15))
                .border_radius(Concrete::px(2))
                .padding((Concrete::zero(), Concrete::px(2)))
                .cursor(Cursor::Pointer)
        })
        .rule(data(attribute::difference::hidden()), |r| {
            r.display(Display::None)
        })
        .rule(
            Selector::from(class::dashboard::card()).descendant(tag(Tag::Details)),
            |r| r.custom("interpolate-size", "allow-keywords"),
        )
        .rule(class::dashboard::content(), |r| {
            r.overflow(Overflow::Hidden)
                .opacity(0.0)
                .height(Concrete::zero())
                .padding((Concrete::zero(), Token::scale(0)))
                .transition(
                    "height var(--duration-base) var(--ease-out), \
                     opacity var(--duration-fast) var(--ease-out)",
                )
        })
        .rule(
            tag(Tag::Details)
                .and(present("open"))
                .child(class::dashboard::content().into()),
            |r| r.opacity(1.0).height(Keyword::Auto),
        )
        .rule(tag(Tag::Body).and(data(attribute::expand::lock())), |r| {
            r.overflow_y(Overflow::Hidden)
        })
        .rule(class::dashboard::backdrop(), |r| {
            r.position(Position::Fixed)
                .custom("inset", "0")
                .z_index(240)
                .background("rgba(0, 0, 0, 0)")
                .custom("pointer-events", "none")
                .transition(
                    "background-color var(--duration-base) var(--ease-out), \
                     backdrop-filter var(--duration-base) var(--ease-out)",
                )
        })
        .rule(
            Selector::from(class::dashboard::backdrop()).and(data(attribute::expand::expanded())),
            |r| {
                r.background("rgba(0, 0, 0, 0.4)")
                    .custom("backdrop-filter", "blur(8px)")
                    .custom("pointer-events", "auto")
            },
        )
        .rule(
            Selector::from(class::dashboard::card()).and(data(attribute::expand::expanded())),
            |r| {
                r.position(Position::Fixed)
                    .z_index(250)
                    .custom("inset", "0")
                    .background(Token::palette(Palette::Background))
                    .border_radius(Concrete::zero())
                    .border_left("4px solid var(--border)")
                    .box_shadow("none")
                    .overflow_y(Overflow::Auto)
                    .custom(
                        "animation",
                        "expand var(--duration-base) var(--ease-out) both",
                    )
            },
        )
        .rule(
            Selector::from(class::dashboard::card())
                .and(data(attribute::expand::expanded()))
                .data(attribute::status::status(), "pass"),
            |r| r.border_left(format!("4px solid {}", palette::PASS)),
        )
        .rule(
            Selector::from(class::dashboard::card())
                .and(data(attribute::expand::expanded()))
                .data(attribute::status::status(), "fail"),
            |r| r.border_left(format!("4px solid {}", palette::FAIL)),
        )
        .rule(
            Selector::from(class::dashboard::card())
                .and(data(attribute::expand::expanded()))
                .pseudo(Pseudo::Hover),
            |r| r.transform("none").box_shadow("none"),
        )
        .rule(
            Selector::from(class::dashboard::card())
                .and(data(attribute::expand::expanded()))
                .descendant(tag(Tag::Summary)),
            |r| {
                r.font_size(Token::scale(1))
                    .padding(Token::scale(1))
                    .border_bottom("1px solid var(--border)")
            },
        )
        .rule(
            Selector::from(class::dashboard::card())
                .and(data(attribute::expand::expanded()))
                .descendant(class::dashboard::content().into()),
            |r| {
                r.opacity(1.0)
                    .height(Keyword::Auto)
                    .padding(Token::scale(1))
            },
        )
        .rule(
            Selector::from(class::dashboard::card())
                .and(data(attribute::expand::expanded()))
                .descendant(class::dashboard::detail().into()),
            |r| r.padding(Token::scale(0)).gap(Token::scale(0)),
        )
        .keyframe("expand", |k| {
            k.step("from", |p| {
                p.opacity(0.0).transform("scale(0.97) translateY(8px)")
            })
            .step("to", |p| p.opacity(1.0).transform("scale(1) translateY(0)"))
        })
}
