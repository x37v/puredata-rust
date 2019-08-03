use puredata_external::builder::ControlExternalBuilder;
use puredata_external::external::ControlExternal;
use puredata_external::outlet::{OutletSend, OutletType};
use puredata_external::pd;
use puredata_external_macros::external;
use std::ffi::CString;
use std::rc::Rc;

external! {
    pub struct HelloAll {
        outlet: Rc<dyn OutletSend>
    }

    impl ControlExternal for HelloAll {
        fn new(builder: &mut dyn ControlExternalBuilder<Self>) -> Self {
            let outlet = builder.new_message_outlet(OutletType::AnyThing);
            Self { outlet }
        }
    }

    impl HelloAll {
        #[bang] //indicates that a bang in Pd should call this
        pub fn bang(&mut self) {
            pd::post(CString::new("Hello world !!").unwrap());
        }

        #[list] //indicates that a list in Pd should call this
        pub fn list(&mut self, list: &[puredata_external::atom::Atom]) {
            let s = CString::new("toast").unwrap().into();
            self.outlet.send_anything(s, &list);
            self.outlet.send_symbol(s);
        }
    }
}
