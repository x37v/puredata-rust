extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Expr, GenericParam, Generics, Ident, Type, Visibility};

#[proc_macro]
pub fn external_processor(input: TokenStream) -> proc_macro::TokenStream {
    let expanded = quote! {};
    proc_macro::TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
