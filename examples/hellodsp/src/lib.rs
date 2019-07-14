use puredata_external::builder::SignalProcessorExternalBuilder;
use puredata_external::external::SignalProcessorExternal;
use puredata_external::pd;

use puredata_external_macros::external;

use std::ffi::CString;

external! {
    pub struct HelloDSP;

    impl HelloDSP {
        #[bang]
        pub fn bang(&mut self) {
            let m = CString::new(format!("hello").to_string()).expect("CString::new failed");
            pd::post(m);
        }

        #[sel(defaults=1)]
        pub fn foo(&mut self, arg: puredata_sys::t_float) {
            let m =
                CString::new(format!("got float {}", arg).to_string()).expect("CString::new failed");
            pd::post(m);
        }
    }

    impl SignalProcessorExternal for HelloDSP {
        fn new(builder: &mut dyn SignalProcessorExternalBuilder<Self>) -> Self {
            builder.new_signal_outlet();
            builder.new_signal_outlet();
            builder.new_signal_outlet();
            builder.new_signal_inlet();
            builder.new_signal_inlet();
            Self {
                //inlet: builder.new_passive_float_inlet(4f32),
            }
        }
        fn process(
            &mut self,
            _frames: usize,
            inputs: &[&mut [puredata_sys::t_float]],
            outputs: &mut [&mut [puredata_sys::t_float]],
            ) {
            for (output, input) in outputs.iter_mut().zip(inputs.iter()) {
                output.copy_from_slice(input);
            }
        }
    }


    impl Drop for HelloDSP {
        fn drop(&mut self) {
            //if you need to do something
        }
    }
}
