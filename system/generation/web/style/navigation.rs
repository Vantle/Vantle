use alignment::Alignment;
use decoration::Decoration;
use display::Display;
use position::Position;
use selector::{Pseudo, Selector, Tag, data, group, tag};
use space::Space;
use style::Style;
use value::{Calculation, Concrete, Keyword, Palette, Token};
use weight::Weight;

#[must_use]
pub fn navigation() -> Style {
    Style::new()
        .rule(tag(Tag::Nav), |r| {
            r.position(Position::Sticky)
                .top(Concrete::zero())
                .background(Token::palette(Palette::Navigation))
                .backdrop_filter("blur(8px)")
                .height(Calculation::start(Token::scale(3)).plus(Token::scale(-2)))
                .padding((Concrete::zero(), Token::scale(1)))
                .border_bottom("1px solid var(--border)")
                .display(Display::Flex)
                .align_items(Alignment::Center)
                .z_index(100)
                .transition(
                    "background-color 0.3s ease, border-color 0.3s ease, \
                     box-shadow var(--duration-fast) var(--ease-out)",
                )
        })
        .rule(
            tag(Tag::Nav).and(data(attribute::scroll::scrolled())),
            |r| {
                r.box_shadow("0 1px 3px rgba(0, 0, 0, 0.08)")
                    .border_bottom("1px solid transparent")
            },
        )
        .rule(class::navigation::logo(), |r| {
            r.display(Display::Flex)
                .align_items(Alignment::Center)
                .flex_shrink(Concrete::zero())
        })
        .rule(
            Selector::from(class::navigation::logo()).descendant(tag(Tag::Img)),
            |r| r.height(Token::scale(2)).width(Keyword::Auto),
        )
        .rule(class::navigation::links(), |r| {
            r.display(Display::Flex)
                .align_items(Alignment::Center)
                .gap(Token::scale(-1))
                .margin_left(value::Keyword::Auto)
        })
        .rule(
            group(vec![
                Selector::from(class::navigation::links()).child(tag(Tag::A)),
                Selector::from(class::navigation::dropdown()).child(tag(Tag::A)),
            ]),
            |r| {
                r.color(Token::palette(Palette::Secondary))
                    .font_size(Token::scale(0))
                    .font_weight(Weight::W500)
                    .padding((Token::scale(-2), Token::scale(-1)))
                    .white_space(Space::Nowrap)
                    .letter_spacing(Concrete::em(0.01))
            },
        )
        .rule(
            group(vec![
                Selector::from(class::navigation::links()).child(tag(Tag::A).pseudo(Pseudo::Hover)),
                Selector::from(class::navigation::dropdown())
                    .child(tag(Tag::A).pseudo(Pseudo::Hover)),
            ]),
            |r| {
                r.color(Token::palette(Palette::Text))
                    .text_decoration(Decoration::None)
            },
        )
        .rule(class::navigation::dropdown(), |r| {
            r.position(Position::Relative)
                .display(Display::Flex)
                .align_items(Alignment::Center)
        })
        .rule(class::navigation::menu(), |r| {
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
                .z_index(200)
        })
        .rule(
            group(vec![
                Selector::from(class::navigation::dropdown())
                    .pseudo(Pseudo::Hover)
                    .descendant(class::navigation::menu().into()),
                Selector::from(class::navigation::dropdown())
                    .pseudo(Pseudo::FocusWithin)
                    .descendant(class::navigation::menu().into()),
            ]),
            |r| r.display(Display::Block),
        )
        .rule(
            Selector::from(class::navigation::menu()).descendant(tag(Tag::A)),
            |r| {
                r.display(Display::Block)
                    .padding((Token::scale(-2), Token::scale(0)))
                    .color(Token::palette(Palette::Secondary))
                    .font_size(Token::scale(0))
            },
        )
        .rule(
            Selector::from(class::navigation::menu()).descendant(tag(Tag::A).pseudo(Pseudo::Hover)),
            |r| {
                r.background(Token::palette(Palette::Code))
                    .color(Token::palette(Palette::Text))
                    .text_decoration(Decoration::None)
            },
        )
        .rule(
            Selector::from(class::navigation::menu()).descendant(tag(Tag::Hr)),
            |r| r.margin((Token::scale(-2), Concrete::zero())),
        )
        .rule(
            Selector::from(class::navigation::menu())
                .descendant(class::navigation::nested().into()),
            |r| {
                r.padding_left(Token::scale(1))
                    .font_size(Token::half(0))
                    .border_left("2px solid var(--border)")
                    .color(Token::palette(Palette::Secondary))
                    .opacity(0.85)
            },
        )
}
