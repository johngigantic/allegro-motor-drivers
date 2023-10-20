//! Derive macros to implement allegro_motor_drivers::io traits.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro to implement the allegro_motor_drivers::io::SpiMessages trait.
/// 
/// Please note that a compile failure will occur if derive macros are defined in the wrong order,
/// or if the bitsize derive macro is not used at all. This occurs because derive macros are parsed
/// from the outside in, and each macro depends on implementations defined by other macros.
/// ```compile_fail
/// #[derive(spi_derive)]
/// struct MyStruct {
///     field_1: bool,
/// }
/// ```
/// 
/// ```compile_fail
/// #[bitsize(1)]
/// #[derive(spi_derive)]
/// struct MyStruct {
///     field_1: bool,
/// }
/// ```
/// 
#[proc_macro_derive(spi_derive)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, attrs, .. } = parse_macro_input!(input as DeriveInput);

    let bitsize = match analyze_bitsize(attrs) {
        Ok(bitsize) => bitsize,
        Err(error_message) => return error_message,
    };
    let parity = has_parity_field(bitsize);
    let read_request_function = generate_read_request(parity);
    let read_response_function = generate_read_response(parity);
    let write_request_function = generate_write_request(parity);

    quote! {
        impl crate::io::SpiMessages for #ident {
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

fn analyze_bitsize(attrs: Vec<syn::Attribute>) -> Result<u16, TokenStream> {
    let mut bitsizes = attrs.iter().filter_map(|attr| {
        if attr.path().is_ident("bitsize") {
            let a: syn::LitInt = attr.parse_args().unwrap();
            Some(a.base10_parse::<u16>().unwrap())
        } else {
            None
        }
    });

    match bitsizes.next() {
        Some(bitsize) => Ok(bitsize),
        None => Err(
            quote! {
                compile_error!("Register must use the 'bitsize(n) attribute following the SpiMessages attribute.");
            }
            .into()
        )
    }
}

fn has_parity_field(bitsize: u16) -> bool {
    bitsize >= 9
}
