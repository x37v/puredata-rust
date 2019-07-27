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
            Arg::Float => quote! { puredata_sys::t_float },
            Arg::Symbol => quote! { *mut puredata_sys::t_symbol },
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

fn main() -> Result<(), Box<std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("method-gen.rs");
    let mut f = File::create(&dest_path)?;

    //build types

    let mut perms = vec![vec![Arg::Float], vec![Arg::Symbol]];
    let types = vec![Arg::Float, Arg::Symbol];
    append_perms(&types, &mut perms, 5);

    let mut variants = Vec::new();
    for p in perms.iter() {
        let s = p
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");
        //build type alias
        let t = syn::Ident::new(&s, proc_macro2::Span::call_site());
        //build method signature
        let args: Vec<proc_macro2::TokenStream> = p.iter().map(Arg::to_sig).collect();
        f.write_all(
            quote! {
                pub type #t<T> = unsafe extern "C" fn(*mut T, #(#args),*);
            }
            .to_string()
            .as_bytes(),
        )?;
        f.write_all(b"\n")?;

        //build enum variant name
        let v = syn::Ident::new(&format!("Sel{}", s), proc_macro2::Span::call_site());
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
            Sel(CString, B<T>),
            #(#variants),*
        }
        }
        .to_string()
        .as_bytes(),
    )?;

    Ok(())
}
