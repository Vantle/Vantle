use selector::Selector;
use value::Value;

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
            selector: selector.into().render(),
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

impl Properties {
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    fn property(mut self, name: &str, value: &str) -> Self {
        self.entries.push((name.into(), value.into()));
        self
    }

    fn valued<V>(self, name: &str, value: V) -> Self
    where
        V: Into<Value>,
    {
        let rendered = value.into().render();
        self.property(name, &rendered)
    }

    #[must_use]
    pub fn margin<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("margin", value)
    }

    #[must_use]
    pub fn margin_top<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("margin-top", value)
    }

    #[must_use]
    pub fn margin_bottom<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("margin-bottom", value)
    }

    #[must_use]
    pub fn margin_left<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("margin-left", value)
    }

    #[must_use]
    pub fn padding<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("padding", value)
    }

    #[must_use]
    pub fn padding_left<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("padding-left", value)
    }

    #[must_use]
    pub fn padding_top<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("padding-top", value)
    }

    #[must_use]
    pub fn width<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("width", value)
    }

    #[must_use]
    pub fn min_width<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("min-width", value)
    }

    #[must_use]
    pub fn max_width<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("max-width", value)
    }

    #[must_use]
    pub fn height<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("height", value)
    }

    #[must_use]
    pub fn min_height<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("min-height", value)
    }

    #[must_use]
    pub fn font_family<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("font-family", value)
    }

    #[must_use]
    pub fn font_size<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("font-size", value)
    }

    #[must_use]
    pub fn font_weight(self, value: weight::Weight) -> Self {
        self.property("font-weight", value.css())
    }

    #[must_use]
    pub fn font_style(self, value: fontstyle::Style) -> Self {
        self.property("font-style", value.css())
    }

    #[must_use]
    pub fn line_height<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("line-height", value)
    }

    #[must_use]
    pub fn letter_spacing<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("letter-spacing", value)
    }

    #[must_use]
    pub fn text_align(self, value: align::Align) -> Self {
        self.property("text-align", value.css())
    }

    #[must_use]
    pub fn text_decoration(self, value: decoration::Decoration) -> Self {
        self.property("text-decoration", value.css())
    }

    #[must_use]
    pub fn text_transform(self, value: transform::Transform) -> Self {
        self.property("text-transform", value.css())
    }

    #[must_use]
    pub fn color<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("color", value)
    }

    #[must_use]
    pub fn background<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("background", value)
    }

    #[must_use]
    pub fn border<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("border", value)
    }

    #[must_use]
    pub fn border_top<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("border-top", value)
    }

    #[must_use]
    pub fn border_left<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("border-left", value)
    }

    #[must_use]
    pub fn border_right<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("border-right", value)
    }

    #[must_use]
    pub fn border_bottom<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("border-bottom", value)
    }

    #[must_use]
    pub fn border_radius<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("border-radius", value)
    }

    #[must_use]
    pub fn border_collapse(self, value: collapse::Collapse) -> Self {
        self.property("border-collapse", value.css())
    }

    #[must_use]
    pub fn border_left_color<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("border-left-color", value)
    }

    #[must_use]
    pub fn display(self, value: display::Display) -> Self {
        self.property("display", value.css())
    }

    #[must_use]
    pub fn position(self, value: position::Position) -> Self {
        self.property("position", value.css())
    }

    #[must_use]
    pub fn top<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("top", value)
    }

    #[must_use]
    pub fn left<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("left", value)
    }

    #[must_use]
    pub fn right<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("right", value)
    }

    #[must_use]
    pub fn overflow(self, value: overflow::Overflow) -> Self {
        self.property("overflow", value.css())
    }

    #[must_use]
    pub fn overflow_y(self, value: overflow::Overflow) -> Self {
        self.property("overflow-y", value.css())
    }

    #[must_use]
    pub fn box_sizing(self, value: sizing::Box) -> Self {
        self.property("box-sizing", value.css())
    }

    #[must_use]
    pub fn box_shadow<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("box-shadow", value)
    }

    #[must_use]
    pub fn gap<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("gap", value)
    }

    #[must_use]
    pub fn flex_wrap(self, value: wrap::Wrap) -> Self {
        self.property("flex-wrap", value.css())
    }

    #[must_use]
    pub fn flex_direction(self, value: direction::Direction) -> Self {
        self.property("flex-direction", value.css())
    }

    #[must_use]
    pub fn flex_shrink<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("flex-shrink", value)
    }

    #[must_use]
    pub fn justify_content<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("justify-content", value)
    }

    #[must_use]
    pub fn align_items<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("align-items", value)
    }

    #[must_use]
    pub fn align_self<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("align-self", value)
    }

    #[must_use]
    pub fn grid_template_columns<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("grid-template-columns", value)
    }

    #[must_use]
    pub fn z_index<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("z-index", value)
    }

    #[must_use]
    pub fn backdrop_filter<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("backdrop-filter", value)
    }

    #[must_use]
    pub fn transition<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("transition", value)
    }

    #[must_use]
    pub fn opacity<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("opacity", value)
    }

    #[must_use]
    pub fn transform<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("transform", value)
    }

    #[must_use]
    pub fn cursor(self, value: cursor::Cursor) -> Self {
        self.property("cursor", value.css())
    }

    #[must_use]
    pub fn white_space(self, value: space::Space) -> Self {
        self.property("white-space", value.css())
    }

    #[must_use]
    pub fn list_style<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("list-style", value)
    }

    #[must_use]
    pub fn appearance(self, value: appearance::Appearance) -> Self {
        self.property("appearance", value.css())
    }

    #[must_use]
    pub fn scroll_padding_top<V>(self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.valued("scroll-padding-top", value)
    }

    #[must_use]
    pub fn custom(self, name: &str, value: &str) -> Self {
        self.property(name, value)
    }
}

impl Default for Properties {
    fn default() -> Self {
        Self::new()
    }
}
