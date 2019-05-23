#![deny(clippy::transmute_ptr_to_ptr)]
use std::ffi::CString;

static mut HELLOWORLD_CLASS: Option<*mut puredata_sys::_class> = None;

#[repr(C)]
pub struct HelloWorld {
    x_obj: puredata_sys::t_object,
}

impl HelloWorld {
    pub extern "C" fn got_float(&mut self, f: puredata_sys::t_float) {
        unsafe {
            let m =
                CString::new(format!("got float {}", f).to_string()).expect("CString::new failed");
            puredata_sys::post(m.as_ptr());
        }
    }
    
    fn bang(&mut self) {
        let m = CString::new("HELLO WORLD!!").expect("CString::new failed");
        unsafe {
            puredata_sys::post(m.as_ptr());
        }
    }

    pub unsafe extern "C" fn bang_trampoline(s: *mut Self) {
        let obj = &mut *(s as *mut Self);
        obj.bang();
    }

    pub extern "C" fn new_pd() -> *mut ::std::os::raw::c_void {
        unsafe {
            let obj = std::mem::transmute::<*mut puredata_sys::t_pd, *mut Self>(
                puredata_sys::pd_new(HELLOWORLD_CLASS.unwrap()),
            );
            obj as *mut ::std::os::raw::c_void
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn helloworld_setup() {
    let name = CString::new("helloworld").expect("CString::new failed");
    let c = puredata_sys::class_new(
        puredata_sys::gensym(name.as_ptr()),
        Some(std::mem::transmute::<
            extern "C" fn() -> *mut ::std::os::raw::c_void,
            unsafe extern "C" fn() -> *mut ::std::os::raw::c_void,
        >(HelloWorld::new_pd)),
        None,
        std::mem::size_of::<HelloWorld>(),
        0,
        0,
    );
    HELLOWORLD_CLASS = Some(c);
    puredata_sys::class_addbang(
        c,
        Some(std::mem::transmute::<
            unsafe extern "C" fn(*mut HelloWorld),
            unsafe extern "C" fn(),
        >(HelloWorld::bang_trampoline)),
    );
    puredata_sys::class_addmethod(
        c,
        Some(std::mem::transmute::<
            extern "C" fn(&mut HelloWorld, puredata_sys::t_float),
            unsafe extern "C" fn(),
        >(HelloWorld::got_float)),
        puredata_sys::gensym(name.as_ptr()),
        puredata_sys::t_atomtype::A_DEFFLOAT,
        0,
    );
}
