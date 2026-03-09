use collapse::Collapse;
use cursor::Cursor;
use decoration::Decoration;
use direction::Direction;
use display::Display;
use observe::trace;
use overflow::Overflow;
use position::Position;
use selector::{Pseudo, Selector, Tag, group, present, tag, universal};
use sizing::Box as Sizing;
use space::Space;
use style::Style;
use transform::Transform;
use value::{Calculation, Concrete, Keyword, Palette, Token};
use weight::Weight;

#[trace(channels = [document])]
fn scale(k: i32) -> String {
    format!("{}rem", proportion::scale(k))
}

fn half(k: i32) -> String {
    format!("{}rem", proportion::half(k))
}

fn grid() -> String {
    let side = proportion::scale(-3);
    format!("{side}fr 1fr {side}fr")
}

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
        .variable("--scale-n2", &scale(-2))
        .variable("--scale-n1", &scale(-1))
        .variable("--scale-n0h", &half(0))
        .variable("--scale-0", &scale(0))
        .variable("--scale-1", &scale(1))
        .variable("--scale-2", &scale(2))
        .variable("--scale-3", &scale(3))
}

#[trace(channels = [document])]
fn dark(properties: style::Properties) -> style::Properties {
    palette::PALETTE
        .iter()
        .chain(palette::SYNTAX)
        .fold(properties, |p, token| {
            p.custom(&format!("--{}", token.role), token.dark)
        })
}

#[trace(channels = [document])]
#[must_use]
pub fn theme() -> Style {
    variables(Style::new())
        .rule(tag(Tag::Html), |r| {
            r.scroll_padding_top(
                Calculation::start(Token::scale(3))
                    .plus(Token::scale(-2))
                    .plus(Token::scale(-2)),
            )
        })
        .rule(universal(), |r| {
            r.margin(Concrete::zero())
                .padding(Concrete::zero())
                .box_sizing(Sizing::Border)
        })
        .rule(tag(Tag::Body), |r| {
            r.font_family("-apple-system, BlinkMacSystemFont, 'Segoe UI', system-ui, sans-serif")
                .background(Token::palette(Palette::Background))
                .color(Token::palette(Palette::Text))
                .line_height(proportion::PHI.to_string())
                .font_size(Token::scale(0))
                .transition("background-color 0.3s ease, color 0.3s ease")
        })
        .rule(tag(Tag::Main), |r| {
            r.padding((Token::scale(2), Token::scale(1)))
                .min_width(Concrete::zero())
        })
        .rule(tag(Tag::H1), |r| {
            r.font_size(Token::scale(3))
                .font_weight(Weight::W700)
                .margin_bottom(Token::scale(-1))
                .line_height("1.1")
                .letter_spacing("-0.03em")
        })
        .rule(tag(Tag::H2), |r| {
            r.font_size(Token::scale(2))
                .font_weight(Weight::W600)
                .margin_top(Token::scale(2))
                .margin_bottom(Token::scale(0))
                .letter_spacing("-0.02em")
        })
        .rule(tag(Tag::H3), |r| {
            r.font_size(Token::scale(1))
                .font_weight(Weight::W600)
                .margin_top(Token::scale(1))
                .margin_bottom(Token::scale(-1))
                .letter_spacing("-0.01em")
        })
        .rule(tag(Tag::H4), |r| {
            r.font_size(Token::scale(0))
                .font_weight(Weight::W600)
                .letter_spacing("-0.01em")
        })
        .rule(tag(Tag::H5), |r| {
            r.font_size(Token::scale(0))
                .font_weight(Weight::W500)
                .color(Token::palette(Palette::Secondary))
        })
        .rule(tag(Tag::P), |r| r.margin_bottom(Token::scale(0)))
        .rule(tag(Tag::A), |r| {
            r.color(Token::palette(Palette::Accent))
                .text_decoration(Decoration::None)
                .transition("color 0.2s")
        })
        .rule(tag(Tag::A).pseudo(Pseudo::Hover), |r| {
            r.color(Token::palette(Palette::Hover))
                .text_decoration(Decoration::Underline)
        })
        .rule(tag(Tag::Code), |r| {
            r.font_family("'SF Mono', 'Fira Code', 'Cascadia Code', monospace")
                .font_size(Token::scale(0))
                .background(Token::palette(Palette::Code))
                .color(Token::palette(Palette::CodeText))
                .padding((Concrete::em(0.15), Concrete::em(0.4)))
                .border_radius(Concrete::px(4))
                .transition("background-color 0.3s ease, color 0.3s ease")
        })
        .rule(tag(Tag::Pre), |r| {
            r.font_family("'SF Mono', 'Fira Code', 'Cascadia Code', monospace")
                .font_size(Token::scale(0))
                .background(Token::palette(Palette::Code))
                .border_radius(Concrete::px(6))
                .padding(Token::scale(0))
                .overflow(Overflow::Auto)
                .margin_bottom(Token::scale(0))
                .line_height("1.5")
                .transition("background-color 0.3s ease")
        })
        .rule(tag(Tag::Pre).descendant(tag(Tag::Code)), |r| {
            r.background(Keyword::Transparent)
                .padding(Concrete::zero())
                .border_radius(Concrete::zero())
        })
        .rule(class::code::block(), |r| {
            r.font_family("'SF Mono', 'Fira Code', 'Cascadia Code', monospace")
                .font_size(Token::scale(0))
                .background(Token::palette(Palette::Code))
                .border_radius(Concrete::px(6))
                .padding(Token::scale(0))
                .overflow(Overflow::Auto)
                .margin_bottom(Token::scale(0))
                .line_height("1.5")
                .position(Position::Relative)
                .white_space(Space::Wrap)
                .transition("background-color 0.3s ease")
        })
        .rule(
            Selector::from(class::code::block()).descendant(class::syntax::keyword().into()),
            |r| r.color(Token::palette(Palette::Keyword)),
        )
        .rule(
            Selector::from(class::code::block()).descendant(class::syntax::entity().into()),
            |r| r.color(Token::palette(Palette::Entity)),
        )
        .rule(
            Selector::from(class::code::block()).descendant(class::syntax::string().into()),
            |r| r.color(Token::palette(Palette::Literal)),
        )
        .rule(
            Selector::from(class::code::block()).descendant(class::syntax::comment().into()),
            |r| {
                r.color(Token::palette(Palette::Comment))
                    .font_style(fontstyle::Style::Italic)
            },
        )
        .rule(
            Selector::from(class::code::block()).descendant(class::syntax::constant().into()),
            |r| r.color(Token::palette(Palette::Constant)),
        )
        .rule(
            Selector::from(class::code::block()).descendant(class::syntax::storage().into()),
            |r| r.color(Token::palette(Palette::Storage)),
        )
        .rule(
            Selector::from(class::code::block()).descendant(class::syntax::punctuation().into()),
            |r| r.color(Token::palette(Palette::Punctuation)),
        )
        .rule(
            Selector::from(class::code::block()).descendant(class::syntax::variable().into()),
            |r| r.color(Token::palette(Palette::Variable)),
        )
        .rule(
            Selector::from(class::code::block()).descendant(class::syntax::function().into()),
            |r| r.color(Token::palette(Palette::Function)),
        )
        .rule(
            Selector::from(class::code::block()).descendant(class::syntax::operator().into()),
            |r| r.color(Token::palette(Palette::Operator)),
        )
        .rule(
            Selector::from(class::code::block()).descendant(class::syntax::r#macro().into()),
            |r| r.color(Token::palette(Palette::Macro)),
        )
        .rule(tag(Tag::Nav), |r| {
            r.position(Position::Sticky)
                .top(Concrete::zero())
                .background(Token::palette(Palette::Navigation))
                .backdrop_filter("blur(8px)")
                .height(Calculation::start(Token::scale(3)).plus(Token::scale(-2)))
                .padding((Concrete::zero(), Token::scale(1)))
                .border_bottom("1px solid var(--border)")
                .display(Display::Flex)
                .align_items("center")
                .z_index("100")
                .transition("background-color 0.3s ease, border-color 0.3s ease")
        })
        .rule(class::nav::logo(), |r| {
            r.display(Display::Flex)
                .align_items("center")
                .flex_shrink(Concrete::zero())
        })
        .rule(
            Selector::from(class::nav::logo()).descendant(tag(Tag::Img)),
            |r| r.height(Token::scale(2)),
        )
        .rule(class::nav::links(), |r| {
            r.display(Display::Flex)
                .align_items("center")
                .gap(Token::scale(-1))
                .margin_left(Keyword::Auto)
        })
        .rule(
            group(vec![
                Selector::from(class::nav::links()).child(tag(Tag::A)),
                Selector::from(class::nav::dropdown()).child(tag(Tag::A)),
            ]),
            |r| {
                r.color(Token::palette(Palette::Secondary))
                    .font_size(Token::scale(0))
                    .font_weight(Weight::W500)
                    .padding((Token::scale(-2), Token::scale(-1)))
                    .white_space(Space::Nowrap)
                    .letter_spacing("0.01em")
            },
        )
        .rule(
            group(vec![
                Selector::from(class::nav::links()).child(tag(Tag::A).pseudo(Pseudo::Hover)),
                Selector::from(class::nav::dropdown()).child(tag(Tag::A).pseudo(Pseudo::Hover)),
            ]),
            |r| {
                r.color(Token::palette(Palette::Text))
                    .text_decoration(Decoration::None)
            },
        )
        .rule(class::nav::dropdown(), |r| {
            r.position(Position::Relative)
                .display(Display::Flex)
                .align_items("center")
        })
        .rule(class::nav::menu(), |r| {
            r.display(Display::None)
                .position(Position::Absolute)
                .top(Concrete::percent(100.0))
                .left(Concrete::zero())
                .background(Token::palette(Palette::Background))
                .border("1px solid var(--border)")
                .border_radius(Concrete::px(6))
                .box_shadow("0 4px 12px #0000001a")
                .padding((Token::scale(-2), Concrete::zero()))
                .min_width(Concrete::px(160))
                .z_index("200")
        })
        .rule(
            group(vec![
                Selector::from(class::nav::dropdown())
                    .pseudo(Pseudo::Hover)
                    .descendant(class::nav::menu().into()),
                Selector::from(class::nav::dropdown())
                    .pseudo(Pseudo::FocusWithin)
                    .descendant(class::nav::menu().into()),
            ]),
            |r| r.display(Display::Block),
        )
        .rule(
            Selector::from(class::nav::menu()).descendant(tag(Tag::A)),
            |r| {
                r.display(Display::Block)
                    .padding((Token::scale(-2), Token::scale(0)))
                    .color(Token::palette(Palette::Secondary))
                    .font_size(Token::scale(0))
            },
        )
        .rule(
            Selector::from(class::nav::menu()).descendant(tag(Tag::A).pseudo(Pseudo::Hover)),
            |r| {
                r.background(Token::palette(Palette::Code))
                    .color(Token::palette(Palette::Text))
                    .text_decoration(Decoration::None)
            },
        )
        .rule(
            Selector::from(class::nav::menu()).descendant(tag(Tag::Hr)),
            |r| r.margin((Token::scale(-2), Concrete::zero())),
        )
        .rule(
            Selector::from(class::nav::menu()).descendant(class::nav::nested().into()),
            |r| {
                r.padding_left(Token::scale(1))
                    .font_size(Token::half(0))
                    .border_left("2px solid var(--border)")
                    .color(Token::palette(Palette::Secondary))
                    .opacity("0.85")
            },
        )
        .rule(class::reference::layout(), |r| {
            r.display(Display::Grid)
                .grid_template_columns(grid())
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
            Selector::from(class::reference::sidebar()).descendant(class::nav::nested().into()),
            |r| {
                r.padding_left(Token::scale(1))
                    .font_size(Token::half(0))
                    .border_left("2px solid var(--border)")
                    .opacity("0.85")
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
        .rule(class::label::outline(), |r| {
            r.font_size(Token::half(0))
                .font_weight(Weight::W500)
                .text_transform(Transform::Uppercase)
                .letter_spacing("0.08em")
                .color(Token::palette(Palette::Secondary))
                .margin_bottom(Token::scale(-1))
        })
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
                .descendant(tag(Tag::A).attribute("data-depth", "0")),
            |r| {
                r.font_weight(Weight::W500)
                    .text_transform(Transform::Uppercase)
                    .letter_spacing("0.08em")
            },
        )
        .rule(
            Selector::from(class::reference::outline())
                .descendant(tag(Tag::A).pseudo(Pseudo::FirstChild)),
            |r| r.padding_top(Concrete::zero()),
        )
        .rule(
            Selector::from(class::reference::outline())
                .descendant(tag(Tag::A).attribute("data-depth", "1")),
            |r| r.padding_left(Token::scale(0)),
        )
        .rule(
            Selector::from(class::reference::outline())
                .descendant(tag(Tag::A).attribute("data-depth", "2")),
            |r| r.padding_left(Token::scale(1)),
        )
        .rule(
            Selector::from(class::reference::outline())
                .descendant(tag(Tag::A).attribute("data-depth", "3")),
            |r| r.padding_left(Token::scale(2)),
        )
        .rule(
            Selector::from(class::reference::outline())
                .descendant(tag(Tag::A).attribute("data-depth", "4")),
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
        .rule(class::label::sidebar(), |r| {
            r.font_size(Token::half(0))
                .font_weight(Weight::W500)
                .text_transform(Transform::Uppercase)
                .letter_spacing("0.08em")
                .color(Token::palette(Palette::Secondary))
                .margin_top(Token::scale(1))
                .margin_bottom(Token::scale(-1))
        })
        .rule(
            Selector::from(class::label::sidebar()).pseudo(Pseudo::FirstChild),
            |r| r.margin_top(Concrete::zero()),
        )
        .rule(class::reference::hamburger(), |r| {
            r.display(Display::None)
                .background(Keyword::Transparent)
                .border(Keyword::None)
                .cursor(Cursor::Pointer)
                .padding(Concrete::zero())
                .align_items("center")
                .justify_content("center")
                .align_self("center")
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
        .rule(tag(Tag::Table), |r| {
            r.width(Concrete::percent(100.0))
                .border_collapse(Collapse::Collapse)
                .margin_bottom(Token::scale(0))
        })
        .rule(tag(Tag::Th), |r| {
            r.text_align(align::Align::Left)
                .padding(Token::scale(-1))
                .border_bottom("2px solid var(--border)")
                .font_weight(Weight::W600)
                .font_size(Token::scale(-1))
                .text_transform(Transform::Uppercase)
                .letter_spacing("0.02em")
                .color(Token::palette(Palette::Secondary))
        })
        .rule(tag(Tag::Td), |r| {
            r.padding(Token::scale(-1))
                .border_bottom("1px solid var(--border)")
        })
        .rule(
            tag(Tag::Tbody).descendant(tag(Tag::Tr).pseudo(Pseudo::NthChild("even".into()))),
            |r| r.background(Token::palette(Palette::Stripe)),
        )
        .rule(tag(Tag::Blockquote), |r| {
            r.border_left("3px solid var(--accent)")
                .padding_left(Token::scale(0))
                .color(Token::palette(Palette::Secondary))
                .margin_bottom(Token::scale(0))
                .font_style(fontstyle::Style::Italic)
        })
        .rule(tag(Tag::Hr), |r| {
            r.border(Keyword::None)
                .border_bottom("1px solid var(--border)")
                .margin((Token::scale(1), Concrete::zero()))
        })
        .rule(tag(Tag::Img), |r| {
            r.max_width(Concrete::percent(100.0))
                .height(Keyword::Auto)
                .display(Display::Block)
        })
        .rule(class::reference::center(), |r| {
            r.text_align(align::Align::Center)
                .margin((Concrete::zero(), Keyword::Auto))
        })
        .rule(class::reference::subtitle(), |r| {
            r.color(Token::palette(Palette::Secondary))
                .font_size(Token::scale(1))
                .font_weight(Weight::W400)
                .letter_spacing("-0.01em")
                .margin_bottom(Token::scale(1))
                .display(Display::Block)
        })
        .rule(
            tag(Tag::A)
                .and(class::reference::subtitle().into())
                .pseudo(Pseudo::Hover),
            |r| {
                r.color(Token::palette(Palette::Text))
                    .text_decoration(Decoration::None)
            },
        )
        .rule(group(vec![tag(Tag::Ul), tag(Tag::Ol)]), |r| {
            r.padding_left(Token::scale(1))
                .margin_bottom(Token::scale(0))
        })
        .rule(tag(Tag::Li), |r| r.margin_bottom(Token::scale(-2)))
        .rule(tag(Tag::Dl), |r| r.margin_bottom(Token::scale(0)))
        .rule(tag(Tag::Dt), |r| {
            r.font_weight(Weight::W600)
                .margin_top(Token::scale(-1))
                .letter_spacing("-0.01em")
        })
        .rule(tag(Tag::Dd), |r| {
            r.margin_bottom(Token::scale(-1))
                .padding_left(Token::scale(0))
        })
        .rule(tag(Tag::Footer), |r| {
            r.text_align(align::Align::Center)
                .padding((
                    Token::scale(2),
                    Concrete::zero(),
                    Token::scale(1),
                    Concrete::zero(),
                ))
                .margin_top(Token::scale(2))
                .border_top("1px solid var(--border)")
                .color(Token::palette(Palette::Secondary))
                .font_size(Token::scale(-1))
                .letter_spacing("0.02em")
        })
        .rule(class::footer::icon(), |r| {
            r.display(Display::Inline)
                .color(Token::palette(Palette::Secondary))
                .margin_top(Token::scale(-1))
                .transition("color 0.2s")
        })
        .rule(
            Selector::from(class::footer::icon()).pseudo(Pseudo::Hover),
            |r| {
                r.color(Token::palette(Palette::Text))
                    .text_decoration(Decoration::None)
            },
        )
        .rule(
            Selector::from(class::footer::icon()).descendant(tag(Tag::Svg)),
            |r| r.display(Display::Block),
        )
        .rule(class::code::toolbar(), |r| {
            r.position(Position::Absolute)
                .top(Concrete::px(8))
                .right(Concrete::px(8))
                .display(Display::Flex)
                .gap(Concrete::px(4))
                .opacity("0")
                .transition("opacity 0.2s")
        })
        .rule(
            Selector::from(class::code::block())
                .pseudo(Pseudo::Hover)
                .descendant(class::code::toolbar().into()),
            |r| r.opacity("1"),
        )
        .rule(
            group(vec![
                class::code::source().into(),
                class::button::copy().into(),
            ]),
            |r| {
                r.appearance(appearance::Appearance::None)
                    .background(Token::palette(Palette::Navigation))
                    .border("1px solid var(--border)")
                    .border_radius(Concrete::px(4))
                    .padding((Concrete::px(2), Concrete::px(8)))
                    .color(Token::palette(Palette::Secondary))
                    .cursor(Cursor::Pointer)
                    .font_size(Token::half(0))
                    .font_family("inherit")
                    .line_height("1.5")
                    .text_decoration(Decoration::None)
                    .display(Display::Inline)
            },
        )
        .rule(
            group(vec![
                Selector::from(class::code::source()).pseudo(Pseudo::Hover),
                Selector::from(class::button::copy()).pseudo(Pseudo::Hover),
            ]),
            |r| {
                r.color(Token::palette(Palette::Text))
                    .text_decoration(Decoration::None)
            },
        )
        .rule(class::button::theme(), |r| {
            r.background(Keyword::Transparent)
                .border(Keyword::None)
                .cursor(Cursor::Pointer)
                .font_size(Token::scale(0))
                .width(Concrete::rem(1.5))
                .height(Concrete::rem(1.5))
                .flex_shrink(Concrete::zero())
                .display(Display::Flex)
                .align_items("center")
                .justify_content("center")
                .overflow(Overflow::Hidden)
                .padding(Concrete::zero())
                .color(Token::palette(Palette::Secondary))
                .transition("color 0.2s")
        })
        .rule(
            Selector::from(class::button::theme()).descendant(tag(Tag::Svg)),
            |r| r.width(Token::scale(0)).height(Token::scale(0)),
        )
        .rule(
            Selector::from(class::button::theme()).pseudo(Pseudo::Hover),
            |r| r.color(Token::palette(Palette::Text)),
        )
        .rule(
            Selector::from(class::reference::enhanced()).descendant(present("data-animate")),
            |r| {
                r.opacity("0")
                    .transform("translateY(20px)")
                    .transition("opacity 0.6s ease, transform 0.6s ease")
            },
        )
        .rule(
            Selector::from(class::reference::enhanced()).descendant(present("data-visible")),
            |r| r.opacity("1").transform("translateY(0)"),
        )
        .rule(tag(Tag::Html).attribute("data-theme", "dark"), dark)
        .rule(
            tag(Tag::Html)
                .attribute("data-theme", "dark")
                .descendant(class::nav::menu().into()),
            |r| r.box_shadow("0 4px 12px #00000066"),
        )
        .rule(tag(Tag::Html).attribute("data-theme", "light"), |r| {
            palette::PALETTE
                .iter()
                .chain(palette::SYNTAX)
                .fold(r, |p, token| {
                    p.custom(&format!("--{}", token.role), token.light)
                })
        })
        .media("prefers-color-scheme: dark", |m| {
            palette::PALETTE
                .iter()
                .chain(palette::SYNTAX)
                .fold(m, |s, token| {
                    s.variable(&format!("--{}", token.role), token.dark)
                })
                .rule(class::nav::menu(), |r| r.box_shadow("0 4px 12px #00000066"))
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
                    .z_index("150")
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
                r.font_size(Token::scale(2)).letter_spacing("-0.02em")
            })
            .rule(tag(Tag::Nav), |r| {
                r.padding((Concrete::zero(), Token::scale(-1)))
            })
        })
}
