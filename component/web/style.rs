use selector::Selector;
use value::{Concrete, Value};

pub struct Style {
    pub rules: Vec<Rule>,
    pub variables: Vec<(String, String)>,
    pub keyframes: Vec<Keyframe>,
    pub media: Vec<Media>,
}

pub struct Rule {
    pub selector: String,
    pub properties: Properties,
}

#[must_use]
pub struct Properties {
    pub entries: Vec<(String, String)>,
}

pub struct Keyframe {
    pub name: String,
    pub steps: Vec<Rule>,
}

pub struct Media {
    pub query: String,
    pub style: Style,
}

impl Style {
    #[must_use]
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            variables: Vec::new(),
            keyframes: Vec::new(),
            media: Vec::new(),
        }
    }

    #[must_use]
    pub fn variable(mut self, name: &str, value: &str) -> Self {
        self.variables.push((name.into(), value.into()));
        self
    }

    #[must_use]
    pub fn rule<S>(mut self, selector: S, f: impl FnOnce(Properties) -> Properties) -> Self
    where
        S: Into<Selector>,
    {
        self.rules.push(Rule {
            selector: selector.into().to_string(),
            properties: f(Properties::new()),
        });
        self
    }

    #[must_use]
    pub fn keyframe(mut self, name: &str, f: impl FnOnce(Keyframe) -> Keyframe) -> Self {
        self.keyframes.push(f(Keyframe::new(name)));
        self
    }

    #[must_use]
    pub fn media(mut self, query: &str, f: impl FnOnce(Style) -> Style) -> Self {
        self.media.push(Media {
            query: query.into(),
            style: f(Style::new()),
        });
        self
    }

    #[must_use]
    pub fn extend(mut self, other: Style) -> Self {
        self.variables.extend(other.variables);
        self.rules.extend(other.rules);
        self.keyframes.extend(other.keyframes);
        self.media.extend(other.media);
        self
    }
}

impl Keyframe {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            steps: Vec::new(),
        }
    }

    #[must_use]
    pub fn step(mut self, position: &str, f: impl FnOnce(Properties) -> Properties) -> Self {
        self.steps.push(Rule {
            selector: position.into(),
            properties: f(Properties::new()),
        });
        self
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! properties {
    (
        valued { $( $method:ident => $css:literal ),* $(,)? }
        typed { $( $tmethod:ident ( $ty:ty ) => $tcss:literal ),* $(,)? }
    ) => {
        impl Properties {
            $(
                pub fn $method<V>(self, value: V) -> Self
                where
                    V: Into<Value>,
                {
                    self.valued($css, value)
                }
            )*
            $(
                pub fn $tmethod(self, value: $ty) -> Self {
                    self.property($tcss, value.css())
                }
            )*
        }
    };
}

impl Properties {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    fn property(mut self, name: &str, value: &str) -> Self {
        self.entries.push((name.into(), value.into()));
        self
    }

    fn valued<V>(mut self, name: &str, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.entries.push((name.into(), value.into().to_string()));
        self
    }

    pub fn z_index(self, value: i32) -> Self {
        self.valued("z-index", Concrete::integer(value))
    }

    pub fn opacity(self, value: f32) -> Self {
        self.valued("opacity", Concrete::unitless(value))
    }

    pub fn custom(self, name: &str, value: &str) -> Self {
        self.property(name, value)
    }
}

properties! {
    valued {
        margin => "margin",
        margin_top => "margin-top",
        margin_bottom => "margin-bottom",
        margin_left => "margin-left",
        padding => "padding",
        padding_left => "padding-left",
        padding_top => "padding-top",
        width => "width",
        min_width => "min-width",
        max_width => "max-width",
        height => "height",
        min_height => "min-height",
        font_family => "font-family",
        font_size => "font-size",
        line_height => "line-height",
        letter_spacing => "letter-spacing",
        color => "color",
        background => "background",
        border => "border",
        border_top => "border-top",
        border_left => "border-left",
        border_right => "border-right",
        border_bottom => "border-bottom",
        border_radius => "border-radius",
        border_color => "border-color",
        border_left_color => "border-left-color",
        top => "top",
        left => "left",
        right => "right",
        box_shadow => "box-shadow",
        gap => "gap",
        flex_shrink => "flex-shrink",
        grid_template_columns => "grid-template-columns",
        backdrop_filter => "backdrop-filter",
        transition => "transition",
        transform => "transform",
        list_style => "list-style",
        scroll_padding_top => "scroll-padding-top",
        outline => "outline",
        outline_offset => "outline-offset",
        pointer_events => "pointer-events",
    }
    typed {
        font_weight(weight::Weight) => "font-weight",
        font_style(fontstyle::Style) => "font-style",
        text_align(align::Align) => "text-align",
        text_decoration(decoration::Decoration) => "text-decoration",
        text_transform(transform::Transform) => "text-transform",
        display(display::Display) => "display",
        position(position::Position) => "position",
        overflow(overflow::Overflow) => "overflow",
        overflow_y(overflow::Overflow) => "overflow-y",
        box_sizing(sizing::Box) => "box-sizing",
        flex_wrap(wrap::Wrap) => "flex-wrap",
        flex_direction(direction::Direction) => "flex-direction",
        justify_content(alignment::Alignment) => "justify-content",
        align_items(alignment::Alignment) => "align-items",
        align_self(alignment::Alignment) => "align-self",
        border_collapse(collapse::Collapse) => "border-collapse",
        cursor(cursor::Cursor) => "cursor",
        white_space(space::Space) => "white-space",
        appearance(appearance::Appearance) => "appearance",
    }
}

impl Default for Properties {
    fn default() -> Self {
        Self::new()
    }
}
