use puredata_external::builder::ControlExternalBuilder;
use puredata_external::external::ControlExternal;
use puredata_external::outlet::{OutletSend, OutletType};
use puredata_external::pd;
use puredata_external_macros::external;
use std::ffi::CString;
use std::ops::Deref;
use std::rc::Rc;

external! {
    pub struct HelloWorld {
        inlet: Rc<dyn Deref<Target = puredata_sys::t_float>>,
        outlet: Rc<dyn OutletSend>,
    }

    impl ControlExternal for HelloWorld {
        fn new(builder: &mut dyn ControlExternalBuilder<Self>) -> Self {
            Self {
                inlet: builder.new_passive_float_inlet(4f32),
                outlet: builder.new_message_outlet(OutletType::Float),
            }
        }
    }

    impl HelloWorld {
        #[bang]
        pub fn bang(&mut self) {
            self.outlet.send_float(**self.inlet);
        }

        #[sel(defaults=1)]
        pub fn blah(&mut self, arg: puredata_sys::t_float) {
            self.outlet.send_float(arg);
        }

        #[sel(name = "soda", defaults=1)]
        pub fn sel2(&mut self, arg: &mut puredata_sys::t_symbol) {
            let m =
                CString::new(format!("got soda {}", arg).to_string()).expect("CString::new failed");
            pd::post(m);
        }
    }
}
