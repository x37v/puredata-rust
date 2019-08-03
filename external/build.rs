// build.rs

use quote::quote;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
enum Arg {
    Float,
    Symbol,
}

impl Arg {
    pub fn to_string(&self) -> String {
        match self {
            Arg::Float => "F",
            Arg::Symbol => "S",
        }
        .to_string()
    }

    pub fn to_sig(&self) -> proc_macro2::TokenStream {
        match self {
            Arg::Float => quote! { puredata_sys::t_floatarg },
            Arg::Symbol => quote! { *mut puredata_sys::t_symbol },
        }
    }

    pub fn to_arg(&self) -> proc_macro2::TokenStream {
        match self {
            Arg::Float => quote! { puredata_sys::t_atomtype::A_FLOAT },
            Arg::Symbol => quote! { puredata_sys::t_atomtype::A_SYMBOL },
        }
    }

    pub fn to_classnewarg(&self) -> proc_macro2::TokenStream {
        match self {
            Arg::Float => quote! { puredata_sys::t_atomtype::A_DEFFLOAT },
            Arg::Symbol => quote! { puredata_sys::t_atomtype::A_DEFSYMBOL },
        }
    }
}

fn append_perms(types: &Vec<Arg>, perms: &mut Vec<Vec<Arg>>, recurse: usize) {
    if recurse == 0 {
        return;
    }
    let mut append = Vec::new();
    for t in types {
        for p in perms.iter() {
            let mut n = p.clone();
            n.push(*t);
            append.push(n);
        }
    }
    append_perms(types, &mut append, recurse - 1);
    for a in append.into_iter() {
        perms.push(a);
    }
}

fn type_alias_name(perm: &Vec<Arg>) -> String {
    perm.iter()
        .map(Arg::to_string)
        .collect::<Vec<String>>()
        .join("")
}

fn classnew_variant_name(type_alias: &String) -> syn::Ident {
    syn::Ident::new(type_alias, proc_macro2::Span::call_site())
}

fn sel_variant_name(type_alias: &String) -> syn::Ident {
    syn::Ident::new(
        &format!("Sel{}", type_alias),
        proc_macro2::Span::call_site(),
    )
}

fn gen_method(perms: &Vec<Vec<Arg>>) -> Result<(), Box<std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("method-gen.rs");
    let mut f = File::create(&dest_path)?;

    //build types
    let mut variants = Vec::new();
    for p in perms.iter() {
        //build type alias
        let alias = type_alias_name(&p);
        let t = syn::Ident::new(&alias, proc_macro2::Span::call_site());
        let v = sel_variant_name(&alias);

        //build method signature
        let args = p.iter().map(Arg::to_sig);
        f.write_all(
            quote! {
                pub type #t<T> = unsafe extern "C" fn(*mut T, #(#args),*);
            }
            .to_string()
            .as_bytes(),
        )?;
        f.write_all(b"\n")?;

        //build up variant
        //TODO when we allow pointers, don't provide defaults if a pointer is at the end?
        variants.push(quote! {
            #v(std::ffi::CString, #t<T>, usize)
        });
    }

    //build enumeration
    f.write_all(
        quote! {
        pub enum Method<T> {
            Bang(B<T>),
            Float(F<T>),
            Symbol(S<T>),
            List(SelList<T>),
            AnyThing(SelList<T>),
            Sel(CString, B<T>),
            #(#variants),*
        }
        }
        .to_string()
        .as_bytes(),
    )?;

    //class new enumeration
    let cvoidptr = quote! { *mut ::std::os::raw::c_void };
    let mut variants = vec![
        quote! { NoArgs(unsafe extern "C" fn() -> #cvoidptr) },
        quote! { VarArgs(unsafe extern "C" fn(*mut puredata_sys::t_symbol, std::os::raw::c_int, *const puredata_sys::t_atom) -> #cvoidptr) },
    ];
    //XXX TODO, filter out pointers when we get them
    for p in perms.iter() {
        //build type alias
        let alias = type_alias_name(&p);
        let v = classnew_variant_name(&alias);
        let args = p.iter().map(Arg::to_sig);
        variants.push(quote! { #v(unsafe extern "C" fn(#(#args),*) -> #cvoidptr) });
    }

    f.write_all(
        quote! {
            pub enum ClassNewMethod {
                #(#variants),*
            }
        }
        .to_string()
        .as_bytes(),
    )?;

    Ok(())
}

fn gen_class(perms: &Vec<Vec<Arg>>) -> Result<(), Box<std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("class-gen.rs");
    let mut f = File::create(&dest_path)?;

    //implementation that adds methods to class
    let mut matches = vec![
        quote! {
            Method::Bang(f) => {
                puredata_sys::class_addbang(
                    self.pd_class,
                    Some(std::mem::transmute::<method::B<T>, PdMethod>(f)),
                    );
            }
        },
        quote! {
            Method::Float(f) => {
                puredata_sys::class_doaddfloat(
                    self.pd_class,
                    Some(std::mem::transmute::<method::F<T>, PdMethod>(f)),
                    );
            }
        },
        quote! {
            Method::Symbol(f) => {
                puredata_sys::class_addsymbol(
                    self.pd_class,
                    Some(std::mem::transmute::<method::S<T>, PdMethod>(f)),
                    );
            }
        },
        quote! {
            Method::List(f) => {
                puredata_sys::class_addlist(
                    self.pd_class,
                    Some(std::mem::transmute::<method::SelList<T>, PdMethod>(f)),
                    );
            }
        },
        quote! {
            Method::AnyThing(f) => {
                puredata_sys::class_addanything(
                    self.pd_class,
                    Some(std::mem::transmute::<method::SelList<T>, PdMethod>(f)),
                    );
            }
        },
        quote! {
            Method::Sel(sel, f) => {
                self.add_sel_method(
                    sel,
                    Some(std::mem::transmute::<method::B<T>, PdMethod>(f)),
                    &mut [],
                    0,
                );
            }
        },
    ];
    for p in perms.iter() {
        let alias = type_alias_name(&p);
        let t = syn::Ident::new(&alias, proc_macro2::Span::call_site());
        let v = sel_variant_name(&alias);
        let args = p.iter().map(Arg::to_arg);
        matches.push(quote! {
            Method::#v(sel, f, defaults) => {
                self.add_sel_method(
                    sel,
                    Some(std::mem::transmute::<method::#t<T>, PdMethod>(f)),
                    &mut [#(#args),*],
                    defaults,
                );
            }
        });
    }

    f.write_all(
        quote! {
        impl<T> Class<T> {
            pub fn add_method(&mut self, m: Method<T>) {
                unsafe {
                    match m {
                        #(#matches)*
                    }
                }
            }
        }
        }
        .to_string()
        .as_bytes(),
    )?;
    Ok(())
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut perms = vec![vec![Arg::Float], vec![Arg::Symbol]];
    let types = vec![Arg::Float, Arg::Symbol];
    append_perms(&types, &mut perms, 5);

    gen_method(&perms)?;
    gen_class(&perms)?;

    Ok(())
}
