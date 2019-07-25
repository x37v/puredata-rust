use std::ffi::CString;

//an alias for the methods that pure data wants for registration with its API
pub type PdMethod = unsafe extern "C" fn();
pub type PdDspMethod<T> = unsafe extern "C" fn(*mut T, *mut *mut puredata_sys::t_signal);
pub type PdDspPerform = unsafe extern "C" fn(*mut puredata_sys::t_int) -> *mut puredata_sys::t_int;

pub type B<T> = unsafe extern "C" fn(*mut T);
pub type F1<T> = unsafe extern "C" fn(*mut T, puredata_sys::t_float);
pub type F2<T> = unsafe extern "C" fn(*mut T, puredata_sys::t_float, puredata_sys::t_float);
pub type F3<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type F4<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type F5<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type F6<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);

pub type S1<T> = unsafe extern "C" fn(*mut T, *mut puredata_sys::t_symbol);
pub type S2<T> = unsafe extern "C" fn(*mut T, *mut puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type S3<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type S4<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type S5<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type S6<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);

pub type FS<T> = unsafe extern "C" fn(*mut T, puredata_sys::t_float, *mut puredata_sys::t_symbol);
pub type SF<T> = unsafe extern "C" fn(*mut T, *mut puredata_sys::t_symbol, puredata_sys::t_float);

pub type FSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);

pub type FSFF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type SFFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type FSSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFSS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);

pub type FSFFF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSFF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFSFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSFSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFFSF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSSSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSSF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFFS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type SFFFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type FSSFS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type SFSFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type FSFSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFFSS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type FSSSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFSSS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);

pub type FSFFFF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFFFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSFFF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFSFFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSFSFF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFSFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSSFF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFSSFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSFFSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFFFSF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSSFSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSFSF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFSSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFFSSF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSSSSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSSSF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFFFS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type SFFFFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type FSSFFS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type SFSFFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type FSFSFS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type SFFSFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type FSSSFS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type SFSSFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type FSFFSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFFFSS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type FSSFSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFSFSS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type FSFSSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFFSSS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type FSSSSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFSSSS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);

pub type FSFFFFF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFFFFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSFFFF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFSFFFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSFSFFF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFSFFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSSFFF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFSSFFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSFFSFF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFFSFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSFSFF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFSFSFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSFSSFF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFSSFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSSSFF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFSSSFF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSFFFSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFFFFSF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSSFFSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSFFSF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFSFSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFFSFSF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSSSFSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSSFSF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFFSSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFFFSSF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSSFSSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSFSSF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFSSSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFFSSSF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSSSSSF<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSSSSF<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFFFFS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type SFFFFFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type FSSFFFS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type SFSFFFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type FSFSFFS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type SFFSFFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type FSSSFFS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type SFSSFFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type FSFFSFS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type SFFFSFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type FSSFSFS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type SFSFSFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type FSFSSFS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type SFFSSFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type FSSSSFS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type SFSSSFS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
);
pub type FSFFFSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFFFFSS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type FSSFFSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFSFFSS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type FSFSFSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFFSFSS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type FSSSFSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFSSFSS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type FSFFSSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFFFSSS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type FSSFSSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFSFSSS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type FSFSSSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFFSSSS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type FSSSSSS<T> = unsafe extern "C" fn(
    *mut T,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);
pub type SFSSSSS<T> = unsafe extern "C" fn(
    *mut T,
    *mut puredata_sys::t_symbol,
    puredata_sys::t_float,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
    *mut puredata_sys::t_symbol,
);

pub enum Method<T> {
    Bang(B<T>),
    Float(F1<T>),
    Symbol(S1<T>),
    Sel(CString, B<T>),

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

    SelSF(CString, SF<T>, usize),
    SelFS(CString, FS<T>, usize),
}
