#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Expr, ExprBlock, GenericParam, Generics, Ident, Token, Type,
    Visibility,
};

struct ProcessorExternal {
    struct_name: Ident,
    struct_contents: ExprBlock,
}
impl Parse for ProcessorExternal {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![pub]>()?;
        input.parse::<Token![struct]>()?;
        let struct_name: Ident = input.parse()?;
        let struct_contents: ExprBlock = input.parse()?;
        Ok(ProcessorExternal {
            struct_name,
            struct_contents,
        })
    }
}

#[proc_macro]
pub fn external_processor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ProcessorExternal {
        struct_name,
        struct_contents,
    } = parse_macro_input!(input as ProcessorExternal);

    let pdname = struct_name.to_string().to_lowercase();
    let flat_name = pdname.clone() + "_tilde";

    let class_name = Ident::new(&(pdname.to_uppercase() + "_CLASS"), Span::call_site());
    let new_method = Ident::new(&(flat_name.clone() + "_new"), Span::call_site());
    let free_method = Ident::new(&(flat_name.clone() + "_free"), Span::call_site());

    let expanded = quote! {
        //original struct
        pub struct #struct_name #struct_contents

        //generated
        type Wrapped = SignalProcessorExternalWrapper<#struct_name>;
        static mut #class_name: Option<*mut puredata_sys::_class> = None;

        //new trampoline
        pub unsafe extern "C" fn #new_method () -> *mut ::std::os::raw::c_void {
            Wrapped::new(#class_name.expect("hello dsp class not set"))
        }

        //free trampoline
        pub unsafe extern "C" fn #free_method (x: *mut Wrapped) {
            let x = &mut *x;
            x.free();
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
