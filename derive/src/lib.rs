//! Derive macros to implement allegro_motor_drivers::io traits.

#![feature(proc_macro_span)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod chips;

#[proc_macro_derive(Messages)]
pub fn spi_message_derive(item: TokenStream) -> TokenStream {
    let DeriveInput {ident, ..} = parse_macro_input!(item as DeriveInput);

    let source_file = proc_macro::Span::call_site().source_file().clone().path();
    let chip_name = source_file.iter().nth(2).unwrap().to_str().unwrap();
    let chip = chips::CHIPS.into_iter().find(|chip| {chip.name == chip_name}).unwrap();

    let read_request_function = generate_read_request(&chip);
    let read_response_function = generate_read_response(&chip);
    let write_request_function = generate_write_request(&chip);

    quote! {
        impl crate::io::spi::Messages for #ident {
            #read_request_function
            #read_response_function
            #write_request_function
        }
    }.into()
}

// /// Derive macro to implement the allegro_motor_drivers::io::SpiMessages trait.
// ///
// /// Please note that a compile failure will occur if derive macros are defined in the wrong order,
// /// or if the bitsize derive macro is not used at all. This occurs because derive macros are parsed
// /// from the outside in, and each macro depends on implementations defined by other macros.
// /// ```compile_fail
// /// #[allegro_register(chip = "a4910")]
// /// struct MyStruct {
// ///     field_1: bool,
// /// }
// /// ```
// ///
// /// ```compile_fail
// /// #[bitsize(1)]
// /// #[allegro_register(chip = "a4910")]
// /// struct MyStruct {
// ///     field_1: bool,
// /// }
// /// ```
// ///
// #[proc_macro_attribute]
// pub fn allegro_register(macro_args: TokenStream, item: TokenStream) -> TokenStream {
//     // let original_struct = proc_macro2::TokenStream::from(item.clone());
//     let DeriveInput { attrs, .. } = parse_macro_input!(macro_args as DeriveInput);
//     let original_struct = parse_macro_input!(item as DeriveInput);
//     let ident = syn::Ident::new(&original_struct.ident.to_string(), proc_macro2::Span::call_site());


//     let chip = match analyze_chip(&attrs) {
//         Ok(chip_name) => chip_name,
//         Err(error_message) => return error_message.to_compile_error().into(),
//     };

//     let read_request_function = generate_read_request(chip.register_size, chip.parity);
//     let read_response_function = generate_read_response(chip.register_size, chip.parity);
//     let write_request_function = generate_write_request(chip.register_size, chip.parity);

//     quote! {
//         #original_struct

//         impl crate::io::spi::Messages for #ident {
//             #read_request_function
//             #read_response_function
//             #write_request_function
//         }
//     }
//     .into()
// }

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

fn generate_read_request(chip: &chips::Chip) -> proc_macro2::TokenStream {
    let address_size: u16 = 16 - (1 + chip.register_size + (chip.parity as u16));

    quote! {
        fn read_request(&self) -> u16 {
            return 0;
        }
    }
}

fn generate_read_response(chip: &chips::Chip) -> proc_macro2::TokenStream {
    quote! {
        fn read_response(&mut self, value: u16) {

        }
    }
}

fn generate_write_request(chip: &chips::Chip) -> proc_macro2::TokenStream {
    quote! {
        fn write_request(&self) -> u16 {
            0
        }
    }
}
