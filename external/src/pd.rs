use std::ffi::CString;

pub fn post(s: CString) {
    unsafe {
        pd_sys::post(s.as_ptr());
    }
}
