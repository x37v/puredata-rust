#[repr(transparent)]
pub struct Atom(puredata_sys::t_atom);

impl Atom {
    pub fn as_ptr(&self) -> *const puredata_sys::t_atom {
        &self.0 as *const puredata_sys::t_atom
    }
    pub fn get_symbol(&self) -> Option<&mut puredata_sys::t_symbol> {
        assert!(!self.as_ptr().is_null());
        unsafe {
            match self.0.a_type {
                puredata_sys::t_atomtype::A_DEFSYM | puredata_sys::t_atomtype::A_SYMBOL => {
                    let s = puredata_sys::atom_getsymbol(self.as_ptr());
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
        assert!(!self.as_ptr().is_null());
        unsafe {
            match self.0.a_type {
                puredata_sys::t_atomtype::A_DEFFLOAT | puredata_sys::t_atomtype::A_FLOAT => {
                    Some(puredata_sys::atom_getfloat(self.as_ptr()))
                }
                _ => None,
            }
        }
    }
    pub fn get_int(&self) -> Option<puredata_sys::t_int> {
        assert!(!self.as_ptr().is_null());
        unsafe {
            match self.0.a_type {
                //pd does the cast
                puredata_sys::t_atomtype::A_DEFFLOAT | puredata_sys::t_atomtype::A_FLOAT => {
                    Some(puredata_sys::atom_getint(self.as_ptr()))
                }
                _ => None,
            }
        }
    }
}
