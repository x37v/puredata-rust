use crate::obj::AsObject;

pub trait OutletSend {
    fn send_bang(&self);
    fn send_float(&self, f: puredata_sys::t_float);
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

    fn send_float(&self, f: f32) {
        unsafe {
            puredata_sys::outlet_float(self.ptr, f as puredata_sys::t_float);
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
        Self {
            ptr: puredata_sys::outlet_new(owner.as_obj(), &mut puredata_sys::s_signal),
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
        fn send_float(&self, _f: f32) {}
    }
}
