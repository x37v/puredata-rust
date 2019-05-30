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
        creator: extern "C" fn() -> *mut ::std::os::raw::c_void,
        destroyer: Option<extern "C" fn(&mut T)>,
    ) -> Self {
        unsafe {
            let destroyer = match destroyer {
                None => None,
                Some(d) => Some(std::mem::transmute::<
                    extern "C" fn(&mut T),
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

    pub fn add_method(&mut self, m: Method<T>) {
        unsafe {
            match m {
                Method::Bang(f) => {
                    puredata_sys::class_addbang(
                        self.pd_class,
                        Some(std::mem::transmute::<method::B<T>, PdMethod>(f)),
                    );
                }
                _ => unimplemented!(), //XXX TODO
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
