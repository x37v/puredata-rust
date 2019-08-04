use crate::method;
use crate::method::{ClassNewMethod, Method, PdDspMethod, PdMethod};
use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::c_int;

pub struct Class<T> {
    pd_class: *mut pd_sys::_class,
    phantom: PhantomData<T>,
}

pub enum SignalClassType<T> {
    NoInput(PdDspMethod<T>),
    WithInput(PdDspMethod<T>, usize), //method, byte offset of translation float field in struct T
}

impl<T> Class<T> {
    pub fn register_new(
        name: CString,
        creator: ClassNewMethod,
        destroyer: Option<unsafe extern "C" fn(*mut T)>,
    ) -> Self {
        unsafe {
            let mut args = [0; 6];
            let creator = std::mem::transmute::<
                _,
                unsafe extern "C" fn() -> *mut ::std::os::raw::c_void,
            >(match creator {
                ClassNewMethod::VarArgs(m) => {
                    args[0] = pd_sys::t_atomtype::A_GIMME;
                    m
                }
                _ => unimplemented!(),
            });
            let destroyer = match destroyer {
                None => None,
                Some(d) => Some(std::mem::transmute::<
                    unsafe extern "C" fn(*mut T),
                    unsafe extern "C" fn(),
                >(d)),
            };
            let flags = pd_sys::CLASS_DEFAULT;
            let name: &mut pd_sys::t_symbol = name.into();
            Self {
                pd_class: pd_sys::class_new(
                    name.as_ptr(),
                    Some(creator),
                    destroyer,
                    std::mem::size_of::<T>(),
                    flags as i32,
                    args[0],
                    args[1],
                    args[2],
                    args[3],
                    args[4],
                    args[5],
                    0,
                ),
                phantom: PhantomData,
            }
        }
    }

    fn register_dsp(&mut self, dsp_method: PdDspMethod<T>) {
        let dsp = CString::new("dsp").expect("failed to allocate 'dsp' cstring");
        unsafe {
            pd_sys::class_addmethod(
                self.pd_class,
                Some(std::mem::transmute::<method::PdDspMethod<T>, PdMethod>(
                    dsp_method,
                )),
                pd_sys::gensym(dsp.as_ptr()),
                pd_sys::t_atomtype::A_CANT,
                0,
            );
        }
    }

    pub fn register_dsp_new(
        name: CString,
        creator: ClassNewMethod,
        dsp_method: SignalClassType<T>,
        destroyer: Option<unsafe extern "C" fn(*mut T)>,
    ) -> Self {
        let mut s = Self::register_new(name, creator, destroyer);
        match dsp_method {
            SignalClassType::NoInput(m) => {
                s.register_dsp(m);
            }
            SignalClassType::WithInput(m, onset) => {
                s.register_dsp(m);
                unsafe {
                    pd_sys::class_domainsignalin(s.pd_class, onset as c_int);
                }
            }
        }
        s
    }

    fn add_sel_method(
        &self,
        sel: CString,
        m: pd_sys::t_method,
        types: &mut [pd_sys::t_atomtype::Type],
        defaults: usize,
    ) {
        //fill in defaults
        let l = types.len();
        assert!(l >= defaults);
        for i in l - defaults..l {
            match types[i] {
                pd_sys::t_atomtype::A_FLOAT | pd_sys::t_atomtype::A_DEFFLOAT => {
                    types[i] = pd_sys::t_atomtype::A_DEFFLOAT
                }
                pd_sys::t_atomtype::A_SYMBOL | pd_sys::t_atomtype::A_DEFSYM => {
                    types[i] = pd_sys::t_atomtype::A_DEFSYM
                }
                _ => panic!("type cannot be made default"),
            }
        }

        //register
        unsafe {
            let sym = pd_sys::gensym(sel.as_ptr());
            match types.len() {
                0 => {
                    pd_sys::class_addmethod(self.pd_class, m, sym, 0);
                }
                1 => {
                    assert!(defaults <= 1);
                    pd_sys::class_addmethod(self.pd_class, m, sym, types[0], 0);
                }
                2 => {
                    assert!(defaults <= 2);
                    pd_sys::class_addmethod(self.pd_class, m, sym, types[0], types[1], 0);
                }
                3 => {
                    assert!(defaults <= 3);
                    pd_sys::class_addmethod(self.pd_class, m, sym, types[0], types[1], types[2], 0);
                }
                4 => {
                    assert!(defaults <= 4);
                    pd_sys::class_addmethod(
                        self.pd_class,
                        m,
                        sym,
                        types[0],
                        types[1],
                        types[2],
                        types[3],
                        0,
                    );
                }
                5 => {
                    assert!(defaults <= 5);
                    pd_sys::class_addmethod(
                        self.pd_class,
                        m,
                        sym,
                        types[0],
                        types[1],
                        types[2],
                        types[3],
                        types[4],
                        0,
                    );
                }
                6 => {
                    assert!(defaults <= 6);
                    pd_sys::class_addmethod(
                        self.pd_class,
                        m,
                        sym,
                        types[0],
                        types[1],
                        types[2],
                        types[3],
                        types[4],
                        types[5],
                        0,
                    );
                }
                _ => unimplemented!(),
            }
        }
    }

    pub fn new_instance(&mut self) -> *mut ::std::os::raw::c_void {
        unsafe {
            let obj =
                std::mem::transmute::<*mut pd_sys::t_pd, *mut Self>(pd_sys::pd_new(self.pd_class));
            //XXX run init?
            obj as *mut ::std::os::raw::c_void
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/class-gen.rs"));

impl<T> Into<*mut pd_sys::_class> for Class<T> {
    fn into(self) -> *mut pd_sys::_class {
        self.pd_class
    }
}
