use language::Language;

pub struct Extraction {
    pub name: &'static str,
    pub content: &'static str,
    pub start: usize,
    pub end: usize,
    pub language: Language,
}

impl Extraction {
    #[must_use]
    pub fn source(&self) -> miette::NamedSource<&'static str> {
        miette::NamedSource::new(self.name, self.content)
    }

    #[must_use]
    pub fn span(&self) -> miette::SourceSpan {
        miette::SourceSpan::new(self.start.into(), self.end - self.start)
    }
}

pub trait Query {
    fn one(&self) -> &Extraction;
    fn at(&self, index: usize) -> &Extraction;
}

impl Query for [Extraction] {
    fn one(&self) -> &Extraction {
        <[Extraction]>::first(self).expect("extraction contains no results")
    }

    fn at(&self, index: usize) -> &Extraction {
        <[Extraction]>::get(self, index).unwrap_or_else(|| {
            panic!(
                "extraction index {index} out of bounds (length {})",
                self.len()
            )
        })
    }
}
