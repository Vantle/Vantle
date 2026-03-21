use alignment::Alignment;
use cursor::Cursor;
use decoration::Decoration;
use direction::Direction;
use display::Display;
use observe::trace;
use overflow::Overflow;
use position::Position;
use selector::{Pseudo, Selector, Tag, data, tag};
use style::Style;
use transform::Transform;
use value::{Calculation, Concrete, Keyword, Palette, Token};
use weight::Weight;

#[trace(channels = [document])]
fn dark(properties: style::Properties) -> style::Properties {
    palette::PALETTE
        .iter()
        .chain(palette::SYNTAX)
        .fold(properties, |p, token| {
            p.custom(&format!("--{}", token.role), token.dark)
        })
}

#[must_use]
pub fn layout() -> Style {
    Style::new()
        .rule(class::reference::layout(), |r| {
            r.display(Display::Grid)
                .grid_template_columns(utility::grid())
                .min_height("calc(100vh - calc(var(--scale-3) + var(--scale-n2)))")
        })
        .rule(class::reference::sidebar(), |r| {
            r.position(Position::Sticky)
                .top(Calculation::start(Token::scale(3)).plus(Token::scale(-2)))
                .height("calc(100vh - calc(var(--scale-3) + var(--scale-n2)))")
                .overflow_y(Overflow::Auto)
                .padding((Token::scale(1), Token::scale(0)))
                .border_right("1px solid var(--border)")
        })
        .rule(
            Selector::from(class::reference::sidebar()).descendant(tag(Tag::A)),
            |r| {
                r.display(Display::Block)
                    .padding((Token::scale(-2), Token::scale(-1)))
                    .color(Token::palette(Palette::Secondary))
                    .font_size(Token::half(0))
                    .border_radius(Concrete::px(4))
            },
        )
        .rule(
            Selector::from(class::reference::sidebar())
                .descendant(tag(Tag::A).pseudo(Pseudo::Hover)),
            |r| {
                r.color(Token::palette(Palette::Text))
                    .background(Token::palette(Palette::Code))
                    .text_decoration(Decoration::None)
            },
        )
        .rule(
            Selector::from(class::reference::sidebar())
                .descendant(class::navigation::nested().into()),
            |r| {
                r.padding_left(Token::scale(1))
                    .font_size(Token::half(0))
                    .border_left("2px solid var(--border)")
                    .opacity(0.85)
            },
        )
        .rule(
            Selector::from(class::reference::sidebar())
                .descendant(tag(Tag::A).attribute("aria-current", "page")),
            |r| {
                r.color(Token::palette(Palette::Accent))
                    .background(Token::palette(Palette::Code))
            },
        )
        .rule(class::reference::outline(), |r| {
            r.position(Position::Sticky)
                .top(Calculation::start(Token::scale(3)).plus(Token::scale(-2)))
                .height("calc(100vh - calc(var(--scale-3) + var(--scale-n2)))")
                .overflow_y(Overflow::Auto)
                .padding((Token::scale(1), Token::scale(0)))
                .border_left("1px solid var(--border)")
        })
        .rule(
            Selector::from(class::reference::outline()).descendant(class::label::label().into()),
            |r| {
                r.font_size(Token::half(0))
                    .font_weight(Weight::W500)
                    .text_transform(Transform::Uppercase)
                    .letter_spacing(Concrete::em(0.08))
                    .color(Token::palette(Palette::Secondary))
                    .margin_bottom(Token::scale(-1))
            },
        )
        .rule(
            Selector::from(class::reference::outline()).descendant(tag(Tag::A)),
            |r| {
                r.display(Display::Block)
                    .padding((Token::scale(-2), Token::scale(-1)))
                    .color(Token::palette(Palette::Secondary))
                    .font_size(Token::half(0))
                    .border_left("2px solid transparent")
                    .transition("color 0.2s, border-left-color 0.2s")
            },
        )
        .rule(
            Selector::from(class::reference::outline())
                .descendant(tag(Tag::A).pseudo(Pseudo::Hover)),
            |r| {
                r.color(Token::palette(Palette::Text))
                    .text_decoration(Decoration::None)
            },
        )
        .rule(
            Selector::from(class::reference::outline())
                .descendant(tag(Tag::A).data(attribute::render::depth(), "0")),
            |r| {
                r.font_weight(Weight::W500)
                    .text_transform(Transform::Uppercase)
                    .letter_spacing(Concrete::em(0.08))
            },
        )
        .rule(
            Selector::from(class::reference::outline())
                .descendant(tag(Tag::A).pseudo(Pseudo::FirstChild)),
            |r| r.padding_top(Concrete::zero()),
        )
        .rule(
            Selector::from(class::reference::outline())
                .descendant(tag(Tag::A).data(attribute::render::depth(), "1")),
            |r| r.padding_left(Token::scale(0)),
        )
        .rule(
            Selector::from(class::reference::outline())
                .descendant(tag(Tag::A).data(attribute::render::depth(), "2")),
            |r| r.padding_left(Token::scale(1)),
        )
        .rule(
            Selector::from(class::reference::outline())
                .descendant(tag(Tag::A).data(attribute::render::depth(), "3")),
            |r| r.padding_left(Token::scale(2)),
        )
        .rule(
            Selector::from(class::reference::outline())
                .descendant(tag(Tag::A).data(attribute::render::depth(), "4")),
            |r| r.padding_left(Token::scale(3)),
        )
        .rule(
            Selector::from(class::reference::outline())
                .descendant(tag(Tag::A).and(class::reference::active().into())),
            |r| {
                r.color(Token::palette(Palette::Accent))
                    .border_left_color(Token::palette(Palette::Accent))
            },
        )
        .rule(
            Selector::from(class::reference::sidebar()).descendant(class::label::label().into()),
            |r| {
                r.font_size(Token::half(0))
                    .font_weight(Weight::W500)
                    .text_transform(Transform::Uppercase)
                    .letter_spacing(Concrete::em(0.08))
                    .color(Token::palette(Palette::Secondary))
                    .margin_top(Token::scale(1))
                    .margin_bottom(Token::scale(-1))
            },
        )
        .rule(
            Selector::from(class::reference::sidebar())
                .descendant(Selector::from(class::label::label()).pseudo(Pseudo::FirstChild)),
            |r| r.margin_top(Concrete::zero()),
        )
        .rule(class::reference::hamburger(), |r| {
            r.display(Display::None)
                .background(Keyword::Transparent)
                .border(Keyword::None)
                .cursor(Cursor::Pointer)
                .padding(Concrete::zero())
                .align_items(Alignment::Center)
                .justify_content(Alignment::Center)
                .align_self(Alignment::Center)
                .height(Token::scale(2))
                .width(Token::scale(2))
                .flex_direction(Direction::Column)
                .gap(Concrete::px(3))
        })
        .rule(
            Selector::from(class::reference::hamburger()).descendant(tag(Tag::Span)),
            |r| {
                r.display(Display::Block)
                    .width(Concrete::px(16))
                    .height(Concrete::px(2))
                    .background(Token::palette(Palette::Secondary))
                    .border_radius(Concrete::px(1))
                    .transition("background-color 0.3s ease")
            },
        )
        .rule(
            Selector::from(class::reference::hamburger())
                .pseudo(Pseudo::Hover)
                .descendant(tag(Tag::Span)),
            |r| r.background(Token::palette(Palette::Text)),
        )
}

#[must_use]
pub fn animation() -> Style {
    Style::new()
        .rule(
            Selector::from(class::reference::enhanced())
                .descendant(data(attribute::scroll::animate())),
            |r| {
                r.opacity(0.0)
                    .transform("translateY(12px)")
                    .transition(
                        "opacity var(--duration-base) var(--ease-out), \
                         transform var(--duration-base) var(--ease-out)",
                    )
                    .custom(
                        "transition-delay",
                        "calc(var(--index, 0) * var(--duration-micro))",
                    )
            },
        )
        .rule(
            Selector::from(class::reference::enhanced())
                .descendant(data(attribute::scroll::visible())),
            |r| r.opacity(1.0).transform("translateY(0)"),
        )
}

#[must_use]
pub fn responsive() -> Style {
    Style::new()
        .rule(tag(Tag::Html).data(attribute::theme::theme(), "dark"), dark)
        .rule(
            tag(Tag::Html)
                .data(attribute::theme::theme(), "dark")
                .descendant(class::navigation::menu().into()),
            |r| r.box_shadow("0 4px 12px #00000066"),
        )
        .rule(
            tag(Tag::Html).data(attribute::theme::theme(), "light"),
            |r| {
                palette::PALETTE
                    .iter()
                    .chain(palette::SYNTAX)
                    .fold(r, |p, token| {
                        p.custom(&format!("--{}", token.role), token.light)
                    })
            },
        )
        .media("prefers-color-scheme: dark", |m| {
            palette::PALETTE
                .iter()
                .chain(palette::SYNTAX)
                .fold(m, |s, token| {
                    s.variable(&format!("--{}", token.role), token.dark)
                })
                .rule(class::navigation::menu(), |r| {
                    r.box_shadow("0 4px 12px #00000066")
                })
        })
        .media("max-width: 1280px", |m| {
            m.rule(class::reference::outline(), |r| r.display(Display::None))
                .rule(class::reference::layout(), |r| {
                    r.grid_template_columns(format!("{}fr 1fr", proportion::scale(-3)))
                })
        })
        .media("max-width: 1024px", |m| {
            m.rule(class::reference::sidebar(), |r| {
                r.display(Display::None)
                    .position(Position::Fixed)
                    .top(Calculation::start(Token::scale(3)).plus(Token::scale(-2)))
                    .left(Concrete::zero())
                    .width(Concrete::px(280))
                    .height("calc(100vh - calc(var(--scale-3) + var(--scale-n2)))")
                    .background(Token::palette(Palette::Background))
                    .z_index(150)
                    .border_right("1px solid var(--border)")
            })
            .rule(
                Selector::from(class::reference::sidebar()).and(class::reference::open().into()),
                |r| r.display(Display::Block),
            )
            .rule(class::reference::layout(), |r| {
                r.grid_template_columns("1fr")
            })
            .rule(class::reference::hamburger(), |r| r.display(Display::Flex))
        })
        .media("max-width: 768px", |m| {
            m.rule(tag(Tag::Main), |r| {
                r.padding((Token::scale(0), Token::scale(-1)))
            })
            .rule(tag(Tag::H1), |r| {
                r.font_size(Token::scale(2))
                    .letter_spacing(Concrete::em(-0.02))
            })
            .rule(tag(Tag::Nav), |r| {
                r.padding((Concrete::zero(), Token::scale(-1)))
            })
        })
}
