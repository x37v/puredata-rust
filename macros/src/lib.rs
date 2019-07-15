extern crate proc_macro;

use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    parenthesized, parse_macro_input, Attribute, Ident, ImplItem, ImplItemMethod, Item, ItemImpl,
    ItemStruct, Lit, LitInt, LitStr, Token, Type,
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
fn add_control(new_method: &Ident, free_method: &Ident) -> proc_macro2::TokenStream {
    quote! {
        puredata_external::class::Class::<Wrapped>::register_new(name, #new_method, Some(#free_method));
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
        puredata_external::class::Class::<Wrapped>::register_dsp_new(
            name,
            #new_method,
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
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream);

fn add_bang(
    trampoline_name: &Ident,
    method_name: &Ident,
    _method: &ImplItemMethod,
    _attr: &Attribute,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    (
        quote! { puredata_external::method::Method::Bang(#trampoline_name) },
        quote! {
            pub unsafe extern "C" fn #trampoline_name(x: *mut Wrapped) {
                let x = &mut *x;
                x.wrapped().#method_name();
            }
        },
    )
}

fn add_sel(
    trampoline_name: &Ident,
    method_name: &Ident,
    _method: &ImplItemMethod,
    attr: &Attribute,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
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
        Err(e) => panic!(e),
    }

    let sel_name = quote! { std::ffi::CString::new(#sel_name).unwrap() };

    //TODO actually allow for more than just SelF1
    (
        quote! { puredata_external::method::Method::SelF1(#sel_name, #trampoline_name, #defaults)},
        quote! {
            pub unsafe extern "C" fn #trampoline_name(x: *mut Wrapped, a0: puredata_sys::t_float) {
                let x = &mut *x;
                x.wrapped().#method_name(a0);
            }
        },
    )
}

static METHOD_ATTRS: &'static [(&'static str, MethodRegisterFn)] =
    &[(&"bang", add_bang), (&"sel", add_sel)];

//extract annotated methods and build trampolines
fn update_method_trampolines(
    trampolines: &mut Vec<proc_macro2::TokenStream>,
    register_methods: &mut Vec<proc_macro2::TokenStream>,
    item: &ItemImpl,
    class_inst: &Ident,
    flat_name: &String,
) -> proc_macro2::TokenStream {
    let mut item = item.clone();
    item.items = item
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

                        //XXX tmp, only supports bang right now
                        let method_name = m.sig.ident.clone();
                        let trampoline_name = Ident::new(
                            &(format!("{:}_{:}_trampoline", flat_name, method_name)),
                            m.sig.ident.span(),
                        );
                        let (pd_method, trampoline) =
                            add_method(&trampoline_name, &method_name, &m, &a);
                        trampolines.push(trampoline);
                        register_methods.push(quote! {
                            #class_inst.add_method(#pd_method);
                        });
                    }
                }
                ImplItem::Method(m)
            }
            _ => i.clone(),
        })
        .collect();
    quote! {
        #item
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
        type Wrapped = puredata_external::wrapper::#wrapper_type<#struct_name>;
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

    let mut register_methods: Vec<proc_macro2::TokenStream> = Vec::new();

    let impls: Vec<proc_macro2::TokenStream> = impls
        .into_iter()
        .map(|i| {
            match i.self_ty.as_ref() {
                Type::Path(tp) => {
                    //matches struct
                    if tp.path.is_ident(the_struct.ident.clone()) {
                        return update_method_trampolines(
                            &mut trampolines,
                            &mut register_methods,
                            i,
                            &class_inst,
                            &flat_name,
                        );
                    }
                }
                _ => (),
            };
            quote! { #i }
        })
        .collect();

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
            let name = std::ffi::CString::new(#class_name).unwrap();
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
