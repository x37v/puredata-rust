use std::ffi::CString;

pub type F1<T> = fn (&mut T, puredata_sys::t_float);
pub type F2<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_float);
pub type F3<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type F4<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type F5<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type F6<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type S1<T> = fn (&mut T, puredata_sys::t_symbol);
pub type S2<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type S3<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type S4<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type S5<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type S6<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);

pub enum Method<T> {
    Bang(fn(&mut T)),
    Float(F1<T>),
    Sel(CString, fn(&mut T)),

    SelF(CString, F1<T>),
    SelDF(CString, F1<T>),

    SelFF(CString, F2<T>),
    SelFDF(CString, F2<T>),

    SelFFF(CString, F3<T>),
    SelFFDF(CString, F3<T>),
    SelFDFDF(CString, F3<T>),

    SelFFFF(CString, F4<T>),
    SelFFFDF(CString, F4<T>),
    SelFFDFDF(CString, F4<T>),
    SelFDFDFDF(CString, F4<T>),
    SelDFDFDFDF(CString, F4<T>),

    SelFFFFF(CString, F5<T>),
    SelFFFFDF(CString, F5<T>),
    SelFFFDFDF(CString, F5<T>),
    SelFFDFDFDF(CString, F5<T>),
    SelFDFDFDFDF(CString, F5<T>),
    SelDFDFDFDFDF(CString, F5<T>),

    SelFFFFFF(CString, F6<T>),
    SelFFFFFDF(CString, F6<T>),
    SelFFFFDFDF(CString, F6<T>),
    SelFFFDFDFDF(CString, F6<T>),
    SelFFDFDFDFDF(CString, F6<T>),
    SelFDFDFDFDFDF(CString, F6<T>),
    SelDFDFDFDFDFDF(CString, F6<T>),
}
