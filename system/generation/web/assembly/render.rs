use element::{Element, Location};
use error::Error;
use language::Language;
use render::Emitter;

pub struct Document {
    handle: web_sys::Document,
    stack: Vec<web_sys::Element>,
}

impl Document {
    fn parent(&self) -> miette::Result<&web_sys::Element> {
        self.stack.last().ok_or(Error::Stack.into())
    }

    fn create(
        &self,
        name: &str,
        attributes: &[(String, String)],
    ) -> miette::Result<web_sys::Element> {
        let element = self
            .handle
            .create_element(name)
            .map_err(|_| Error::Element { name: name.into() })?;
        for (key, value) in attributes {
            element
                .set_attribute(key, value)
                .map_err(|_| Error::Attribute { key: key.clone() })?;
        }
        Ok(element)
    }

    fn append(&self, node: &web_sys::Node) -> miette::Result<()> {
        self.parent()?
            .append_child(node)
            .map_err(|_| Error::Append)?;
        Ok(())
    }

    fn insert(&self, content: &str) -> miette::Result<()> {
        let node = self.handle.create_text_node(content);
        self.append(&node)
    }
}

impl Emitter for Document {
    fn open(&mut self, name: &str, attributes: &[(String, String)]) -> miette::Result<()> {
        let element = self.create(name, attributes)?;
        self.append(&element)?;
        self.stack.push(element);
        Ok(())
    }

    fn close(&mut self, _name: &str) -> miette::Result<()> {
        self.stack.pop();
        Ok(())
    }

    fn void(&mut self, name: &str, attributes: &[(String, String)]) -> miette::Result<()> {
        let element = self.create(name, attributes)?;
        self.append(&element)
    }

    fn text(&mut self, content: &str) -> miette::Result<()> {
        self.insert(content)
    }

    fn raw(&mut self, content: &str) -> miette::Result<()> {
        let container = self
            .handle
            .create_element("div")
            .map_err(|_| Error::Element { name: "div".into() })?;
        container.set_inner_html(content);
        let parent = self.parent()?;
        while let Some(child) = container.first_child() {
            parent.append_child(&child).map_err(|_| Error::Append)?;
        }
        Ok(())
    }

    fn code(
        &mut self,
        content: &str,
        language: Language,
        _location: Option<&Location>,
    ) -> miette::Result<()> {
        let block = self.create(
            "div",
            &[
                ("class".into(), "code-block".into()),
                ("data-language".into(), language.name().into()),
            ],
        )?;
        let text = self.handle.create_text_node(content);
        block.append_child(&text).map_err(|_| Error::Append)?;
        self.append(&block)
    }
}

pub fn emit(
    document: &web_sys::Document,
    parent: &web_sys::Element,
    elements: &[Element],
) -> miette::Result<()> {
    let mut emitter = Document {
        handle: document.clone(),
        stack: vec![parent.clone()],
    };
    render::render(&mut emitter, elements)
}
