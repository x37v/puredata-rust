use puredata_external::builder::ControlExternalBuilder;
use puredata_external::external::ControlExternal;
use puredata_external::outlet::{OutletSend, OutletType};
use puredata_external_macros::external;
use std::rc::Rc;

//https://github.com/pure-data/externals-howto#a-complex-external-counter
external! {
    pub struct Counter {
        count: isize,
        step: isize,
        range: (isize, isize),
        count_outlet: Rc<dyn OutletSend>,
        wrap_outlet: Rc<dyn OutletSend>,
    }

    impl ControlExternal for Counter {
        fn new(builder: &mut dyn ControlExternalBuilder<Self>) -> Self {
            let mut args = builder.creation_args().iter();

            let mut step = 1;
            let mut range = (0, 1);
            if let Some(atom) = args.next() {
                range.0 = atom.get_int().unwrap_or(0) as isize;
            }

            if let Some(atom) = args.next() {
                range.1 = atom.get_int().unwrap_or(1) as isize;
            }

            if let Some(atom) = args.next() {
                step = atom.get_int().unwrap_or(1) as isize;
            }

            let count_outlet = builder.new_message_outlet(OutletType::Float);
            let wrap_outlet = builder.new_message_outlet(OutletType::Bang);
            Self { count: 0, step, range, count_outlet, wrap_outlet }
        }
    }

    impl Counter {
        #[bang]
        pub fn bang(&mut self) {
            let f = self.count as puredata_sys::t_float;
            self.count = self.count.wrapping_add(1);
            self.count_outlet.send_float(f);
        }

        #[sel(defaults=1)]
        pub fn set(&mut self, v: puredata_sys::t_float) {
            self.count = v as isize;
        }

        //XXX broken, passing `defaults
        #[sel]
        pub fn reset(&mut self) {
            self.count = self.range.0;
        }

        #[sel(defaults=2)]
        pub fn bound(&mut self, bottom: puredata_sys::t_float, top: puredata_sys::t_float) {
            self.range =
            if bottom > top {
                (top as isize, bottom as isize)
            } else {
                (bottom as isize, top as isize)
            };
        }
    }
}
