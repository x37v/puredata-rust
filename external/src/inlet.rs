use crate::obj::AsObject;

pub trait InletSignal {}

pub struct SignalInlet {
    ptr: *mut pd_sys::_inlet,
}

pub mod passive {
    use crate::obj::AsObject;
    use std::boxed::Box;
    use std::ops::Deref;

    pub struct FloatInlet {
        value: Box<pd_sys::t_float>,
        inlet: *mut pd_sys::_inlet,
    }

    impl FloatInlet {
        pub fn new(owner: &mut dyn AsObject, initial_value: pd_sys::t_float) -> Self {
            let value = Box::new(initial_value);
            unsafe {
                let value = Box::into_raw(value);
                let inlet = pd_sys::floatinlet_new(owner.as_obj(), value);
                let value = Box::from_raw(value);
                Self { inlet, value }
            }
        }
    }

    impl Deref for FloatInlet {
        type Target = pd_sys::t_float;

        fn deref(&self) -> &pd_sys::t_float {
            self.value.deref()
        }
    }

    impl Drop for FloatInlet {
        fn drop(&mut self) {
            unsafe {
                pd_sys::inlet_free(self.inlet);
            }
        }
    }
}

impl SignalInlet {
    pub fn new(owner: &mut dyn AsObject) -> Self {
        unsafe {
            let obj = owner.as_obj();
            Self {
                ptr: pd_sys::inlet_new(
                    obj,
                    &mut (*obj).te_g.g_pd,
                    &mut pd_sys::s_signal,
                    &mut pd_sys::s_signal,
                ),
            }
        }
    }
}

impl Drop for SignalInlet {
    fn drop(&mut self) {
        unsafe {
            pd_sys::inlet_free(self.ptr);
        }
    }
}

impl InletSignal for SignalInlet {}
