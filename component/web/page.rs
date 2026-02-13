use body::Body;
use element::Element;

pub struct Page {
    pub title: String,
    pub stylesheet: Option<String>,
    pub wasm: Option<String>,
    pub context: Option<String>,
    pub identifier: Option<String>,
    pub root: Option<String>,
    pub body: Vec<Element>,
}

impl Page {
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: String::new(),
            stylesheet: None,
            wasm: None,
            context: None,
            identifier: None,
            root: None,
            body: Vec::new(),
        }
    }

    #[must_use]
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.into();
        self
    }

    #[must_use]
    pub fn stylesheet(mut self, path: &str) -> Self {
        self.stylesheet = Some(path.into());
        self
    }

    #[must_use]
    pub fn wasm(mut self, path: &str) -> Self {
        self.wasm = Some(path.into());
        self
    }

    #[must_use]
    pub fn context(mut self, value: &str) -> Self {
        self.context = Some(value.into());
        self
    }

    #[must_use]
    pub fn identifier(mut self, value: &str) -> Self {
        self.identifier = Some(value.into());
        self
    }

    #[must_use]
    pub fn root(mut self, value: &str) -> Self {
        self.root = Some(value.into());
        self
    }

    #[must_use]
    pub fn body(mut self, f: impl FnOnce(Body) -> Body) -> Self {
        let body = f(Body::new());
        self.body = body.elements;
        self
    }

    #[must_use]
    pub fn compose(self, f: impl FnOnce(Page) -> Page) -> Self {
        f(self)
    }
}

impl Default for Page {
    fn default() -> Self {
        Self::new()
    }
}
