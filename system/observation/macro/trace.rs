use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{bracketed, Ident, Token};

pub struct Arguments {
    pub channels: Vec<channel::Specification>,
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Arguments { channels: vec![] });
        }

        let ident: Ident = input.parse()?;
        if ident != "channels" {
            return Err(syn::Error::new(ident.span(), "expected 'channels'"));
        }

        input.parse::<Token![=]>()?;

        let content;
        bracketed!(content in input);

        let specs: Punctuated<channel::Specification, Token![,]> =
            content.parse_terminated(channel::Specification::parse, Token![,])?;

        let channels = specs.into_iter().collect::<Vec<_>>();

        let mut seen = std::collections::HashSet::new();
        for spec in &channels {
            let name = spec.name.to_string();
            if !seen.insert(name.clone()) {
                return Err(syn::Error::new(spec.name.span(), "duplicate channel"));
            }
        }

        Ok(Arguments { channels })
    }
}
