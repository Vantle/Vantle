use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tag {
    elements: BTreeMap<&'static str, usize>,
}

impl Tag {
    #[must_use]
    pub fn new(name: &'static str) -> Self {
        Self {
            elements: BTreeMap::from([(name, 1)]),
        }
    }

    #[must_use]
    pub fn tag(mut self, name: &'static str) -> Self {
        *self.elements.entry(name).or_insert(0) += 1;
        self
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for (&name, &count) in &self.elements {
            if !first {
                f.write_str(".")?;
            }
            first = false;
            f.write_str(name)?;
            if count > 1 {
                write!(f, " x {count}")?;
            }
        }
        Ok(())
    }
}
