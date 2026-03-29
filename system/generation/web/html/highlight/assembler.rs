use element::Element;

pub struct Assembler {
    stack: Vec<Vec<Element>>,
    error: Option<error::Error>,
}

impl Default for Assembler {
    fn default() -> Self {
        Self {
            stack: vec![Vec::new()],
            error: None,
        }
    }
}

impl Assembler {
    pub fn push(&mut self, element: Element) {
        if let Some(top) = self.stack.last_mut() {
            top.push(element);
        }
    }

    pub fn extend(&mut self, elements: Vec<Element>) {
        if let Some(top) = self.stack.last_mut() {
            top.extend(elements);
        }
    }

    pub fn open(&mut self) {
        self.stack.push(Vec::new());
    }

    pub fn close(&mut self) -> Vec<Element> {
        if let Some(children) = self.stack.pop() {
            children
        } else {
            self.error = Some(error::Error::Stack);
            Vec::new()
        }
    }

    pub fn finish(&mut self) -> error::Result<Vec<Element>> {
        if let Some(error) = self.error.take() {
            return Err(error);
        }
        Ok(self.stack.pop().unwrap_or_default())
    }
}
