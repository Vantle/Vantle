use std::fmt::Write;

use element::{Element, Location};
use language::Language;

struct Compact {
    html: String,
}

impl Compact {
    fn attributes(&mut self, name: &str, pairs: &[(String, String)]) {
        self.html.push('<');
        self.html.push_str(name);
        for (key, value) in pairs {
            write!(self.html, " {key}=\"").unwrap();
            escape::stream(&mut self.html, value);
            self.html.push('"');
        }
        self.html.push('>');
    }
}

impl render::Emitter for Compact {
    fn open(&mut self, name: &str, attributes: &[(String, String)]) -> miette::Result<()> {
        self.attributes(name, attributes);
        Ok(())
    }

    fn close(&mut self, name: &str) -> miette::Result<()> {
        write!(self.html, "</{name}>").unwrap();
        Ok(())
    }

    fn void(&mut self, name: &str, attributes: &[(String, String)]) -> miette::Result<()> {
        self.attributes(name, attributes);
        Ok(())
    }

    fn text(&mut self, content: &str) -> miette::Result<()> {
        escape::stream(&mut self.html, content);
        Ok(())
    }

    fn raw(&mut self, content: &str) -> miette::Result<()> {
        self.html.push_str(content);
        Ok(())
    }

    fn code(
        &mut self,
        content: &str,
        _language: Language,
        _location: Option<&Location>,
    ) -> miette::Result<()> {
        escape::stream(&mut self.html, content);
        Ok(())
    }
}

pub fn fragment(elements: &[Element]) -> miette::Result<String> {
    let mut emitter = Compact {
        html: String::new(),
    };
    render::render(&mut emitter, elements)?;
    Ok(emitter.html)
}
