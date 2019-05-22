use std::ffi::CString;

pub fn post(s: CString) {
    unsafe {
        puredata_sys::post(s.as_ptr());
    }
}
