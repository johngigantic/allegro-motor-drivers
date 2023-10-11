//! A derive macro to set up registers for parsing in and out of SPI messages

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(AllegroRegister)]
pub fn derive(input: TokenStream) -> TokenStream {
    quote! {}.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
