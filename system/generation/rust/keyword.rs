pub struct Result {
    pub key: &'static str,
    pub variable: syn::Ident,
}

pub struct Parameters {
    pub key: &'static str,
    pub variable: syn::Ident,
}

#[must_use]
pub fn result() -> Result {
    Result {
        key: "()",
        variable: syn::Ident::new("result", proc_macro2::Span::call_site()),
    }
}

#[must_use]
pub fn parameters() -> Parameters {
    Parameters {
        key: "parameters",
        variable: syn::Ident::new("parameters", proc_macro2::Span::call_site()),
    }
}
