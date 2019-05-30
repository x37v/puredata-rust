use std::ffi::CString;

pub type B<T> = extern "C" fn(&mut T);
pub type F1<T> = extern "C" fn(&mut T, puredata_sys::t_float);
pub type F2<T> = extern "C" fn(&mut T, puredata_sys::t_float, puredata_sys::t_float);
pub type F3<T> =
    extern "C" fn(&mut T, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type F4<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type F5<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type F6<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);

pub type S1<T> = extern "C" fn(&mut T, puredata_sys::t_symbol);
pub type S2<T> = extern "C" fn(&mut T, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type S3<T> =
    extern "C" fn(&mut T, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type S4<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type S5<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type S6<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);

pub type FS<T> = extern "C" fn(&mut T, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SF<T> = extern "C" fn(&mut T, puredata_sys::t_symbol, puredata_sys::t_float);

pub type FSF<T> =
    extern "C" fn(&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFF<T> =
    extern "C" fn(&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type FSS<T> =
    extern "C" fn(&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFS<T> =
    extern "C" fn(&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);

pub type FSFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type SFFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type FSSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type SFSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);

pub type FSFFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFSFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSFSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFFSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSSSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type SFFFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type FSSFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type SFSFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type FSFSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type SFFSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type FSSSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type SFSSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);

pub type FSFFFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFFFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSFFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFSFFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSFSFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFSFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSSFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFSSFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSFFSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFFFSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSSFSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSFSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFSSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFFSSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSSSSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSSSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFFFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type SFFFFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type FSSFFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type SFSFFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type FSFSFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type SFFSFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type FSSSFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type SFSSFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type FSFFSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type SFFFSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type FSSFSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type SFSFSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type FSFSSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type SFFSSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type FSSSSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type SFSSSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);

pub type FSFFFFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFFFFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSFFFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFSFFFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSFSFFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFSFFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSSFFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFSSFFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSFFSFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFFSFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSFSFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFSFSFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSFSSFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFFSSFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSSSSFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type SFSSSFF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
);
pub type FSFFFSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFFFFSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSSFFSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSFFSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFSFSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFFSFSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSSSFSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSSFSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFFSSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFFFSSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSSFSSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSFSSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFSSSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFFSSSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSSSSSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type SFSSSSF<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
);
pub type FSFFFFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type SFFFFFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type FSSFFFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type SFSFFFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type FSFSFFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type SFFSFFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type FSSSFFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type SFSSFFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type FSFFSFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type SFFFSFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type FSSFSFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type SFSFSFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type FSFSSFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type SFFSSFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type FSSSSFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type SFSSSFS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
);
pub type FSFFFSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type SFFFFSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type FSSFFSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type SFSFFSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type FSFSFSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type SFFSFSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type FSSSFSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type SFSSFSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type FSFFSSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type SFFFSSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type FSSFSSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type SFSFSSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type FSFSSSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type SFFSSSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type FSSSSSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);
pub type SFSSSSS<T> = extern "C" fn(
    &mut T,
    puredata_sys::t_symbol,
    puredata_sys::t_float,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
    puredata_sys::t_symbol,
);

pub enum Method<T> {
    Bang(B<T>),
    Float(F1<T>),
    Symbol(S1<T>),
    Sel(CString, fn(&mut T)),

    //selector, func, number of defaults (from back, at most the arity of the func)
    SelF1(CString, F1<T>, usize),
    SelF2(CString, F2<T>, usize),
    SelF3(CString, F3<T>, usize),
    SelF4(CString, F4<T>, usize),
    SelF5(CString, F5<T>, usize),
    SelF6(CString, F6<T>, usize),

    //selector, func, number of defaults (from back, at most the arity of the func)
    SelS1(CString, S1<T>, usize),
    SelS2(CString, S2<T>, usize),
    SelS3(CString, S3<T>, usize),
    SelS4(CString, S4<T>, usize),
    SelS5(CString, S5<T>, usize),
    SelS6(CString, S6<T>, usize),
}
