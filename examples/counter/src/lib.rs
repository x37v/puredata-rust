use puredata_external::builder::ControlExternalBuilder;
use puredata_external::external::ControlExternal;
use puredata_external::outlet::{OutletSend, OutletType};
use puredata_external_macros::external;
use std::rc::Rc;

//based on https://github.com/pure-data/externals-howto#a-simple-external-counter
external! {
    pub struct Counter {
        count: isize,
        outlet: Rc<dyn OutletSend>
    }

    impl ControlExternal for Counter {
        fn new(builder: &mut dyn ControlExternalBuilder<Self>) -> Self {
            let count = if let Some(atom) = builder.creation_args().iter().next() {
                atom.get_int().unwrap_or(0) as isize
            } else {
                0isize
            };
            let outlet = builder.new_message_outlet(OutletType::Float);
            Self { count, outlet }
        }
    }

    impl Counter {
        #[bang]
        pub fn bang(&mut self) {
            let f = self.count as puredata_sys::t_float;
            self.count = self.count.wrapping_add(1);
            self.outlet.send_float(f);
        }
    }
}
