//! A derive macro to implement `allegro_motor_drivers::regs::AllegroRegister` trait.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro to implement the `AllegroRegister` trait.
///
/// See the [original trait][trait] for examples and complete documentation.
///
/// [trait]: ../allegro_motor_drivers/regs/trait.AllegroRegister.html
///
/// Please note that a compile failure will occur if derive macros are defined in the wrong order,
/// or if the bitsize derive macro is not used at all. This occurs because derive macros are parsed
/// from the outside in, and each macro depends on implementations defined by other macros.
///
/// ```compile_fail
/// # #[macro_use] extern crate allegro_motor_derive;
/// # use bilge::prelude::*;
/// #[derive(AllegroRegister)]
/// struct WithoutBitsize {
///     field_1: bool,
/// }
/// ```
///
/// ```compile_fail
/// # #[macro_use] extern crate allegro_motor_derive;
/// # use bilge::prelude::*;
/// #[bitsize(1)]
/// #[derive(AllegroRegister)]
/// struct WrongMacroOrder {
///     field_1: bool,
/// }
/// ```
///
#[proc_macro_derive(AllegroRegister)]
pub fn allegro_derive(item: TokenStream) -> TokenStream {
    let DeriveInput { ident, attrs, .. } = parse_macro_input!(item as DeriveInput);

    let bitsize = match analyze_bitsize(&attrs) {
        Ok(bitsize) => proc_macro2::Literal::u16_unsuffixed(bitsize),
        Err(error_msg) => return error_msg.into_compile_error().into(),
    };

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

fn analyze_bitsize(attrs: &[syn::Attribute]) -> Result<u16, syn::Error> {
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
        None => Err(syn::Error::new_spanned(
            attrs.iter().next(),
            "'bitsize' not found in object attributes.",
        )),
    }
}
