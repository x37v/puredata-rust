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

pub type FS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type FSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);


pub enum Method<T> {
    Bang(fn(&mut T)),
    Float(F1<T>),
    Sel(CString, fn(&mut T)),

    //selector, func, number of defaults (from back, at most the arity of the func)
    SelF(CString, F1<T>, usize),
    SelFF(CString, F2<T>, usize),
    SelFFF(CString, F3<T>, usize),
    SelFFFF(CString, F4<T>, usize),
    SelFFFFF(CString, F5<T>, usize),
    SelFFFFFF(CString, F6<T>, usize),

    //selector, func, number of defaults (from back, at most the arity of the func)
    SelS(CString, S1<T>, usize),
    SelSS(CString, S2<T>, usize),
    SelSSS(CString, S3<T>, usize),
    SelSSSS(CString, S4<T>, usize),
    SelSSSSS(CString, S5<T>, usize),
    SelSSSSSS(CString, S6<T>, usize),
}
