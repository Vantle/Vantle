use collapse::Collapse;
use decoration::Decoration;
use display::Display;
use overflow::Overflow;
use position::Position;
use selector::{Parity, Pseudo, Selector, Tag, group, tag, universal};
use sizing::Box as Sizing;
use space::Space;
use style::Style;
use transform::Transform;
use value::{Calculation, Concrete, Keyword, Palette, Token};
use weight::Weight;

#[must_use]
pub fn foundation() -> Style {
    Style::new()
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
                .line_height(Concrete::unitless(1.618_034))
                .font_size(Token::scale(0))
                .transition("background-color 0.3s ease, color 0.3s ease")
        })
        .rule(tag(Tag::Main), |r| {
            r.padding((Token::scale(2), Token::scale(1)))
                .min_width(Concrete::zero())
        })
}

#[must_use]
pub fn typography() -> Style {
    Style::new()
        .rule(tag(Tag::H1), |r| {
            r.font_size(Token::scale(3))
                .font_weight(Weight::W700)
                .margin_bottom(Token::scale(-1))
                .line_height(Concrete::unitless(1.1))
                .letter_spacing(Concrete::em(-0.03))
        })
        .rule(tag(Tag::H2), |r| {
            r.font_size(Token::scale(2))
                .font_weight(Weight::W600)
                .margin_top(Token::scale(2))
                .margin_bottom(Token::scale(0))
                .letter_spacing(Concrete::em(-0.02))
        })
        .rule(tag(Tag::H3), |r| {
            r.font_size(Token::scale(1))
                .font_weight(Weight::W600)
                .margin_top(Token::scale(1))
                .margin_bottom(Token::scale(-1))
                .letter_spacing(Concrete::em(-0.01))
        })
        .rule(tag(Tag::H4), |r| {
            r.font_size(Token::scale(0))
                .font_weight(Weight::W600)
                .letter_spacing(Concrete::em(-0.01))
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
                .line_height(Concrete::unitless(1.5))
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
                .line_height(Concrete::unitless(1.5))
                .position(Position::Relative)
                .white_space(Space::Wrap)
                .transition("background-color 0.3s ease")
        })
}

#[must_use]
pub fn content() -> Style {
    Style::new()
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
                .letter_spacing(Concrete::em(0.02))
                .color(Token::palette(Palette::Secondary))
        })
        .rule(tag(Tag::Td), |r| {
            r.padding(Token::scale(-1))
                .border_bottom("1px solid var(--border)")
        })
        .rule(
            tag(Tag::Tbody).descendant(tag(Tag::Tr).pseudo(Pseudo::NthChild(Parity::Even))),
            |r| r.background(Token::palette(Palette::Stripe)),
        )
        .rule(tag(Tag::Blockquote), |r| {
            r.border_left("3px solid var(--accent)")
                .padding_left(Token::scale(0))
                .color(Token::palette(Palette::Secondary))
                .margin_bottom(Token::scale(0))
                .font_style(emphasis::Style::Italic)
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
                .letter_spacing(Concrete::em(-0.01))
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
                .letter_spacing(Concrete::em(-0.01))
        })
        .rule(tag(Tag::Dd), |r| {
            r.margin_bottom(Token::scale(-1))
                .padding_left(Token::scale(0))
        })
}

#[must_use]
pub fn footer() -> Style {
    Style::new()
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
                .letter_spacing(Concrete::em(0.02))
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
}
