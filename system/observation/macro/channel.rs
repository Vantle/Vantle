use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Token};

pub struct Specification {
    pub name: Ident,
    pub weight: u8,
}

impl Parse for Specification {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let weight = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            let lit: LitInt = input.parse()?;
            lit.base10_parse()?
        } else {
            1
        };
        Ok(Specification { name, weight })
    }
}
