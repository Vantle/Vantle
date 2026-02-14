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
    pub fn rule(mut self, selector: &str, f: impl FnOnce(Properties) -> Properties) -> Self {
        self.rules.push(Rule {
            selector: selector.into(),
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

    #[must_use]
    pub fn margin(self, value: &str) -> Self {
        self.property("margin", value)
    }

    #[must_use]
    pub fn padding(self, value: &str) -> Self {
        self.property("padding", value)
    }

    #[must_use]
    pub fn width(self, value: &str) -> Self {
        self.property("width", value)
    }

    #[must_use]
    pub fn height(self, value: &str) -> Self {
        self.property("height", value)
    }

    #[must_use]
    pub fn max_width(self, value: &str) -> Self {
        self.property("max-width", value)
    }

    #[must_use]
    pub fn min_height(self, value: &str) -> Self {
        self.property("min-height", value)
    }

    #[must_use]
    pub fn font_family(self, value: &str) -> Self {
        self.property("font-family", value)
    }

    #[must_use]
    pub fn font_size(self, value: &str) -> Self {
        self.property("font-size", value)
    }

    #[must_use]
    pub fn font_weight(self, value: &str) -> Self {
        self.property("font-weight", value)
    }

    #[must_use]
    pub fn line_height(self, value: &str) -> Self {
        self.property("line-height", value)
    }

    #[must_use]
    pub fn text_align(self, value: &str) -> Self {
        self.property("text-align", value)
    }

    #[must_use]
    pub fn text_decoration(self, value: &str) -> Self {
        self.property("text-decoration", value)
    }

    #[must_use]
    pub fn color(self, value: &str) -> Self {
        self.property("color", value)
    }

    #[must_use]
    pub fn background(self, value: &str) -> Self {
        self.property("background", value)
    }

    #[must_use]
    pub fn border(self, value: &str) -> Self {
        self.property("border", value)
    }

    #[must_use]
    pub fn border_left(self, value: &str) -> Self {
        self.property("border-left", value)
    }

    #[must_use]
    pub fn border_bottom(self, value: &str) -> Self {
        self.property("border-bottom", value)
    }

    #[must_use]
    pub fn border_radius(self, value: &str) -> Self {
        self.property("border-radius", value)
    }

    #[must_use]
    pub fn border_collapse(self, value: &str) -> Self {
        self.property("border-collapse", value)
    }

    #[must_use]
    pub fn display(self, value: &str) -> Self {
        self.property("display", value)
    }

    #[must_use]
    pub fn position(self, value: &str) -> Self {
        self.property("position", value)
    }

    #[must_use]
    pub fn top(self, value: &str) -> Self {
        self.property("top", value)
    }

    #[must_use]
    pub fn left(self, value: &str) -> Self {
        self.property("left", value)
    }

    #[must_use]
    pub fn right(self, value: &str) -> Self {
        self.property("right", value)
    }

    #[must_use]
    pub fn overflow(self, value: &str) -> Self {
        self.property("overflow", value)
    }

    #[must_use]
    pub fn box_sizing(self, value: &str) -> Self {
        self.property("box-sizing", value)
    }

    #[must_use]
    pub fn gap(self, value: &str) -> Self {
        self.property("gap", value)
    }

    #[must_use]
    pub fn flex_wrap(self, value: &str) -> Self {
        self.property("flex-wrap", value)
    }

    #[must_use]
    pub fn justify_content(self, value: &str) -> Self {
        self.property("justify-content", value)
    }

    #[must_use]
    pub fn align_items(self, value: &str) -> Self {
        self.property("align-items", value)
    }

    #[must_use]
    pub fn backdrop_filter(self, value: &str) -> Self {
        self.property("backdrop-filter", value)
    }

    #[must_use]
    pub fn transition(self, value: &str) -> Self {
        self.property("transition", value)
    }

    #[must_use]
    pub fn opacity(self, value: &str) -> Self {
        self.property("opacity", value)
    }

    #[must_use]
    pub fn transform(self, value: &str) -> Self {
        self.property("transform", value)
    }

    #[must_use]
    pub fn cursor(self, value: &str) -> Self {
        self.property("cursor", value)
    }

    #[must_use]
    pub fn white_space(self, value: &str) -> Self {
        self.property("white-space", value)
    }

    #[must_use]
    pub fn list_style(self, value: &str) -> Self {
        self.property("list-style", value)
    }

    #[must_use]
    pub fn margin_bottom(self, value: &str) -> Self {
        self.property("margin-bottom", value)
    }

    #[must_use]
    pub fn margin_top(self, value: &str) -> Self {
        self.property("margin-top", value)
    }

    #[must_use]
    pub fn padding_left(self, value: &str) -> Self {
        self.property("padding-left", value)
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
