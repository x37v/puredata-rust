use pd_ext::builder::ControlExternalBuilder;
use pd_ext::external::ControlExternal;
use pd_ext::outlet::{OutletSend, OutletType};
use pd_ext_macros::external;

//https://github.com/pure-data/externals-howto#a-complex-external-counter
external! {
    pub struct Counter {
        count: isize,
        step: isize,
        range: (isize, isize),
        count_outlet: Box<dyn OutletSend>,
        wrap_outlet: Box<dyn OutletSend>,
    }

    impl ControlExternal for Counter {
        fn new(builder: &mut dyn ControlExternalBuilder<Self>) -> Result<Self, String> {

            //parse the args
            let mut step = 1;
            let mut range = (0, 1);
            let mut args = builder.creation_args().iter();
            if let Some(atom) = args.next() {
                range.0 = atom.get_int().unwrap_or(0) as isize;
                if let Some(atom) = args.next() {
                    range.1 = atom.get_int().unwrap_or(1) as isize;
                    if let Some(atom) = args.next() {
                        step = atom.get_int().unwrap_or(1) as isize;
                    }
                }
            }

            //reorder if needed
            if range.0 > range.1 {
                range = (range.1, range.0);
            }

            let count_outlet = builder.new_message_outlet(OutletType::Float);
            let wrap_outlet = builder.new_message_outlet(OutletType::Bang);
            Ok(Self { count: 0, step, range, count_outlet, wrap_outlet })
        }
    }

    impl Counter {
        #[bang]
        pub fn bang(&mut self) {
            let f = self.count as pd_sys::t_float;
            self.count = self.count.wrapping_add(self.step);

            if self.range.0 != self.range.1 {
                //if we're stepping up
                if self.step > 0 {
                    //and count has exceeded the upper range
                    if  self.count > self.range.1 {
                        self.count = self.range.0; //set to lower
                        self.wrap_outlet.send_bang();
                    }
                } else {
                    if  self.count < self.range.0 {
                        self.count = self.range.1; //set to upper
                        self.wrap_outlet.send_bang();
                    }
                }
            }

            self.count_outlet.send_float(f);
        }

        #[sel(defaults=1)]
        pub fn set(&mut self, v: pd_sys::t_float) {
            self.count = v as isize;
        }

        #[sel]
        pub fn reset(&mut self) {
            self.count = self.range.0;
        }

        #[sel(defaults=2)]
        pub fn bound(&mut self, bottom: pd_sys::t_float, top: pd_sys::t_float) {
            self.range =
            if bottom > top {
                (top as isize, bottom as isize)
            } else {
                (bottom as isize, top as isize)
            };
        }
    }
}
