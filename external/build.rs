// build.rs

use quote::quote;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn append_perms(types: &Vec<&'static str>, perms: &mut Vec<Vec<&'static str>>, recurse: usize) {
    if recurse == 0 {
        return;
    }
    let mut append = Vec::new();
    for t in types {
        for p in perms.iter() {
            let mut n = p.clone();
            n.push(t);
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
    /*
    let types = vec![
        (&"F", quote! { puredata_sys::t_float }),
        (&"S", quote! {*mut puredata_sys::t_symbol}),
    ];
    */

    let mut perms = vec![vec!["F"], vec!["S"]];
    let types = vec!["F", "S"];
    append_perms(&types, &mut perms, 5);

    let mut variants = Vec::new();
    for p in perms.iter() {
        let s = p.join("");
        let t = syn::Ident::new(&s, proc_macro2::Span::call_site());
        f.write_all(
            quote! {
                pub type #t<T> = unsafe extern "C" fn(*mut T, puredata_sys::t_float);
            }
            .to_string()
            .as_bytes(),
        )?;
        f.write_all(b"\n")?;

        let v = syn::Ident::new(&format!("Sel{}", s), proc_macro2::Span::call_site());
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
