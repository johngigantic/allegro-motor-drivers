//! A derive macro to set up registers for parsing in and out of SPI messages

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AllegroRegister)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data
    } = parse_macro_input!(input as DeriveInput);

    let bitsize = analyze_bitsize(attrs).unwrap();

    quote! {}.into()
}

fn analyze_bitsize(attrs: Vec<syn::Attribute>) -> Option<u16> {
    for attr in attrs {
        if attr.path().is_ident("bitsize") {
            let a: syn::LitInt = attr.parse_args().unwrap();
            let value = a.base10_parse::<u16>().unwrap();
            return Some(value);
        }
    }
    None
}
