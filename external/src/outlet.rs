use crate::obj::AsObject;
use crate::symbol::Symbol;

pub trait OutletSend {
    fn send_bang(&self);
    fn send_float(&self, v: pd_sys::t_float);
    fn send_symbol(&self, v: Symbol);
    fn send_list(&self, v: &dyn std::ops::Deref<Target = [crate::atom::Atom]>);
    fn send_anything(&self, sel: Symbol, v: &dyn std::ops::Deref<Target = [crate::atom::Atom]>);
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
    AnyThing,
}

pub struct Outlet {
    //outlet_type: OutletType,
    ptr: *mut pd_sys::_outlet,
}

pub struct SignalOutlet {
    ptr: *mut pd_sys::_outlet,
}

impl Outlet {
    pub fn new(outlet_type: OutletType, owner: &mut dyn AsObject) -> Self {
        unsafe {
            let ptr = match outlet_type {
                OutletType::Bang => pd_sys::outlet_new(owner.as_obj(), &mut pd_sys::s_bang),
                OutletType::Float => pd_sys::outlet_new(owner.as_obj(), &mut pd_sys::s_float),
                OutletType::Symbol => pd_sys::outlet_new(owner.as_obj(), &mut pd_sys::s_symbol),
                OutletType::Pointer => pd_sys::outlet_new(owner.as_obj(), &mut pd_sys::s_pointer),
                OutletType::List => pd_sys::outlet_new(owner.as_obj(), &mut pd_sys::s_list),
                OutletType::AnyThing => pd_sys::outlet_new(
                    owner.as_obj(),
                    std::ptr::null_mut() as *mut pd_sys::t_symbol,
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
            pd_sys::outlet_bang(self.ptr);
        }
    }

    fn send_float(&self, v: pd_sys::t_float) {
        unsafe {
            pd_sys::outlet_float(self.ptr, v);
        }
    }

    fn send_symbol(&self, v: Symbol) {
        unsafe {
            pd_sys::outlet_symbol(self.ptr, v.inner());
        }
    }

    fn send_list(&self, v: &dyn std::ops::Deref<Target = [crate::atom::Atom]>) {
        unsafe {
            let argc = v.len() as std::os::raw::c_int;
            let argv = v.deref().as_ptr() as *const pd_sys::t_atom;
            //XXX pd doesn't indicate const or mut but shouldn't be modifying
            let argv = std::mem::transmute::<_, *mut pd_sys::t_atom>(argv);
            pd_sys::outlet_list(self.ptr, &mut pd_sys::s_list, argc, argv);
        }
    }

    fn send_anything(&self, sel: Symbol, v: &dyn std::ops::Deref<Target = [crate::atom::Atom]>) {
        unsafe {
            let argc = v.len() as std::os::raw::c_int;
            let argv = v.deref().as_ptr() as *const pd_sys::t_atom;
            //XXX pd doesn't indicate const or mut but shouldn't be modifying
            let argv = std::mem::transmute::<_, *mut pd_sys::t_atom>(argv);
            pd_sys::outlet_anything(self.ptr, sel.inner(), argc, argv);
        }
    }
}

impl Drop for Outlet {
    fn drop(&mut self) {
        unsafe {
            pd_sys::outlet_free(self.ptr);
        }
    }
}

impl SignalOutlet {
    pub fn new(owner: &mut dyn AsObject) -> Self {
        let obj = owner.as_obj();
        unsafe {
            Self {
                ptr: pd_sys::outlet_new(obj, &mut pd_sys::s_signal),
            }
        }
    }
}

impl OutletSignal for SignalOutlet {}

impl Drop for SignalOutlet {
    fn drop(&mut self) {
        unsafe {
            pd_sys::outlet_free(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    impl OutletSend for () {
        fn send_bang(&self) {}
        fn send_float(&self, _v: pd_sys::t_float) {}
        fn send_symbol(&self, _v: Symbol) {}
        fn send_list(&self, _v: &dyn std::ops::Deref<Target = [crate::atom::Atom]>) {}
        fn send_anything(
            &self,
            sel: Symbol,
            _v: &dyn std::ops::Deref<Target = [crate::atom::Atom]>,
        ) {
        }
    }
}
