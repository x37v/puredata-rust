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
            /*
              for (o, i) in output.iter_mut().zip(input.iter()) {
            *o = *i;
            }
            */
        }
    }
}

impl HelloDSP {
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

impl Drop for HelloDSP {
    fn drop(&mut self) {
        //if you need to do something
    }
}

//pub type Wrapped = SignalProcessorExternalWrapper<HelloDSP>;

//static mut HELLODSP_CLASS: Option<*mut puredata_sys::_class> = None;

/*
pub unsafe extern "C" fn hellodsp_tilde_new() -> *mut ::std::os::raw::c_void {
    Wrapped::new(HELLODSP_CLASS.expect("hello dsp class not set"))
}

pub unsafe extern "C" fn hellodsp_tilde_free(x: *mut Wrapped) {
    let x = &mut *x;
    x.free();
}
*/

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

pub unsafe extern "C" fn hellodsp_tilde_dsp_trampoline(
    x: *mut Wrapped,
    sp: *mut *mut puredata_sys::t_signal,
) {
    let x = &mut *x;
    x.dsp(sp, hellodsp_tilde_perform_trampoline);
}

pub unsafe extern "C" fn hellodsp_tilde_perform_trampoline(
    w: *mut puredata_sys::t_int,
) -> *mut puredata_sys::t_int {
    //actually longer than 2 but .offset(1) didn't seem to work correctly
    //but slice does
    let x = std::slice::from_raw_parts(w, 2);
    let x = &mut *std::mem::transmute::<_, *mut Wrapped>(x[1]);
    x.perform(w)
}

#[no_mangle]
pub unsafe extern "C" fn hellodsp_tilde_setup() {
    let name = CString::new("hellodsp~").expect("CString::new failed");
    let mut c = Class::<Wrapped>::register_dsp_new(
        name,
        hellodsp_tilde_new,
        SignalClassType::WithInput(
            hellodsp_tilde_dsp_trampoline,
            Wrapped::float_convert_field_offset(),
        ),
        Some(hellodsp_tilde_free),
    );
    c.add_method(Method::Bang(hellodsp_tilde_bang_trampoline));

    let name = CString::new("blah").expect("CString::new failed");
    c.add_method(Method::SelF1(name, hellodsp_tilde_float_trampoline, 1));

    HELLODSP_CLASS = Some(c.into());
}
