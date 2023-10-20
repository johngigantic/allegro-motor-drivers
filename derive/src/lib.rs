//! A derive macro to set up registers for parsing in and out of SPI messages

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AllegroRegister)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, attrs, .. } = parse_macro_input!(input as DeriveInput);

    let bitsize = analyze_bitsize(attrs);
    let parity = has_parity_field(bitsize);

    let read_request_function = generate_read_request(parity);
    let read_response_function = generate_read_response(parity);
    let write_request_function = generate_write_request(parity);

    quote! {
        impl allegro_motor_drivers::io::AllegroRegister for #ident {
            #read_request_function
            #read_response_function
            #write_request_function
        }
    }
    .into()
}

fn generate_read_request(_parity: bool) -> proc_macro2::TokenStream {
    quote! {
        fn read_request(&self) -> u16 {
            0
        }
    }
}

fn generate_read_response(_parity: bool) -> proc_macro2::TokenStream {
    quote! {
        fn read_response(&mut self, value: u16) {

        }
    }
}

fn generate_write_request(_parity: bool) -> proc_macro2::TokenStream {
    quote! {
        fn write_request(&self) -> u16 {
            0
        }
    }
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

fn has_parity_field(bitsize: u16) -> bool {
    bitsize >= 9
}
