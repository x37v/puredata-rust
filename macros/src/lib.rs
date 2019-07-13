extern crate proc_macro;

use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Expr, ExprBlock, GenericParam, Generics, Ident, Item, ItemImpl,
    ItemStruct, LitStr, Token, Type, Visibility,
};

#[derive(Copy, Clone, Debug)]
enum ExternalType {
    Control,
    Signal,
}

struct Parsed {
    items: Vec<Item>,
}

impl Parse for Parsed {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }

        Ok(Self { items })
    }
}

fn get_type(
    the_struct: &ItemStruct,
    impls: &Vec<&ItemImpl>,
) -> Result<(ExternalType, &'static str), &'static str> {
    let type_traits = [
        (ExternalType::Control, &"ControlExternal"),
        (ExternalType::Signal, &"SignalGeneratorExternal"),
        (ExternalType::Signal, &"SignalProcessorExternal"),
    ];
    //look through the impls and see if we find a matching one, return it and the ExternalType
    for i in impls {
        if let Some((_, p, _)) = &i.trait_ {
            if let Some(type_trait) = type_traits
                .iter()
                .find(|x| p.segments.last().unwrap().value().ident == x.1)
            {
                match i.self_ty.as_ref() {
                    Type::Path(tp) => {
                        //matches struct
                        if tp.path.is_ident(the_struct.ident.clone()) {
                            return Ok((type_trait.0, type_trait.1));
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    Err("Couldn't find External Implementation")
}

//return the class initialization item
fn add_control(new_method: &Ident, free_method: &Ident) -> proc_macro2::TokenStream {
    quote! {
        Class::<Wrapped>::register_new(name, #new_method, #free_method);
    }
}

//return the class initialization item
fn add_dsp(
    trampolines: &mut Vec<proc_macro2::TokenStream>,
    new_method: &Ident,
    free_method: &Ident,
    flat_name: &String,
    name_span: Span,
) -> proc_macro2::TokenStream {
    let dsp_method = Ident::new(&(flat_name.clone() + "_dsp"), name_span.clone());
    let perform_method = Ident::new(&(flat_name.clone() + "_perform"), name_span.clone());

    trampolines.push(quote! {
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
    });

    quote! {
        Class::<Wrapped>::register_dsp_new(
            name,
            #new_method,
            SignalClassType::WithInput( #dsp_method, Wrapped::float_convert_field_offset(),),
            Some(#free_method),);
    }
}

#[proc_macro]
pub fn external(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Parsed { items } = parse_macro_input!(input as Parsed);

    let mut impls = Vec::new();
    let mut structs = Vec::new();
    let mut remain = Vec::new();
    let mut trampolines = Vec::new();

    for item in items.iter() {
        match item {
            Item::Struct(x) => structs.push(x),
            Item::Impl(x) => impls.push(x),
            _ => remain.push(item),
        }
    }

    let the_struct = structs.first().expect("couldn't find the struct");
    let (etype, extern_impl) = get_type(&the_struct, &impls).unwrap();

    let name_ident = the_struct.clone();
    let struct_name = the_struct.ident.clone();

    //build up names
    let lower_name = struct_name.to_string().to_lowercase();
    let upper_name = struct_name.to_string().to_uppercase();
    let (flat_name, class_name) = match etype {
        ExternalType::Signal => (format!("{:}_tilde", lower_name), format!("{}~", lower_name)),
        ExternalType::Control => (lower_name.clone(), lower_name.clone()),
    };

    let class_name = LitStr::new(&(class_name), name_ident.span());
    let class_static = Ident::new(&(format!("{:}_CLASS", upper_name)), name_ident.span());
    let class_inst = Ident::new("c", name_ident.span());
    let new_method = Ident::new(&(format!("{:}_new", flat_name)), name_ident.span());
    let free_method = Ident::new(&(format!("{:}_free", flat_name)), name_ident.span());
    let setup_method = Ident::new(&(format!("{:}_setup", flat_name)), name_ident.span());
    let wrapper_type = Ident::new(&(format!("{:}Wrapper", extern_impl)), name_ident.span());

    let wrapped_class = quote! {
        //generated
        type Wrapped = #wrapper_type<#struct_name>;
        static mut #class_static: Option<*mut puredata_sys::_class> = None;
    };

    trampolines.push(quote! {
        //new trampoline
        pub unsafe extern "C" fn #new_method () -> *mut ::std::os::raw::c_void {
            Wrapped::new(#class_static.expect("class not initialized"))
        }

        //free trampoline
        pub unsafe extern "C" fn #free_method (x: *mut Wrapped) {
            let x = &mut *x;
            x.free();
        }
    });

    let mut register_methods = Vec::new();
    register_methods.push(quote! {
        #class_inst.add_method(Method::Bang(hellodsp_tilde_bang_trampoline));
    });

    register_methods.push(quote! {
        let name = CString::new("blah").unwrap();
        #class_inst.add_method(Method::SelF1(name, hellodsp_tilde_float_trampoline, 1));
    });

    let class_new_method = match etype {
        ExternalType::Signal => add_dsp(
            &mut trampolines,
            &new_method,
            &free_method,
            &flat_name,
            name_ident.span(),
        ),
        ExternalType::Control => add_control(&new_method, &free_method),
    };

    trampolines.push(quote! {
        #[no_mangle]
        pub unsafe extern "C" fn #setup_method() {
            let name = CString::new(#class_name).unwrap();
            let mut #class_inst = #class_new_method

            #(#register_methods)*

            #class_static = Some(#class_inst.into());
        }
    });

    let expanded = quote! {
        #the_struct

        #(#impls)*

        #wrapped_class

        #(#trampolines)*

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
