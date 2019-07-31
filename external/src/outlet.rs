use crate::obj::AsObject;

pub trait OutletSend {
    fn send_bang(&self);
    fn send_float(&self, f: puredata_sys::t_float);
    fn send_list(&self, l: &mut dyn std::ops::DerefMut<Target = [crate::atom::Atom]>);
}

//marker traits
pub trait OutletSignal {}

#[derive(Copy, Clone)]
pub enum OutletType {
    Bang,
    Float,
    Symbol,
    Pointer,
    List,
    Message,
}

pub struct Outlet {
    //outlet_type: OutletType,
    ptr: *mut puredata_sys::_outlet,
}

pub struct SignalOutlet {
    ptr: *mut puredata_sys::_outlet,
}

impl Outlet {
    pub fn new(outlet_type: OutletType, owner: &mut dyn AsObject) -> Self {
        unsafe {
            let ptr = match outlet_type {
                OutletType::Bang => {
                    puredata_sys::outlet_new(owner.as_obj(), &mut puredata_sys::s_bang)
                }
                OutletType::Float => {
                    puredata_sys::outlet_new(owner.as_obj(), &mut puredata_sys::s_float)
                }
                OutletType::Symbol => {
                    puredata_sys::outlet_new(owner.as_obj(), &mut puredata_sys::s_symbol)
                }
                OutletType::Pointer => {
                    puredata_sys::outlet_new(owner.as_obj(), &mut puredata_sys::s_pointer)
                }
                OutletType::List => {
                    puredata_sys::outlet_new(owner.as_obj(), &mut puredata_sys::s_list)
                }
                OutletType::Message => puredata_sys::outlet_new(
                    owner.as_obj(),
                    std::ptr::null_mut() as *mut puredata_sys::t_symbol,
                ),
            };
            Self {
                /*outlet_type,*/ ptr,
            }
        }
    }
}

impl OutletSend for Outlet {
    fn send_bang(&self) {
        unsafe {
            puredata_sys::outlet_bang(self.ptr);
        }
    }

    fn send_float(&self, f: puredata_sys::t_float) {
        unsafe {
            puredata_sys::outlet_float(self.ptr, f);
        }
    }

    fn send_list(&self, l: &mut dyn std::ops::DerefMut<Target = [crate::atom::Atom]>) {
        unsafe {
            let argc = l.len() as std::os::raw::c_int;
            let argv = l.deref_mut().as_mut_ptr() as *mut puredata_sys::t_atom;
            puredata_sys::outlet_list(self.ptr, &mut puredata_sys::s_list, argc, argv);
        }
    }
}

impl Drop for Outlet {
    fn drop(&mut self) {
        unsafe {
            puredata_sys::outlet_free(self.ptr);
        }
    }
}

impl SignalOutlet {
    pub fn new(owner: &mut dyn AsObject) -> Self {
        let obj = owner.as_obj();
        unsafe {
            Self {
                ptr: puredata_sys::outlet_new(obj, &mut puredata_sys::s_signal),
            }
        }
    }
}

impl OutletSignal for SignalOutlet {}

impl Drop for SignalOutlet {
    fn drop(&mut self) {
        unsafe {
            puredata_sys::outlet_free(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    impl OutletSend for () {
        fn send_bang(&self) {}
        fn send_float(&self, _f: puredata_sys::t_float) {}
        fn send_list(&self, l: &mut dyn std::ops::DerefMut<Target = [crate::atom::Atom]>) {}
    }
}
