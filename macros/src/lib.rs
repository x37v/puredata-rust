#![recursion_limit = "128"]
extern crate proc_macro;

use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    parenthesized, parse_macro_input, Attribute, FnArg, Ident, ImplItem, ImplItemMethod, Item,
    ItemImpl, ItemStruct, Lit, LitInt, LitStr, Pat, Token, Type, TypePath,
};

#[derive(Copy, Clone, Debug)]
enum ExternalType {
    Control,
    Signal,
}

struct Parsed {
    items: Vec<Item>,
}

#[derive(Debug)]
struct SelArgs {
    pub name: Option<Lit>,
    pub defaults: Option<Lit>,
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

impl Parse for SelArgs {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        if input.is_empty() {
            return Ok(Self {
                name: None,
                defaults: None,
            });
        }

        let content;
        parenthesized!(content in input);

        let mut name = None;
        let mut defaults = None;
        while !content.is_empty() {
            let key: Ident = content.parse()?;
            let _: Token![=] = content.parse()?;
            let value: Lit = content.parse()?;
            match key.to_string().as_ref() {
                "name" => name = Some(value),
                "defaults" => defaults = Some(value),
                v => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!("unknown sel attribute key '{}'", v),
                    ));
                }
            }
            let _: syn::Result<Token![,]> = content.parse();
        }
        Ok(Self { name, defaults })
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
        match i.self_ty.as_ref() {
            Type::Path(tp) => {
                //see if the type matches the struct we care about
                if tp.path.is_ident(the_struct.ident.clone()) {
                    //does this impl implement a trait?
                    if let Some((_, p, _)) = &i.trait_ {
                        //is it a trait we care about?
                        if let Some(type_trait) = type_traits
                            .iter()
                            .find(|x| p.segments.last().unwrap().value().ident == x.1)
                        {
                            return Ok((type_trait.0, type_trait.1));
                        }
                    }
                }
            }
            _ => (),
        }
    }
    Err("Couldn't find External Implementation")
}

//return the class initialization item
fn add_control(new_method_name: &Ident, free_method: &Ident) -> proc_macro2::TokenStream {
    quote! {
        puredata_external::class::Class::<Wrapped>::register_new(name, puredata_external::method::ClassNewMethod::VarArgs(#new_method_name), Some(#free_method));
    }
}

//return the class initialization item
fn add_dsp(
    trampolines: &mut Vec<proc_macro2::TokenStream>,
    new_method_name: &Ident,
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
        puredata_external::class::Class::<Wrapped>::register_dsp_new(
            name,
            puredata_external::method::ClassNewMethod::VarArgs(#new_method_name),
            puredata_external::class::SignalClassType::WithInput( #dsp_method, Wrapped::float_convert_field_offset(),),
            Some(#free_method),);
    }
}

//returns the method type token stream (for registering), and the trampoline implementation
type MethodRegisterFn = fn(
    trampoline_name: &Ident,
    method_name: &Ident,
    _method: &ImplItemMethod,
    attr: &Attribute,
) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)>;

fn add_bang(
    trampoline_name: &Ident,
    method_name: &Ident,
    _method: &ImplItemMethod,
    _attr: &Attribute,
) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    Ok((
        quote! { puredata_external::method::Method::Bang(#trampoline_name) },
        quote! {
            pub unsafe extern "C" fn #trampoline_name(x: *mut Wrapped) {
                let x = &mut *x;
                x.wrapped().#method_name();
            }
        },
    ))
}

fn add_list(
    trampoline_name: &Ident,
    method_name: &Ident,
    _method: &ImplItemMethod,
    _attr: &Attribute,
) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    //TODO validate arguments
    Ok((
        quote! { puredata_external::method::Method::List(#trampoline_name) },
        quote! {
            pub unsafe extern "C" fn #trampoline_name(x: *mut Wrapped, _sel: /*ignored, always &s_list*/ *mut puredata_sys::t_symbol, argc: std::os::raw::c_int, argv: *const puredata_sys::t_atom) {
                let x = &mut *x;
                let args = puredata_external::atom::Atom::slice_from_raw_parts(argv, argc);
                x.wrapped().#method_name(args);
            }
        },
    ))
}

fn add_anything(
    trampoline_name: &Ident,
    method_name: &Ident,
    _method: &ImplItemMethod,
    _attr: &Attribute,
) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    //TODO validate arguments
    Ok((
        quote! { puredata_external::method::Method::AnyThing(#trampoline_name) },
        quote! {
            pub unsafe extern "C" fn #trampoline_name(x: *mut Wrapped, sel: *mut puredata_sys::t_symbol, argc: std::os::raw::c_int, argv: *const puredata_sys::t_atom) {
                let x = &mut *x;
                let args = puredata_external::atom::Atom::slice_from_raw_parts(argv, argc);
                x.wrapped().#method_name(puredata_external::symbol::Symbol::try_from(sel).unwrap(), args);
            }
        },
    ))
}

fn type_path_final_eq(p: &TypePath, ident: &str) -> bool {
    p.path.segments.last().unwrap().value().ident == ident
}

fn add_sel(
    trampoline_name: &Ident,
    method_name: &Ident,
    method: &ImplItemMethod,
    attr: &Attribute,
) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let mut sel_name = Lit::Str(LitStr::new(&method_name.to_string(), method_name.span()));
    let mut defaults = Lit::Int(LitInt::new(0, syn::IntSuffix::Usize, attr.span()));

    //extract name and defaults if they're given
    let args: syn::parse::Result<SelArgs> = syn::parse2(attr.tts.clone());
    match args {
        Ok(args) => {
            if let Some(d) = args.defaults {
                defaults = d.clone();
            }
            if let Some(n) = args.name {
                sel_name = n.clone();
            }
        }
        Err(e) => return Err(syn::Error::new(attr.span(), e)),
    }

    let sel_name = quote! { std::ffi::CString::new(#sel_name).unwrap() };

    //XXX assert that the first arg is SelfRef

    //extract the args
    //arg name, arg type, string rep for Sel method, tramp arg type if different (*mut -> &mut)
    let args: syn::Result<Vec<(&Ident, &Type, String, Option<proc_macro2::TokenStream>)>> = method
        .sig
        .decl
        .inputs
        .iter()
        .skip(1)
        .map(|a| {
            if let FnArg::Captured(a) = a {
                match (&a.pat, &a.ty) {
                    (Pat::Ident(i), Type::Path(p)) => {
                        if type_path_final_eq(&p, &"t_float") {
                            return Ok((&i.ident, &a.ty, "F".to_string(), None));
                        } else if type_path_final_eq(&p, &"Symbol") {
                            return Ok((
                                &i.ident,
                                &a.ty,
                                "S".to_string(),
                                Some(quote! { *mut puredata_sys::t_symbol }),
                            ));
                        }
                    }
                    _ => (),
                };
            }

            return Err(syn::Error::new(
                a.span(),
                format!("unsupported arg type {:?}", a),
            ));
        })
        .collect();
    let args = args?;

    //build variant
    let variant = format!(
        "Sel{}",
        args.iter()
            .map(|x| x.2.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
    let variant = Ident::new(&variant, method_name.span());

    let mut tramp_args: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut wrapped_params: Vec<Ident> = Vec::new();
    //*mut -> &mut
    let mut wrapped_refs = vec![quote! { let x = &mut *x; }];
    for a in args.iter() {
        let ident = a.0;
        let typ = a.1;
        if let Some(t) = &a.3 {
            //TODO allow more types other than symbol
            let call_type = quote! { puredata_external::symbol::Symbol };
            wrapped_refs.push(quote! { let #ident = #call_type::try_from(#ident).unwrap(); });
            tramp_args.push(quote! { #ident: #t });
        } else {
            tramp_args.push(quote! { #ident: #typ });
        }
        wrapped_params.push(a.0.clone());
    }

    let call = if args.len() == 0 {
        quote! { puredata_external::method::Method::#variant(#sel_name, #trampoline_name)}
    } else {
        quote! { puredata_external::method::Method::#variant(#sel_name, #trampoline_name, #defaults)}
    };

    Ok((
        call,
        quote! {
            pub unsafe extern "C" fn #trampoline_name(x: *mut Wrapped, #(#tramp_args),*) {
                #(#wrapped_refs)*;
                x.wrapped().#method_name(#(#wrapped_params),*);
            }
        },
    ))
}

static METHOD_ATTRS: &'static [(&'static str, MethodRegisterFn)] = &[
    (&"bang", add_bang),
    (&"sel", add_sel),
    (&"list", add_list),
    (&"anything", add_anything),
];

//extract annotated methods and build trampolines
fn update_method_trampolines(
    trampolines: &mut Vec<proc_macro2::TokenStream>,
    register_methods: &mut Vec<proc_macro2::TokenStream>,
    item: &ItemImpl,
    class_inst: &Ident,
    flat_name: &String,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut item = item.clone();
    let updated_items: syn::Result<Vec<ImplItem>> = item
        .items
        .iter()
        .map(|i| match &i {
            ImplItem::Method(m) => {
                let mut m = m.clone();
                for (n, add_method) in METHOD_ATTRS.iter() {
                    if let Some(pos) = m
                        .attrs
                        .iter()
                        .position(|a| a.path.segments.last().unwrap().value().ident == n)
                    {
                        let a = m.attrs.remove(pos);

                        let method_name = m.sig.ident.clone();
                        let trampoline_name = Ident::new(
                            &(format!("{:}_method_{:}_trampoline", flat_name, method_name)),
                            m.sig.ident.span(),
                        );
                        let (pd_method, trampoline) =
                            add_method(&trampoline_name, &method_name, &m, &a)?;
                        trampolines.push(trampoline);
                        register_methods.push(quote! {
                            #class_inst.add_method(#pd_method);
                        });
                    }
                }
                Ok(ImplItem::Method(m))
            }
            _ => Ok(i.clone()),
        })
        .collect();
    item.items = updated_items?;
    Ok(quote! {
        #item
    })
}

#[proc_macro]
pub fn external(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Parsed { items } = parse_macro_input!(input as Parsed);
    match parse_and_build(items) {
        Ok(ts) => ts,
        Err(err) => err.to_compile_error().into(),
    }
}

fn parse_and_build(items: Vec<Item>) -> syn::Result<proc_macro::TokenStream> {
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
    let new_method_name = Ident::new(&(format!("{:}_new", flat_name)), name_ident.span());
    let free_method = Ident::new(&(format!("{:}_free", flat_name)), name_ident.span());
    let setup_method = Ident::new(&(format!("{:}_setup", flat_name)), name_ident.span());
    let wrapper_type = Ident::new(&(format!("{:}Wrapper", extern_impl)), name_ident.span());

    let wrapped_class = quote! {
        //generated
        type Wrapped = puredata_external::wrapper::#wrapper_type<#struct_name>;
        static mut #class_static: Option<*mut puredata_sys::_class> = None;
    };

    //new trampoline
    trampolines.push(
        quote! {
            pub unsafe extern "C" fn #new_method_name (name: *mut puredata_sys::t_symbol, argc: std::os::raw::c_int, argv: *const puredata_sys::t_atom) -> *mut ::std::os::raw::c_void {
                let args = puredata_external::atom::Atom::slice_from_raw_parts(argv, argc);
                Wrapped::new(#class_static.expect("class not initialized"), &args, name)
            }
        });

    //free trampoline
    trampolines.push(quote! {
        pub unsafe extern "C" fn #free_method (x: *mut Wrapped) {
            let x = &mut *x;
            x.free();
        }
    });

    let mut register_methods: Vec<proc_macro2::TokenStream> = Vec::new();

    let impls: syn::Result<Vec<proc_macro2::TokenStream>> = impls
        .into_iter()
        .map(|i| {
            match i.self_ty.as_ref() {
                Type::Path(tp) => {
                    //matches struct
                    if tp.path.is_ident(the_struct.ident.clone()) {
                        return Ok(update_method_trampolines(
                            &mut trampolines,
                            &mut register_methods,
                            i,
                            &class_inst,
                            &flat_name,
                        )?);
                    }
                }
                _ => (),
            };
            Ok(quote! { #i })
        })
        .collect();
    let impls = impls?;

    let class_new_method = match etype {
        ExternalType::Signal => add_dsp(
            &mut trampolines,
            &new_method_name,
            &free_method,
            &flat_name,
            name_ident.span(),
        ),
        ExternalType::Control => add_control(&new_method_name, &free_method),
    };

    trampolines.push(quote! {
        #[no_mangle]
        pub unsafe extern "C" fn #setup_method() {
            let name = std::ffi::CString::new(#class_name).unwrap();
            let mut #class_inst = #class_new_method

            #(#register_methods)*

            #class_static = Some(#class_inst.into());
        }
    });

    //how do we not get dupes of TryFrom if it is already included
    let expanded = quote! {
        use std::convert::TryFrom;

        #the_struct

        #(#impls)*

        #wrapped_class

        #(#trampolines)*

        #(#remain)*
    };

    Ok(proc_macro::TokenStream::from(expanded))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
