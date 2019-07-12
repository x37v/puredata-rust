use puredata_external::builder::SignalProcessorExternalBuilder;
use puredata_external::class::{Class, SignalClassType};
use puredata_external::external::SignalProcessorExternal;
use puredata_external::method::Method;
use puredata_external::outlet::{OutletSend, OutletType};
use puredata_external::pd;
use puredata_external::wrapper::SignalProcessorExternalWrapper;

use puredata_external_macros::external_processor;

use std::ffi::CString;
use std::ops::Deref;

use std::rc::Rc;

external_processor! {
    pub struct HelloDSP {
        //THIS IS A TEST
    }

    impl HelloDSP {
        //#[bang_method]
        pub fn bang(&mut self) {
            //let m = CString::new(format!("hello {}", **self.inlet).to_string())
            let m = CString::new(format!("hello").to_string()).expect("CString::new failed");
            pd::post(m);
        }

        pub fn float(&mut self, arg: puredata_sys::t_float) {
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

pub unsafe extern "C" fn hellodsp_tilde_bang_trampoline(x: *mut Wrapped) {
    let x = &mut *x;
    x.wrapped().bang();
}

pub unsafe extern "C" fn hellodsp_tilde_float_trampoline(
    x: *mut Wrapped,
    arg: puredata_sys::t_float,
) {
    let x = &mut *x;
    x.wrapped().float(arg);
}
