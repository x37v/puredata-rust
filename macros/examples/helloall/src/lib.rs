use pd_ext::builder::ControlExternalBuilder;
use pd_ext::external::ControlExternal;
use pd_ext::outlet::{OutletSend, OutletType};
use pd_ext::pd;
use pd_ext_macros::external;
use std::ffi::CString;

external! {
    #[name="hello/all"] //allows us to change the default name from "helloall"
    pub struct HelloAll {
        outlet: Box<dyn OutletSend>
    }

    impl ControlExternal for HelloAll {
        fn new(builder: &mut dyn ControlExternalBuilder<Self>) -> Result<Self, String> {
            let outlet = builder.new_message_outlet(OutletType::AnyThing);
            Ok(Self { outlet })
        }
    }

    impl HelloAll {
        #[bang] //indicates that a bang in Pd should call this
        pub fn bang(&mut self) {
            pd::post(CString::new("Hello world !!").unwrap());
        }

        #[list] //indicates that a list in Pd should call this
        pub fn list(&mut self, list: &[pd_ext::atom::Atom]) {
            let s = CString::new("toast").unwrap().into();
            self.outlet.send_anything(s, &list);
            self.outlet.send_symbol(s);
        }

        #[anything]
        pub fn foo(&mut self, sel: pd_ext::symbol::Symbol, list: &[pd_ext::atom::Atom]) {
            self.outlet.send_symbol(sel);
            self.outlet.send_list(&list);
        }

        #[sel]
        pub fn bar(&mut self, s: pd_ext::symbol::Symbol) {
            self.outlet.send_symbol(s);
        }
    }
}
