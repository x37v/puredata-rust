use std::ffi::CString;

//an alias for the methods that pure data wants for registration with its API
pub type PdMethod = unsafe extern "C" fn();
pub type PdDspMethod<T> = unsafe extern "C" fn(*mut T, *mut *mut puredata_sys::t_signal);
pub type PdDspPerform = unsafe extern "C" fn(*mut puredata_sys::t_int) -> *mut puredata_sys::t_int;

pub type B<T> = unsafe extern "C" fn(*mut T);
pub type SelList<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    std::os::raw::c_int,
    *const puredata_sys::t_atom,
);

include!(concat!(env!("OUT_DIR"), "/method-gen.rs"));
