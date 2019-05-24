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

pub type FSFF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type SFFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type FSSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFSF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSFS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SFFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol);
pub type FSSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFSS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);

pub type FSFFF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type SFFFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type FSSFF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type SFSFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type FSFSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFFSF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSSSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFSSF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSFFS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SFFFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol);
pub type FSSFS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SFSFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);
pub type FSFSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFFSS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type FSSSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFSSS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);

pub type FSFFFF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type SFFFFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type FSSFFF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type SFSFFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type FSFSFF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type SFFSFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type FSSSFF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type SFSSFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type FSFFSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFFFSF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSSFSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFSFSF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSFSSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFFSSF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSSSSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFSSSF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSFFFS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SFFFFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol);
pub type FSSFFS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SFSFFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol);
pub type FSFSFS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SFFSFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);
pub type FSSSFS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SFSSFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);
pub type FSFFSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFFFSS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type FSSFSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFSFSS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type FSFSSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFFSSS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type FSSSSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFSSSS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);

pub type FSFFFFF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type SFFFFFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type FSSFFFF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type SFSFFFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type FSFSFFF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type SFFSFFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type FSSSFFF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type SFSSFFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float);
pub type FSFFSFF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type SFFFSFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type FSSFSFF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type SFSFSFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type FSFSSFF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type SFFSSFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type FSSSSFF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type SFSSSFF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float);
pub type FSFFFSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFFFFSF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSSFFSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFSFFSF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSFSFSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFFSFSF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSSSFSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFSSFSF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSFFSSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFFFSSF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSSFSSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFSFSSF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSFSSSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFFSSSF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSSSSSF<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float);
pub type SFSSSSF<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float);
pub type FSFFFFS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SFFFFFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol);
pub type FSSFFFS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SFSFFFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol);
pub type FSFSFFS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SFFSFFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol);
pub type FSSSFFS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SFSSFFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol);
pub type FSFFSFS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SFFFSFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);
pub type FSSFSFS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SFSFSFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);
pub type FSFSSFS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SFFSSFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);
pub type FSSSSFS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);
pub type SFSSSFS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol);
pub type FSFFFSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFFFFSS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type FSSFFSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFSFFSS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type FSFSFSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFFSFSS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type FSSSFSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFSSFSS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type FSFFSSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFFFSSS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type FSSFSSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFSFSSS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type FSFSSSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFFSSSS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type FSSSSSS<T> = fn (&mut T, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);
pub type SFSSSSS<T> = fn (&mut T, puredata_sys::t_symbol, puredata_sys::t_float, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol, puredata_sys::t_symbol);


pub enum Method<T> {
    Bang(fn(&mut T)),
    Float(F1<T>),
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
