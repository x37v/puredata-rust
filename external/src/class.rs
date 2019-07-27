use crate::method;
use crate::method::{Method, PdDspMethod, PdMethod};
use puredata_sys::{t_atom, t_floatarg, t_symbol};
use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::c_int;

pub struct Class<T> {
    pd_class: *mut puredata_sys::_class,
    phantom: PhantomData<T>,
}

pub enum SignalClassType<T> {
    NoInput(PdDspMethod<T>),
    WithInput(PdDspMethod<T>, usize), //method, byte offset of translation float field in struct T
}

pub type CVoidPtr = *mut ::std::os::raw::c_void;
pub type SymPtr = *mut t_symbol;

pub enum ClassNewMethod {
    NoArgs(unsafe extern "C" fn() -> CVoidPtr),
    VarArgs(unsafe extern "C" fn(SymPtr, c_int, *const t_atom) -> CVoidPtr),
    F1(unsafe extern "C" fn(t_floatarg) -> CVoidPtr),
    F2(unsafe extern "C" fn(t_floatarg, t_floatarg) -> CVoidPtr),
    F3(unsafe extern "C" fn(t_floatarg, t_floatarg, t_floatarg) -> CVoidPtr),
    F4(unsafe extern "C" fn(t_floatarg, t_floatarg, t_floatarg, t_floatarg) -> CVoidPtr),
    F5(
        unsafe extern "C" fn(
            t_floatarg,
            t_floatarg,
            t_floatarg,
            t_floatarg,
            t_floatarg,
        ) -> CVoidPtr,
    ),
    F6(
        unsafe extern "C" fn(
            t_floatarg,
            t_floatarg,
            t_floatarg,
            t_floatarg,
            t_floatarg,
            t_floatarg,
        ) -> CVoidPtr,
    ),
    S1(unsafe extern "C" fn(SymPtr) -> CVoidPtr),
    S2(unsafe extern "C" fn(SymPtr, SymPtr) -> CVoidPtr),
    S3(unsafe extern "C" fn(SymPtr, SymPtr, SymPtr) -> CVoidPtr),
    S4(unsafe extern "C" fn(SymPtr, SymPtr, SymPtr, SymPtr) -> CVoidPtr),
    S5(unsafe extern "C" fn(SymPtr, SymPtr, SymPtr, SymPtr, SymPtr) -> CVoidPtr),
    S6(unsafe extern "C" fn(SymPtr, SymPtr, SymPtr, SymPtr, SymPtr, SymPtr) -> CVoidPtr),
    //TODO all combinations of F and S up to 6 args total
}

impl<T> Class<T> {
    pub fn register_new(
        name: CString,
        creator: unsafe extern "C" fn() -> *mut ::std::os::raw::c_void,
        destroyer: Option<unsafe extern "C" fn(*mut T)>,
    ) -> Self {
        unsafe {
            let destroyer = match destroyer {
                None => None,
                Some(d) => Some(std::mem::transmute::<
                    unsafe extern "C" fn(*mut T),
                    unsafe extern "C" fn(),
                >(d)),
            };
            let flags = puredata_sys::CLASS_DEFAULT;
            let args = [0; 6];
            Self {
                pd_class: puredata_sys::class_new(
                    puredata_sys::gensym(name.as_ptr()),
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
            puredata_sys::class_addmethod(
                self.pd_class,
                Some(std::mem::transmute::<method::PdDspMethod<T>, PdMethod>(
                    dsp_method,
                )),
                puredata_sys::gensym(dsp.as_ptr()),
                puredata_sys::t_atomtype::A_CANT,
                0,
            );
        }
    }

    pub fn register_dsp_new(
        name: CString,
        creator: unsafe extern "C" fn() -> *mut ::std::os::raw::c_void,
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
                    puredata_sys::class_domainsignalin(s.pd_class, onset as c_int);
                }
            }
        }
        s
    }

    fn add_sel_method(
        &self,
        sel: CString,
        m: puredata_sys::t_method,
        types: &mut [puredata_sys::t_atomtype::Type],
        defaults: usize,
    ) {
        //fill in defaults
        let l = types.len();
        assert!(l >= defaults);
        for i in l - defaults..l {
            match types[i] {
                puredata_sys::t_atomtype::A_FLOAT | puredata_sys::t_atomtype::A_DEFFLOAT => {
                    types[i] = puredata_sys::t_atomtype::A_DEFFLOAT
                }
                puredata_sys::t_atomtype::A_SYMBOL | puredata_sys::t_atomtype::A_DEFSYM => {
                    types[i] = puredata_sys::t_atomtype::A_DEFSYM
                }
                _ => (), //panic?
            }
        }

        //register
        unsafe {
            match types.len() {
                1 => {
                    assert!(defaults <= 1);
                    puredata_sys::class_addmethod(
                        self.pd_class,
                        m,
                        puredata_sys::gensym(sel.as_ptr()),
                        types[0],
                        0,
                    );
                }
                2 => {
                    assert!(defaults <= 2);
                    puredata_sys::class_addmethod(
                        self.pd_class,
                        m,
                        puredata_sys::gensym(sel.as_ptr()),
                        types[0],
                        types[1],
                        0,
                    );
                }
                3 => {
                    assert!(defaults <= 3);
                    puredata_sys::class_addmethod(
                        self.pd_class,
                        m,
                        puredata_sys::gensym(sel.as_ptr()),
                        types[0],
                        types[1],
                        types[2],
                        0,
                    );
                }
                4 => {
                    assert!(defaults <= 4);
                    puredata_sys::class_addmethod(
                        self.pd_class,
                        m,
                        puredata_sys::gensym(sel.as_ptr()),
                        types[0],
                        types[1],
                        types[2],
                        types[3],
                        0,
                    );
                }
                5 => {
                    assert!(defaults <= 5);
                    puredata_sys::class_addmethod(
                        self.pd_class,
                        m,
                        puredata_sys::gensym(sel.as_ptr()),
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
                    puredata_sys::class_addmethod(
                        self.pd_class,
                        m,
                        puredata_sys::gensym(sel.as_ptr()),
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
            let obj = std::mem::transmute::<*mut puredata_sys::t_pd, *mut Self>(
                puredata_sys::pd_new(self.pd_class),
            );
            //XXX run init?
            obj as *mut ::std::os::raw::c_void
        }
    }
}

/*
impl<T> Class<T> {
    pub fn add_method(&mut self, m: Method<T>) {
        unsafe {
            match m {
                Method::Bang(f) => {
                    puredata_sys::class_addbang(
                        self.pd_class,
                        Some(std::mem::transmute::<method::B<T>, PdMethod>(f)),
                    );
                }
                Method::Float(f) => {
                    puredata_sys::class_doaddfloat(
                        self.pd_class,
                        Some(std::mem::transmute::<method::F<T>, PdMethod>(f)),
                    );
                }
                Method::SelF(sel, f, defaults) => {
                    self.add_sel_method(
                        sel,
                        Some(std::mem::transmute::<method::F<T>, PdMethod>(f)),
                        &mut [puredata_sys::t_atomtype::A_FLOAT],
                        defaults,
                    );
                }
                _ => unimplemented!(), //XXX TODO
            }
        }
    }
}
*/

include!(concat!(env!("OUT_DIR"), "/class-gen.rs"));

impl<T> Into<*mut puredata_sys::_class> for Class<T> {
    fn into(self) -> *mut puredata_sys::_class {
        self.pd_class
    }
}
