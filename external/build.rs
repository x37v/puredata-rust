// build.rs

use quote::quote;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<std::error::Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join("method-gen.rs");
    let mut f = File::create(&dest_path)?;

    //build types
    let types = vec![
        ("F", quote! { puredata_sys::t_float }),
        ("S", quote! {*mut puredata_sys::t_symbol}),
    ];

    /*
    for nargs in 1...6 {
        for t in types {
        }
    }
    */
    f.write_all(
        quote! {
            pub type F1<T> = unsafe extern "C" fn(*mut T, puredata_sys::t_float);
        }
        .to_string()
        .as_bytes(),
    )?;

    f.write_all(
        b"
        pub fn message() -> &'static str {
            \"Hello, World!\"
        }
    ",
    )?;

    Ok(())
}
