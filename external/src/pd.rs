use std::ffi::CString;

pub fn sample_rate() -> pd_sys::t_float {
    unsafe { pd_sys::sys_getsr() }
}

pub fn post(s: CString) {
    unsafe {
        pd_sys::post(s.as_ptr());
    }
}
