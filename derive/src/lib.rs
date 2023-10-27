//! Derive macros to implement allegro_motor_drivers::io traits.

#![feature(proc_macro_span)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod chips;

// #[proc_macro_derive(AllegroRegister)]
// pub fn allegro_derive(item: TokenStream) -> TokenStream {
//     let DeriveInput {ident, attrs, ..} = parse_macro_input!(item as DeriveInput);

//     let bitsize = analyze_bitsize(&attrs);

//     quote! {
//         impl crate::regs::AllegroRegister<bilge::prelude::UInt<usize, #bitsize>> for #ident {
//             fn get_value(&self) -> u16 {
//                 self.value.into()
//             }
        
//             fn set_value(&mut self, value: bilge::prelude::UInt<usize, #bitsize>) {
//                 self.value = value;
//             }
//         }
//     }.into()
// }

// fn analyze_bitsize(attrs: &Vec<syn::Attribute>) -> u16 {
//     let mut bitsizes = attrs.iter().filter_map(|attr| {
//         if attr.path().is_ident("bitsize") {
//             let a: syn::LitInt = attr.parse_args().unwrap();
//             Some(a.base10_parse::<u16>().unwrap())
//         } else {
//             None
//         }
//     });

//     match bitsizes.next() {
//         Some(bitsize) => bitsize,
//         None => panic!("'bitsize' not found in object attributes."),
//     }
// }


/// Derive macro to implement the allegro_motor_drivers::regs::AllegroRegister trait.
///
/// Please note that a compile failure will occur if derive macros are defined in the wrong order,
/// or if the bitsize derive macro is not used at all. This occurs because derive macros are parsed
/// from the outside in, and each macro depends on implementations defined by other macros.
/// ```compile_fail
/// #[allegro_register(chip = "a4910")]
/// struct MyStruct {
///     field_1: bool,
/// }
/// ```
///
/// ```compile_fail
/// #[bitsize(1)]
/// #[allegro_register(chip = "a4910")]
/// struct MyStruct {
///     field_1: bool,
/// }
/// ```
///
#[proc_macro_derive(AllegroRegister)]
pub fn allegro_derive(item: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(item as DeriveInput);

    let source_file = proc_macro::Span::call_site().source_file().clone().path();
    let chip_name = source_file.iter().nth(2).unwrap().to_str().unwrap();
    let chip = chips::CHIPS.into_iter().find(|chip| {chip.name == chip_name}).unwrap();
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

// fn analyze_chip(attrs: &[syn::Attribute]) -> Result<chips::Chip, syn::Error> {
//     let mut selected_chip = chips::Chip::default();

//     for attr in attrs.iter() {
//         if attr.path().is_ident("allegro_register") {
//             attr.parse_nested_meta(|meta| {
//                 if meta.path.is_ident("chip") {
//                     let value = meta.value()?;
//                     let str: syn::LitStr = value.parse()?;
//                     let valid_chip = chips::CHIPS
//                         .into_iter()
//                         .find(|chip| {*chip.name == str.value()})
//                         .ok_or_else(|| meta.error("Invalid chip name"))?;
//                     selected_chip = valid_chip;
//                     Ok(())
//                 } else {
//                     let error_msg = r#"Register must provide a chip identifier, e.g. '#[allegro_register(chip = "a4910"))]'"#;
//                     Err(meta.error(error_msg))
//                 }
//             })?;
//         }
//     }
//     Ok(selected_chip)
// }
