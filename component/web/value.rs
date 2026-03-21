pub enum Value {
    Token(Token),
    Calculation(Calculation),
    Concrete(Concrete),
    Keyword(Keyword),
    Composite(Vec<Value>),
    Literal(String),
}

pub enum Token {
    Scale(i32),
    Half(i32),
    Palette(Palette),
    Custom(String),
}

pub enum Palette {
    Text,
    Secondary,
    Background,
    Code,
    CodeText,
    Accent,
    Hover,
    Border,
    Navigation,
    Stripe,
    Keyword,
    Entity,
    Literal,
    Comment,
    Constant,
    Storage,
    Punctuation,
    Variable,
    Function,
    Operator,
    Macro,
}

pub enum Concrete {
    Zero,
    Rem(f32),
    Px(i32),
    Percent(f32),
    Em(f32),
    Seconds(f32),
    Unitless(f32),
    Integer(i32),
}

pub enum Keyword {
    Auto,
    None,
    Inherit,
    Initial,
    Transparent,
    Current,
}

pub struct Calculation {
    terms: Vec<Term>,
}

struct Term {
    operation: Operation,
    value: Value,
}

enum Operation {
    Identity,
    Add,
    Subtract,
}

impl Value {
    #[must_use]
    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Token(token) => std::fmt::Display::fmt(token, f),
            Self::Calculation(calculation) => std::fmt::Display::fmt(calculation, f),
            Self::Concrete(concrete) => std::fmt::Display::fmt(concrete, f),
            Self::Keyword(keyword) => f.write_str(keyword.render()),
            Self::Composite(values) => {
                for (i, value) in values.iter().enumerate() {
                    if i > 0 {
                        f.write_str(" ")?;
                    }
                    std::fmt::Display::fmt(value, f)?;
                }
                Ok(())
            }
            Self::Literal(literal) => f.write_str(literal),
        }
    }
}

impl Token {
    #[must_use]
    pub fn scale(n: i32) -> Value {
        Value::Token(Self::Scale(n))
    }

    #[must_use]
    pub fn half(n: i32) -> Value {
        Value::Token(Self::Half(n))
    }

    #[must_use]
    pub fn palette(palette: Palette) -> Value {
        Value::Token(Self::Palette(palette))
    }

    #[must_use]
    pub fn custom(name: &str) -> Value {
        Value::Token(Self::Custom(name.into()))
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scale(n) => write!(f, "var(--scale-{})", Label(*n)),
            Self::Half(n) => write!(f, "var(--scale-{}h)", Label(*n)),
            Self::Palette(palette) => write!(f, "var(--{})", palette.render()),
            Self::Custom(name) => write!(f, "var(--{name})"),
        }
    }
}

struct Label(i32);

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 < 0 {
            write!(f, "n{}", self.0.abs())
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl Palette {
    fn render(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Secondary => "text-secondary",
            Self::Background => "background",
            Self::Code => "code-background",
            Self::CodeText => "code-text",
            Self::Accent => "accent",
            Self::Hover => "accent-hover",
            Self::Border => "border",
            Self::Navigation => "nav-background",
            Self::Stripe => "table-stripe",
            Self::Keyword => "syntax-keyword",
            Self::Entity => "syntax-entity",
            Self::Literal => "syntax-string",
            Self::Comment => "syntax-comment",
            Self::Constant => "syntax-constant",
            Self::Storage => "syntax-storage",
            Self::Punctuation => "syntax-punctuation",
            Self::Variable => "syntax-variable",
            Self::Function => "syntax-function",
            Self::Operator => "syntax-operator",
            Self::Macro => "syntax-macro",
        }
    }
}

impl Concrete {
    #[must_use]
    pub fn zero() -> Value {
        Value::Concrete(Self::Zero)
    }

    #[must_use]
    pub fn rem(n: f32) -> Value {
        Value::Concrete(Self::Rem(n))
    }

    #[must_use]
    pub fn px(n: i32) -> Value {
        Value::Concrete(Self::Px(n))
    }

    #[must_use]
    pub fn percent(n: f32) -> Value {
        Value::Concrete(Self::Percent(n))
    }

    #[must_use]
    pub fn em(n: f32) -> Value {
        Value::Concrete(Self::Em(n))
    }

    #[must_use]
    pub fn seconds(n: f32) -> Value {
        Value::Concrete(Self::Seconds(n))
    }

    #[must_use]
    pub fn unitless(n: f32) -> Value {
        Value::Concrete(Self::Unitless(n))
    }

    #[must_use]
    pub fn integer(n: i32) -> Value {
        Value::Concrete(Self::Integer(n))
    }
}

impl std::fmt::Display for Concrete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => f.write_str("0"),
            Self::Rem(n) => write!(f, "{n}rem"),
            Self::Px(n) => write!(f, "{n}px"),
            Self::Percent(n) => write!(f, "{n}%"),
            Self::Em(n) => write!(f, "{n}em"),
            Self::Seconds(n) => write!(f, "{n}s"),
            Self::Unitless(n) => write!(f, "{n}"),
            Self::Integer(n) => write!(f, "{n}"),
        }
    }
}

impl Keyword {
    fn render(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::None => "none",
            Self::Inherit => "inherit",
            Self::Initial => "initial",
            Self::Transparent => "transparent",
            Self::Current => "currentColor",
        }
    }
}

impl Calculation {
    #[must_use]
    pub fn start<V>(value: V) -> Self
    where
        V: Into<Value>,
    {
        Self {
            terms: vec![Term {
                operation: Operation::Identity,
                value: value.into(),
            }],
        }
    }

    #[must_use]
    pub fn plus<V>(mut self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.terms.push(Term {
            operation: Operation::Add,
            value: value.into(),
        });
        self
    }

    #[must_use]
    pub fn minus<V>(mut self, value: V) -> Self
    where
        V: Into<Value>,
    {
        self.terms.push(Term {
            operation: Operation::Subtract,
            value: value.into(),
        });
        self
    }
}

impl std::fmt::Display for Calculation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("calc(")?;
        for (index, term) in self.terms.iter().enumerate() {
            if index > 0 {
                match term.operation {
                    Operation::Add => f.write_str(" + ")?,
                    Operation::Subtract => f.write_str(" - ")?,
                    Operation::Identity => {}
                }
            }
            std::fmt::Display::fmt(&term.value, f)?;
        }
        f.write_str(")")
    }
}

impl From<Calculation> for Value {
    fn from(calculation: Calculation) -> Self {
        Self::Calculation(calculation)
    }
}

impl From<Keyword> for Value {
    fn from(keyword: Keyword) -> Self {
        Self::Keyword(keyword)
    }
}

impl From<&str> for Value {
    fn from(literal: &str) -> Self {
        Self::Literal(literal.into())
    }
}

impl From<&String> for Value {
    fn from(literal: &String) -> Self {
        Self::Literal(literal.clone())
    }
}

impl From<String> for Value {
    fn from(literal: String) -> Self {
        Self::Literal(literal)
    }
}

impl<V1, V2> From<(V1, V2)> for Value
where
    V1: Into<Value>,
    V2: Into<Value>,
{
    fn from((a, b): (V1, V2)) -> Self {
        Self::Composite(vec![a.into(), b.into()])
    }
}

impl<V1, V2, V3> From<(V1, V2, V3)> for Value
where
    V1: Into<Value>,
    V2: Into<Value>,
    V3: Into<Value>,
{
    fn from((a, b, c): (V1, V2, V3)) -> Self {
        Self::Composite(vec![a.into(), b.into(), c.into()])
    }
}

impl<V1, V2, V3, V4> From<(V1, V2, V3, V4)> for Value
where
    V1: Into<Value>,
    V2: Into<Value>,
    V3: Into<Value>,
    V4: Into<Value>,
{
    fn from((a, b, c, d): (V1, V2, V3, V4)) -> Self {
        Self::Composite(vec![a.into(), b.into(), c.into(), d.into()])
    }
}

#[must_use]
pub fn composite(values: Vec<Value>) -> Value {
    Value::Composite(values)
}
