use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};
use trace as arguments;

#[proc_macro_attribute]
pub fn trace(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as arguments::Arguments);
    let input = parse_macro_input!(item as ItemFn);

    let mut specs = args
        .channels
        .iter()
        .map(|c| (c.name.to_string(), c.weight))
        .collect::<Vec<_>>();
    specs.sort_by(|a, b| a.0.cmp(&b.0));

    let channels = specs
        .iter()
        .map(|(name, weight)| format!("{name}:{weight}"))
        .collect::<Vec<_>>()
        .join(",");

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;

    let expanded = quote! {
        #(#attrs)*
        #[::tracing::instrument(level = "debug", skip_all, fields(channels = #channels))]
        #vis #sig #block
    };

    TokenStream::from(expanded)
}
