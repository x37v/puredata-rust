extern crate proc_macro;

use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Expr, ExprBlock, GenericParam, Generics, Ident, Item, ItemImpl,
    LitStr, Token, Type, Visibility,
};

struct Parsed {
    items: Vec<Item>,
}

impl Parse for Parsed {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }

        Ok(Self { items })
    }
}

#[proc_macro]
pub fn external(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Parsed { items } = parse_macro_input!(input as Parsed);

    let mut impls = Vec::new();
    let mut structs = Vec::new();
    let mut remain = Vec::new();

    for item in items.iter() {
        match item {
            Item::Struct(x) => structs.push(x),
            Item::Impl(x) => impls.push(x),
            _ => remain.push(item),
        }
    }

    let the_struct = structs.first().expect("couldn't find the struct");

    let name_ident = the_struct.clone();
    let struct_name = the_struct.ident.clone();

    let lower_name = struct_name.to_string().to_lowercase();
    let upper_name = struct_name.to_string().to_uppercase();
    let flat_name = lower_name.clone() + "_tilde";

    let class_name = LitStr::new(&(lower_name.clone() + "~"), name_ident.span());
    let class_object = Ident::new(&(upper_name + "_CLASS"), name_ident.span());
    let new_method = Ident::new(&(flat_name.clone() + "_new"), name_ident.span());
    let free_method = Ident::new(&(flat_name.clone() + "_free"), name_ident.span());
    let dsp_method = Ident::new(&(flat_name.clone() + "_dsp"), name_ident.span());
    let perform_method = Ident::new(&(flat_name.clone() + "_perform"), name_ident.span());
    let setup_method = Ident::new(&(flat_name.clone() + "_setup"), name_ident.span());

    let wrapped_class = quote! {
        //generated
        type Wrapped = SignalProcessorExternalWrapper<#struct_name>;
        static mut #class_object: Option<*mut puredata_sys::_class> = None;
    };

    let alloc_trampolines = quote! {
        //new trampoline
        pub unsafe extern "C" fn #new_method () -> *mut ::std::os::raw::c_void {
            Wrapped::new(#class_object.expect("hello dsp class not set"))
        }

        //free trampoline
        pub unsafe extern "C" fn #free_method (x: *mut Wrapped) {
            let x = &mut *x;
            x.free();
        }
    };

    let dsp_trampolines = quote! {
        pub unsafe extern "C" fn #dsp_method(
            x: *mut Wrapped,
            sp: *mut *mut puredata_sys::t_signal,
            ) {
            let x = &mut *x;
            x.dsp(sp, #perform_method);
        }

        pub unsafe extern "C" fn #perform_method(
            w: *mut puredata_sys::t_int,
            ) -> *mut puredata_sys::t_int {
            //actually longer than 2 but .offset(1) didn't seem to work correctly
            //but slice does
            let x = std::slice::from_raw_parts(w, 2);
            let x = &mut *std::mem::transmute::<_, *mut Wrapped>(x[1]);
            x.perform(w)
        }
    };

    let mut register_methods = quote! {
        c.add_method(Method::Bang(hellodsp_tilde_bang_trampoline));
    };

    register_methods = quote! {
        #register_methods
        let name = CString::new("blah").expect("CString::new failed");
        c.add_method(Method::SelF1(name, hellodsp_tilde_float_trampoline, 1));
    };

    let setup_trampoline = quote! {
        #[no_mangle]
        pub unsafe extern "C" fn #setup_method() {
            let name = CString::new(#class_name).expect("CString::new failed");
            let mut c = Class::<Wrapped>::register_dsp_new(
                name,
                #new_method,
                SignalClassType::WithInput(
                    #dsp_method,
                    Wrapped::float_convert_field_offset(),
                ),
                Some(#free_method),
            );
            #register_methods
            #class_object = Some(c.into());
        }
    };

    let expanded = quote! {
        #the_struct

        #(#impls)*

        #wrapped_class

        #setup_trampoline

        #alloc_trampolines

        #dsp_trampolines

        #(#remain)*
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
