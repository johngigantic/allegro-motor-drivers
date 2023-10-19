//! A derive macro to set up registers for parsing in and out of SPI messages

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AllegroRegister)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, attrs, .. } = parse_macro_input!(input as DeriveInput);

    let _bitsize = analyze_bitsize(attrs);

    quote! {
        impl allegro_motor_drivers::io::AllegroRegister for #ident {
            fn read_request(&self) -> u16 {
                0
            }

            fn read_response(&mut self, value: u16) {

            }

            fn write_request(&self) -> u16 {
                0
            }
        }
    }
    .into()
}

fn analyze_bitsize(attrs: Vec<syn::Attribute>) -> u16 {
    let mut bitsizes = attrs.iter().filter_map(|attr| {
        if attr.path().is_ident("bitsize") {
            let a: syn::LitInt = attr.parse_args().unwrap();
            Some(a.base10_parse::<u16>().unwrap())
        } else {
            None
        }
    });

    match bitsizes.next() {
        Some(bitsize) => bitsize,
        None => panic!("'bitsize' not found in object attributes."),
    }
}

fn _has_parity_field(bitsize: u16) -> bool {
    bitsize >= 9
}
