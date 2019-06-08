use crate::method;
use crate::method::{Method, PdMethod};
use std::ffi::CString;
use std::marker::PhantomData;

pub struct Class<T> {
    pd_class: *mut puredata_sys::_class,
    phantom: PhantomData<T>,
}

impl<T> Class<T> {
    pub fn register_new(
        name: CString,
        creator: unsafe extern "C" fn() -> *mut ::std::os::raw::c_void,
        destroyer: Option<unsafe extern "C" fn(&mut T)>,
    ) -> Self {
        unsafe {
            let destroyer = match destroyer {
                None => None,
                Some(d) => Some(std::mem::transmute::<
                    unsafe extern "C" fn(&mut T),
                    unsafe extern "C" fn(),
                >(d)),
            };
            Self {
                pd_class: puredata_sys::class_new(
                    puredata_sys::gensym(name.as_ptr()),
                    Some(creator),
                    destroyer,
                    std::mem::size_of::<T>(),
                    0,
                    0,
                ),
                phantom: PhantomData,
            }
        }
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
                        Some(std::mem::transmute::<method::F1<T>, PdMethod>(f)),
                    );
                }
                Method::SelF1(sel, f, defaults) => {
                    self.add_sel_method(
                        sel,
                        Some(std::mem::transmute::<method::F1<T>, PdMethod>(f)),
                        &mut [puredata_sys::t_atomtype::A_FLOAT],
                        defaults,
                    );
                }
                _ => unimplemented!(), //XXX TODO
            }
        }
    }
}

impl<T> Into<*mut puredata_sys::_class> for Class<T> {
    fn into(self) -> *mut puredata_sys::_class {
        self.pd_class
    }
}
