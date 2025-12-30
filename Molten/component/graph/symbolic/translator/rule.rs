pub use filter;
pub use terminate;

pub type Function<Element> = Box<dyn Fn(Element) -> bool + 'static>;
pub type Lambda<Element> = dyn Fn(Element) -> bool + 'static;

pub struct Rules<Element> {
    pub filter: Function<Element>,
    pub terminator: Function<Element>,
    pub limiter: Option<usize>,
}

impl<Element> Rules<Element> {
    #[must_use]
    pub fn filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(Element) -> bool + 'static,
    {
        self.filter = Box::new(filter);
        self
    }

    #[must_use]
    pub fn terminator<T>(mut self, terminator: T) -> Self
    where
        T: Fn(Element) -> bool + 'static,
    {
        self.terminator = Box::new(terminator);
        self
    }

    #[must_use]
    pub fn limiter(mut self, limiter: usize) -> Self {
        self.limiter = Some(limiter);
        self
    }
}

impl<Element> Default for Rules<Element> {
    fn default() -> Self {
        Self {
            filter: filter::none(),
            terminator: terminate::none(),
            limiter: None,
        }
    }
}

#[must_use]
pub fn glyph() -> Function<u8> {
    Box::new(|element: u8| !element.is_ascii_whitespace())
}

#[must_use]
pub fn is(value: u8) -> Function<u8> {
    Box::new(move |element: u8| value == element)
}

#[must_use]
pub fn not(value: u8) -> Function<u8> {
    Box::new(move |element: u8| value != element)
}
