use std::env;
use std::path::PathBuf;

fn main() {
    let vars = [
        "s_pointer",
        "s_float",
        "s_symbol",
        "s_bang",
        "s_list",
        "s_anything",
        "s_signal",
        "s__N",
        "s__X",
        "s_x",
        "s_y",
        "s_",
    ];
    let funcs = [
        "gensym",
        "atom_.*",
        "binbuf_.*",
        "clock_.*",
        "pd_.*",
        "gpointer_.*",
        ".*inlet.*",
        ".*outlet.*",
        "glob_.*",
        "canvas_.*",
        "sys_.*",
        "class_.*",
        "obj_.*",
        "plus_perform",
        "zero_perform",
        "copy_perform",
        "dsp_.*",
        "ilog2",
        "mayer_.*",
        "resample.*",
        "garray_.*",
        "value_.*",
        ".*bytes",
        "post",
        "startpost",
        "poststring",
        "postfloat",
        "postatom",
        "endpost",
        "error",
        "verbose",
        "bug",
        "pd_error",
        "logpost",
    ];
    let types = ["t_atomtype", "t_symbol"];

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .constified_enum_module("t_atomtype")
        .rustfmt_bindings(true);

    for i in vars.iter() {
        builder = builder.whitelist_var(i);
    }

    for i in funcs.iter() {
        builder = builder.whitelist_function(i);
    }

    for i in types.iter() {
        builder = builder.whitelist_type(i);
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
