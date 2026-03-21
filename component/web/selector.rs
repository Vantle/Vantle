pub enum Selector {
    Tag(Tag),
    Universal,
    Class(String),
    Id(String),
    Attribute(Attribute),
    Pseudo(Box<Selector>, Pseudo),
    Descendant(Box<Selector>, Box<Selector>),
    Child(Box<Selector>, Box<Selector>),
    Compound(Vec<Selector>),
    Group(Vec<Selector>),
}

pub enum Tag {
    Html,
    Body,
    Nav,
    Main,
    Aside,
    Footer,
    Section,
    Div,
    Span,
    P,
    A,
    Img,
    Hr,
    Br,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Ul,
    Ol,
    Li,
    Dl,
    Dt,
    Dd,
    Table,
    Thead,
    Tbody,
    Tr,
    Th,
    Td,
    Pre,
    Code,
    Strong,
    Em,
    Blockquote,
    Button,
    Svg,
    Details,
    Summary,
    Input,
}

pub enum Pseudo {
    Hover,
    Focus,
    FocusVisible,
    FocusWithin,
    FirstChild,
    NthChild(Parity),
    Active,
}

pub enum Parity {
    Even,
    Odd,
}

pub struct Attribute {
    name: String,
    value: Option<String>,
}

#[must_use]
pub fn tag(element: Tag) -> Selector {
    Selector::Tag(element)
}

#[must_use]
pub fn class(reference: reference::Reference) -> Selector {
    reference.into()
}

#[must_use]
pub fn id(name: &str) -> Selector {
    Selector::Id(name.into())
}

#[must_use]
pub fn attribute(name: &str, value: &str) -> Selector {
    Selector::Attribute(Attribute {
        name: name.into(),
        value: Some(value.into()),
    })
}

#[must_use]
pub fn present(name: &str) -> Selector {
    Selector::Attribute(Attribute {
        name: name.into(),
        value: None,
    })
}

#[must_use]
pub fn data(reference: attribute::Reference) -> Selector {
    present(reference.name())
}

#[must_use]
pub fn universal() -> Selector {
    Selector::Universal
}

#[must_use]
pub fn group(selectors: Vec<Selector>) -> Selector {
    Selector::Group(selectors)
}

impl Selector {
    #[must_use]
    pub fn descendant(self, other: Selector) -> Selector {
        Selector::Descendant(Box::new(self), Box::new(other))
    }

    #[must_use]
    pub fn child(self, other: Selector) -> Selector {
        Selector::Child(Box::new(self), Box::new(other))
    }

    #[must_use]
    pub fn pseudo(self, pseudo: Pseudo) -> Selector {
        Selector::Pseudo(Box::new(self), pseudo)
    }

    #[must_use]
    pub fn and(self, other: Selector) -> Selector {
        match self {
            Selector::Compound(mut items) => {
                items.push(other);
                Selector::Compound(items)
            }
            _ => Selector::Compound(vec![self, other]),
        }
    }

    #[must_use]
    pub fn attribute(self, name: &str, value: &str) -> Selector {
        self.and(Selector::Attribute(Attribute {
            name: name.into(),
            value: Some(value.into()),
        }))
    }

    #[must_use]
    pub fn data(self, reference: attribute::Reference, value: &str) -> Selector {
        self.attribute(reference.name(), value)
    }

    #[must_use]
    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl std::fmt::Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tag(element) => f.write_str(element.render()),
            Self::Universal => f.write_str("*"),
            Self::Class(name) => write!(f, ".{name}"),
            Self::Id(name) => write!(f, "#{name}"),
            Self::Attribute(attr) => std::fmt::Display::fmt(attr, f),
            Self::Pseudo(base, pseudo) => write!(f, "{base}:{pseudo}"),
            Self::Descendant(parent, child) => write!(f, "{parent} {child}"),
            Self::Child(parent, child) => write!(f, "{parent} > {child}"),
            Self::Compound(items) => {
                for item in items {
                    std::fmt::Display::fmt(item, f)?;
                }
                Ok(())
            }
            Self::Group(items) => {
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    std::fmt::Display::fmt(item, f)?;
                }
                Ok(())
            }
        }
    }
}

impl Tag {
    fn render(&self) -> &'static str {
        match self {
            Self::Html => "html",
            Self::Body => "body",
            Self::Nav => "nav",
            Self::Main => "main",
            Self::Aside => "aside",
            Self::Footer => "footer",
            Self::Section => "section",
            Self::Div => "div",
            Self::Span => "span",
            Self::P => "p",
            Self::A => "a",
            Self::Img => "img",
            Self::Hr => "hr",
            Self::Br => "br",
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
            Self::Ul => "ul",
            Self::Ol => "ol",
            Self::Li => "li",
            Self::Dl => "dl",
            Self::Dt => "dt",
            Self::Dd => "dd",
            Self::Table => "table",
            Self::Thead => "thead",
            Self::Tbody => "tbody",
            Self::Tr => "tr",
            Self::Th => "th",
            Self::Td => "td",
            Self::Pre => "pre",
            Self::Code => "code",
            Self::Strong => "strong",
            Self::Em => "em",
            Self::Blockquote => "blockquote",
            Self::Button => "button",
            Self::Svg => "svg",
            Self::Details => "details",
            Self::Summary => "summary",
            Self::Input => "input",
        }
    }
}

impl std::fmt::Display for Pseudo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hover => f.write_str("hover"),
            Self::Focus => f.write_str("focus"),
            Self::FocusVisible => f.write_str("focus-visible"),
            Self::FocusWithin => f.write_str("focus-within"),
            Self::FirstChild => f.write_str("first-child"),
            Self::NthChild(parity) => write!(f, "nth-child({})", parity.css()),
            Self::Active => f.write_str("active"),
        }
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(value) => write!(f, "[{}=\"{}\"]", self.name, value),
            None => write!(f, "[{}]", self.name),
        }
    }
}

impl Parity {
    #[must_use]
    pub fn css(&self) -> &'static str {
        match self {
            Self::Even => "even",
            Self::Odd => "odd",
        }
    }
}

impl From<reference::Reference> for Selector {
    fn from(reference: reference::Reference) -> Self {
        let words = reference.words();
        if words.len() == 1 {
            Self::Class(words[0].into())
        } else {
            Self::Compound(
                words
                    .iter()
                    .map(|w| Self::Class((*w).into()))
                    .collect::<Vec<_>>(),
            )
        }
    }
}
