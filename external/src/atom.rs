#[repr(transparent)]
pub struct Atom {
    pub inner: *const puredata_sys::t_atom,
}

impl Atom {
    pub fn get_symbol(&self) -> Option<&mut puredata_sys::t_symbol> {
        assert!(!self.inner.is_null());
        unsafe {
            match (*self.inner).a_type {
                puredata_sys::t_atomtype::A_DEFSYM | puredata_sys::t_atomtype::A_SYMBOL => {
                    let s = puredata_sys::atom_getsymbol(self.inner);
                    if s.is_null() {
                        None
                    } else {
                        Some(&mut *s)
                    }
                }
                _ => None,
            }
        }
    }
    pub fn get_float(&self) -> Option<puredata_sys::t_float> {
        assert!(!self.inner.is_null());
        unsafe {
            match (*self.inner).a_type {
                puredata_sys::t_atomtype::A_DEFFLOAT | puredata_sys::t_atomtype::A_FLOAT => {
                    Some(puredata_sys::atom_getfloat(self.inner))
                }
                _ => None,
            }
        }
    }
    pub fn get_int(&self) -> Option<puredata_sys::t_int> {
        assert!(!self.inner.is_null());
        unsafe {
            match (*self.inner).a_type {
                //pd does the cast
                puredata_sys::t_atomtype::A_DEFFLOAT | puredata_sys::t_atomtype::A_FLOAT => {
                    Some(puredata_sys::atom_getint(self.inner))
                }
                _ => None,
            }
        }
    }
}
