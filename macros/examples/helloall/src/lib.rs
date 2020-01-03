use pd_ext::builder::ControlExternalBuilder;
use pd_ext::external::ControlExternal;
use pd_ext::outlet::{OutletSend, OutletType};
use pd_ext::pd;
use pd_ext_macros::external;
use std::ffi::CString;

external! {
    pub struct HelloAll {
        outlet: Box<dyn OutletSend>
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
        pub fn list(&mut self, list: &[pd_ext::atom::Atom]) {
            if let Ok(s) = std::convert::TryFrom::try_from("list") {
                self.outlet.send_symbol(s);
                self.outlet.send_anything(s, &list);
            }
        }

        #[anything]
        pub fn foo(&mut self, sel: pd_ext::symbol::Symbol, list: &[pd_ext::atom::Atom]) {
            if let Ok(s) = std::convert::TryFrom::try_from("anything") {
                self.outlet.send_symbol(s);
            }
            self.outlet.send_symbol(sel);
            self.outlet.send_list(&list);
        }

        #[sel]
        pub fn bar(&mut self, s: pd_ext::symbol::Symbol) {
            if let Ok(s) = std::convert::TryFrom::try_from("bar") {
                self.outlet.send_symbol(s);
            }
            self.outlet.send_symbol(s);
        }

        #[sel]
        pub fn var(&mut self, list: &[pd_ext::atom::Atom]) {
            if let Ok(s) = std::convert::TryFrom::try_from("sel var") {
                self.outlet.send_symbol(s);
            }
            self.outlet.send_list(&list);
        }

        #[pointer]
        pub fn baz(&mut self, p: pd_ext::pointer::Pointer) {
            self.outlet.send_pointer(&p);
        }
    }
}
