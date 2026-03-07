use std::collections::HashMap;

use syn::{
    File, Ident, ItemFn, ItemMod, Signature,
    visit::{self, Visit},
};

pub struct Context {
    pub functions: HashMap<String, Signature>,
}

impl Context {
    #[must_use]
    pub fn from(ast: &File) -> Self {
        let mut collector = Collector::default();
        collector.visit_file(ast);

        Self {
            functions: collector.functions,
        }
    }
}

#[derive(Default)]
struct Collector {
    functions: HashMap<String, Signature>,
    module: Vec<Ident>,
}

impl Visit<'_> for Collector {
    fn visit_item_fn(&mut self, item: &ItemFn) {
        let name = item.sig.ident.to_string();
        let qualified = self.qualified(&name);

        self.functions.insert(qualified, item.sig.clone());

        visit::visit_item_fn(self, item);
    }

    fn visit_item_mod(&mut self, item: &ItemMod) {
        self.module.push(item.ident.clone());
        visit::visit_item_mod(self, item);
        self.module.pop();
    }
}

impl Collector {
    fn qualified(&self, name: &str) -> String {
        if self.module.is_empty() {
            name.to_string()
        } else {
            let path = self
                .module
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("::");
            format!("{path}::{name}")
        }
    }
}
