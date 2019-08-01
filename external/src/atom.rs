#[repr(transparent)]
pub struct Atom(pub puredata_sys::t_atom);

impl Atom {
    pub fn slice_from_raw_parts(
        argv: *const puredata_sys::t_atom,
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

    pub fn set_semi(&mut self) {
        self.0.a_type = puredata_sys::t_atomtype::A_SEMI;
        self.0.a_w.w_index = 0;
    }

    pub fn set_comma(&mut self) {
        self.0.a_type = puredata_sys::t_atomtype::A_COMMA;
        self.0.a_w.w_index = 0;
    }

    pub fn set_pointer(&mut self, v: &mut puredata_sys::t_gpointer) {
        self.0.a_type = puredata_sys::t_atomtype::A_POINTER;
        self.0.a_w.w_gpointer = v as *mut puredata_sys::t_gpointer;
    }

    pub fn set_float(&mut self, v: puredata_sys::t_float) {
        self.0.a_type = puredata_sys::t_atomtype::A_FLOAT;
        self.0.a_w.w_float = v;
    }

    pub fn set_symbol(&mut self, v: &mut puredata_sys::t_symbol) {
        self.0.a_type = puredata_sys::t_atomtype::A_SYMBOL;
        self.0.a_w.w_symbol = v as *mut puredata_sys::t_symbol;
    }

    pub fn set_dollar(&mut self, v: std::os::raw::c_int) {
        self.0.a_type = puredata_sys::t_atomtype::A_DOLLAR;
        self.0.a_w.w_index = v;
    }

    pub fn set_dollarsym(&mut self, v: &mut puredata_sys::t_symbol) {
        self.0.a_type = puredata_sys::t_atomtype::A_DOLLSYM;
        self.0.a_w.w_symbol = v as *mut puredata_sys::t_symbol;
    }
}

impl Copy for Atom {}
impl Clone for Atom {
    fn clone(&self) -> Self {
        let a = puredata_sys::_atom {
            a_type: self.0.a_type,
            a_w: self.0.a_w,
        };
        Self(a)
    }
}

impl Default for Atom {
    fn default() -> Self {
        let a = puredata_sys::_atom {
            a_type: puredata_sys::t_atomtype::A_FLOAT,
            a_w: {
                puredata_sys::word {
                    w_float: 0f32.into(),
                }
            },
        };
        Self(a)
    }
}
