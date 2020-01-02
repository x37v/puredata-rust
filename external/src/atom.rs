use crate::symbol::Symbol;
use std::convert::TryInto;

#[repr(transparent)]
pub struct Atom(pub pd_sys::t_atom);

impl Atom {
    pub fn slice_from_raw_parts(
        argv: *const pd_sys::t_atom,
        argc: std::os::raw::c_int,
    ) -> &'static [Atom] {
        unsafe {
            let (argv, argc) = if argv.is_null() {
                (std::ptr::null(), 0)
            } else {
                (
                    std::mem::transmute::<_, *const crate::atom::Atom>(argv),
                    if argc < 0 as std::os::raw::c_int {
                        0usize
                    } else {
                        argc as usize
                    },
                )
            };
            std::slice::from_raw_parts(argv, argc)
        }
    }

    pub fn get_type(&self) -> pd_sys::t_atomtype::Type {
        self.0.a_type
    }

    pub fn is_type(&self, t: pd_sys::t_atomtype::Type) -> bool {
        self.get_type() == t
    }

    pub fn as_ptr(&self) -> *const pd_sys::t_atom {
        &self.0 as *const pd_sys::t_atom
    }
    pub fn get_symbol(&self) -> Option<Symbol> {
        assert!(!self.as_ptr().is_null());
        unsafe {
            match self.0.a_type {
                pd_sys::t_atomtype::A_DEFSYM | pd_sys::t_atomtype::A_SYMBOL => {
                    pd_sys::atom_getsymbol(self.as_ptr()).try_into().ok()
                }
                _ => None,
            }
        }
    }
    pub fn get_float(&self) -> Option<pd_sys::t_float> {
        assert!(!self.as_ptr().is_null());
        unsafe {
            match self.0.a_type {
                pd_sys::t_atomtype::A_DEFFLOAT | pd_sys::t_atomtype::A_FLOAT => {
                    Some(pd_sys::atom_getfloat(self.as_ptr()))
                }
                _ => None,
            }
        }
    }
    pub fn get_int(&self) -> Option<pd_sys::t_int> {
        assert!(!self.as_ptr().is_null());
        unsafe {
            match self.0.a_type {
                //pd does the cast
                pd_sys::t_atomtype::A_DEFFLOAT | pd_sys::t_atomtype::A_FLOAT => {
                    Some(pd_sys::atom_getint(self.as_ptr()))
                }
                _ => None,
            }
        }
    }

    pub fn set_semi(&mut self) {
        self.0.a_type = pd_sys::t_atomtype::A_SEMI;
        self.0.a_w.w_index = 0;
    }

    pub fn set_comma(&mut self) {
        self.0.a_type = pd_sys::t_atomtype::A_COMMA;
        self.0.a_w.w_index = 0;
    }

    pub fn set_pointer(&mut self, v: &mut pd_sys::t_gpointer) {
        self.0.a_type = pd_sys::t_atomtype::A_POINTER;
        self.0.a_w.w_gpointer = v as *mut pd_sys::t_gpointer;
    }

    pub fn set_float(&mut self, v: pd_sys::t_float) {
        self.0.a_type = pd_sys::t_atomtype::A_FLOAT;
        self.0.a_w.w_float = v;
    }

    pub fn set_symbol(&mut self, v: Symbol) {
        self.0.a_type = pd_sys::t_atomtype::A_SYMBOL;
        self.0.a_w.w_symbol = v.inner();
    }

    pub fn set_dollar(&mut self, v: std::os::raw::c_int) {
        self.0.a_type = pd_sys::t_atomtype::A_DOLLAR;
        self.0.a_w.w_index = v;
    }

    pub fn set_dollarsym(&mut self, v: Symbol) {
        self.0.a_type = pd_sys::t_atomtype::A_DOLLSYM;
        self.0.a_w.w_symbol = v.inner();
    }
}

impl Copy for Atom {}
impl Clone for Atom {
    fn clone(&self) -> Self {
        let a = pd_sys::_atom {
            a_type: self.0.a_type,
            a_w: self.0.a_w,
        };
        Self(a)
    }
}

impl std::convert::From<usize> for Atom {
    fn from(v: usize) -> Self {
        let mut s = Self::default();
        s.set_float(v as pd_sys::t_float);
        s
    }
}

impl std::convert::From<f64> for Atom {
    fn from(v: f64) -> Self {
        let mut s = Self::default();
        s.set_float(v as pd_sys::t_float);
        s
    }
}

impl std::convert::From<f32> for Atom {
    fn from(v: f32) -> Self {
        let mut s = Self::default();
        s.set_float(v as pd_sys::t_float);
        s
    }
}

impl std::convert::TryInto<String> for Atom {
    type Error = String;
    fn try_into(self) -> Result<String, Self::Error> {
        if let Some(s) = self.get_symbol() {
            Ok(s.into())
        } else if let Some(f) = self.get_float() {
            Ok(f.to_string())
        } else {
            Err(format!(
                "don't know how to convert {} to string",
                self.0.a_type
            ))
        }
    }
}

impl Default for Atom {
    fn default() -> Self {
        let a = pd_sys::_atom {
            a_type: pd_sys::t_atomtype::A_FLOAT,
            a_w: {
                pd_sys::word {
                    w_float: 0f32.into(),
                }
            },
        };
        Self(a)
    }
}
