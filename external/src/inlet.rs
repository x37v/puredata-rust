pub mod passive {
    use crate::obj::AsObject;
    use std::boxed::Box;
    use std::ops::Deref;

    pub struct FloatInlet {
        value: Box<puredata_sys::t_float>,
        inlet: *mut puredata_sys::_inlet,
    }

    impl FloatInlet {
        pub fn new(owner: &mut dyn AsObject, initial_value: puredata_sys::t_float) -> Self {
            let value = Box::new(initial_value);
            unsafe {
                let value = Box::into_raw(value);
                let inlet = puredata_sys::floatinlet_new(owner.as_obj(), value);
                let value = Box::from_raw(value);
                Self { inlet, value }
            }
        }
    }

    impl Deref for FloatInlet {
        type Target = puredata_sys::t_float;

        fn deref(&self) -> &puredata_sys::t_float {
            self.value.deref()
        }
    }

    impl Drop for FloatInlet {
        fn drop(&mut self) {
            unsafe {
                puredata_sys::inlet_free(self.inlet);
            }
        }
    }
}
