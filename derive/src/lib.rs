//! Derive macros to implement allegro_motor_drivers::io traits.

#![feature(proc_macro_span)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod chips;

/// Derive macro to implement the allegro_motor_drivers::regs::AllegroRegister trait.
///
#[proc_macro_derive(AllegroRegister)]
pub fn allegro_derive(item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(item as DeriveInput);

    let source_file = proc_macro::Span::call_site().source_file().clone().path();
    let chip_name = source_file.iter().nth(2).unwrap().to_str().unwrap();
    let chip = chips::CHIPS
        .into_iter()
        .find(|chip| chip.name == chip_name)
        .unwrap();
    let bitsize = proc_macro2::Literal::u16_unsuffixed(chip.register_size);

    quote! {
        impl crate::regs::AllegroRegister<bilge::prelude::UInt<u16, #bitsize>> for #ident {
            fn get_value(&self) -> u16 {
                self.value.into()
            }

            fn set_value(&mut self, value: bilge::prelude::UInt<u16, #bitsize>) {
                self.value = value;
            }
        }
    }
    .into()
}
