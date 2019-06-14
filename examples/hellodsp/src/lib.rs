use puredata_external::builder::SignalExternalBuilder;
use puredata_external::class::{Class, SignalClassType};
use puredata_external::external::SignalExternal;
use puredata_external::method::Method;
use puredata_external::outlet::{OutletSend, OutletType};
use puredata_external::pd;
use puredata_external::wrapper::SignalExternalWrapper;

use std::ffi::CString;
use std::ops::Deref;
use std::rc::Rc;
use std::slice;

pub type Wrapped = SignalExternalWrapper<HelloWorldExternal>;

static mut HELLODSP_CLASS: Option<*mut puredata_sys::_class> = None;

pub struct HelloWorldExternal {
    inlet: Rc<dyn Deref<Target = puredata_sys::t_float>>,
    //outlet: Rc<dyn OutletSend>,
}

impl SignalExternal for HelloWorldExternal {
    fn new(builder: &mut dyn SignalExternalBuilder<Self>) -> Self {
        Self {
            inlet: builder.new_passive_float_inlet(4f32),
            //outlet: builder.new_outlet(OutletType::Float),
        }
    }
    fn process(
        &mut self,
        inputs: &[&[puredata_sys::t_float]],
        outputs: &[&mut [puredata_sys::t_float]],
    ) {
        println!(
            "process!! {} {} {}",
            inputs.len(),
            outputs.len(),
            inputs[0].len()
        );
    }
}

impl HelloWorldExternal {
    pub fn bang(&mut self) {
        let m = CString::new(format!("hello {}", **self.inlet).to_string())
            .expect("CString::new failed");
        pd::post(m);
    }
    pub fn float(&mut self, arg: puredata_sys::t_float) {
        let m =
            CString::new(format!("got float {}", arg).to_string()).expect("CString::new failed");
        pd::post(m);
    }
}

pub unsafe extern "C" fn hellodsp_tilde_new() -> *mut ::std::os::raw::c_void {
    Wrapped::new(HELLODSP_CLASS.expect("hello dsp class not set"))
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
    let x = std::mem::transmute::<_, *mut Wrapped>(w.offset(1));
    let x = &mut *x;
    let nframes = *std::mem::transmute::<_, *const usize>(w.offset(2));
    let input = std::mem::transmute::<_, *const puredata_sys::t_sample>(w.offset(3));
    let input = slice::from_raw_parts(input, nframes);
    x.wrapped().process(&[input], &mut []);
    w.offset(4)
}

#[no_mangle]
pub unsafe extern "C" fn hellodsp_tilde_setup() {
    let name = CString::new("hellodsp~").expect("CString::new failed");
    let mut c = Class::<Wrapped>::register_dsp_new(
        name,
        hellodsp_tilde_new,
        SignalClassType::WithInput(hellodsp_tilde_dsp_trampoline, 0),
        None,
    );
    c.add_method(Method::Bang(hellodsp_tilde_bang_trampoline));

    let name = CString::new("blah").expect("CString::new failed");
    c.add_method(Method::SelF1(name, hellodsp_tilde_float_trampoline, 1));

    HELLODSP_CLASS = Some(c.into());
}
