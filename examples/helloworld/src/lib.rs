use std::ffi::CString;

static mut HELLOWORLD_CLASS: Option<*mut puredata_sys::_class> = None;

#[repr(C)]
pub struct t_helloworld {
    x_obj: puredata_sys::t_object,
}

pub unsafe extern "C" fn helloworld_bang(_x: t_helloworld) {
    let m = CString::new("HELLO WORLD!!").expect("CString::new failed");
    puredata_sys::post(m.as_ptr());
}

pub unsafe extern "C" fn helloworld_new() -> *mut ::std::os::raw::c_void {
    let obj = std::mem::transmute::<*mut puredata_sys::t_pd, *mut t_helloworld>(
        puredata_sys::pd_new(HELLOWORLD_CLASS.unwrap()),
    );
    obj as *mut ::std::os::raw::c_void
}

#[no_mangle]
pub unsafe extern "C" fn helloworld_setup() {
    let name = CString::new("helloworld").expect("CString::new failed");
    let c = puredata_sys::class_new(
        puredata_sys::gensym(name.as_ptr()),
        Some(helloworld_new),
        None,
        std::mem::size_of::<t_helloworld>(),
        0,
        0,
    );
    HELLOWORLD_CLASS = Some(c);
    puredata_sys::class_addbang(
        c,
        Some(std::mem::transmute::<
            unsafe extern "C" fn(t_helloworld),
            unsafe extern "C" fn(),
        >(helloworld_bang)),
    );
}
