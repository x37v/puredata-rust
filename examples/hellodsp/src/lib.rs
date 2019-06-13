use puredata_external::builder::ExternalBuilder;
use puredata_external::class::{Class, SignalClassType};
use puredata_external::external::External;
use puredata_external::method::Method;
use puredata_external::outlet::{OutletSend, OutletType};
use puredata_external::pd;
use puredata_external::wrapper::ExternalWrapper;

use std::ffi::CString;
use std::ops::Deref;
use std::rc::Rc;

pub type Wrapped = ExternalWrapper<HelloWorldExternal>;

static mut HELLODSP_CLASS: Option<*mut puredata_sys::_class> = None;

pub struct HelloWorldExternal {
    inlet: Rc<dyn Deref<Target = puredata_sys::t_float>>,
    //outlet: Rc<dyn OutletSend>,
}

impl External for HelloWorldExternal {
    fn new(builder: &mut dyn ExternalBuilder<Self>) -> Self {
        Self {
            inlet: builder.new_passive_float_inlet(4f32),
            //outlet: builder.new_outlet(OutletType::Float),
        }
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

pub unsafe extern "C" fn hellodsp_new() -> *mut ::std::os::raw::c_void {
    Wrapped::new(HELLODSP_CLASS.expect("hello dsp class not set"))
}

pub unsafe extern "C" fn hellodsp_bang_trampoline(x: *mut Wrapped) {
    let x = &mut *x;
    x.wrapped().bang();
}

pub unsafe extern "C" fn hellodsp_float_trampoline(x: *mut Wrapped, arg: puredata_sys::t_float) {
    let x = &mut *x;
    x.wrapped().float(arg);
}

pub unsafe extern "C" fn hellodsp_dsp_trampoline(
    x: *mut Wrapped,
    sp: *mut *mut puredata_sys::t_signal,
) {
    let x = &mut *x;
    x.dsp(sp);
}

#[no_mangle]
pub unsafe extern "C" fn hellodsp_setup() {
    let name = CString::new("hellodsp").expect("CString::new failed");
    let mut c = Class::<Wrapped>::register_dsp_new(
        name,
        hellodsp_new,
        SignalClassType::WithInput(hellodsp_dsp_trampoline, 0),
        None,
    );
    c.add_method(Method::Bang(hellodsp_bang_trampoline));

    let name = CString::new("blah").expect("CString::new failed");
    c.add_method(Method::SelF1(name, hellodsp_float_trampoline, 1));

    HELLODSP_CLASS = Some(c.into());
}
