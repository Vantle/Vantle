use cursor::Cursor;
use decoration::Decoration;
use display::Display;
use selector::{Pseudo, Selector, Tag, group, tag};
use style::Style;
use value::{Concrete, Keyword, Palette, Token};

fn syntax(style: Style, token: Selector, palette: Palette) -> Style {
    style.rule(
        Selector::from(class::code::block()).descendant(token),
        |r| r.color(Token::palette(palette)),
    )
}

#[must_use]
pub fn highlighting() -> Style {
    let style = Style::new();
    let style = syntax(style, class::syntax::keyword().into(), Palette::Keyword);
    let style = syntax(style, class::syntax::entity().into(), Palette::Entity);
    let style = syntax(style, class::syntax::string().into(), Palette::Literal);
    let style = style.rule(
        Selector::from(class::code::block()).descendant(class::syntax::comment().into()),
        |r| {
            r.color(Token::palette(Palette::Comment))
                .font_style(emphasis::Style::Italic)
        },
    );
    let style = syntax(style, class::syntax::constant().into(), Palette::Constant);
    let style = syntax(style, class::syntax::storage().into(), Palette::Storage);
    let style = syntax(
        style,
        class::syntax::punctuation().into(),
        Palette::Punctuation,
    );
    let style = syntax(style, class::syntax::variable().into(), Palette::Variable);
    let style = syntax(style, class::syntax::function().into(), Palette::Function);
    let style = syntax(style, class::syntax::operator().into(), Palette::Operator);
    syntax(style, class::syntax::r#macro().into(), Palette::Macro)
}

#[must_use]
pub fn toolbar() -> Style {
    Style::new()
        .rule(class::code::toolbar(), |r| {
            r.position(position::Position::Absolute)
                .top(Concrete::px(8))
                .right(Concrete::px(8))
                .display(Display::Flex)
                .gap(Concrete::px(4))
                .opacity(0.0)
                .transition("opacity 0.2s")
        })
        .rule(
            Selector::from(class::code::block())
                .pseudo(Pseudo::Hover)
                .descendant(class::code::toolbar().into()),
            |r| r.opacity(1.0),
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
                    .line_height(Concrete::unitless(1.5))
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
                .align_items(alignment::Alignment::Center)
                .justify_content(alignment::Alignment::Center)
                .overflow(overflow::Overflow::Hidden)
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
}
