use puredata_external::builder::ControlExternalBuilder;
use puredata_external::class::Class;
use puredata_external::external::ControlExternal;
use puredata_external::method::Method;
use puredata_external::outlet::{OutletSend, OutletType};
use puredata_external::pd;
use puredata_external::wrapper::ControlExternalWrapper;

use std::ffi::CString;
use std::ops::Deref;
use std::rc::Rc;

pub type Wrapped = ControlExternalWrapper<HelloWorldExternal>;

static mut HELLOWORLD_CLASS: Option<*mut puredata_sys::_class> = None;

pub struct HelloWorldExternal {
    inlet: Rc<dyn Deref<Target = puredata_sys::t_float>>,
    //outlet: Rc<dyn OutletSend>,
}

impl ControlExternal for HelloWorldExternal {
    fn new(builder: &mut dyn ControlExternalBuilder<Self>) -> Self {
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

pub unsafe extern "C" fn helloworld_new() -> *mut ::std::os::raw::c_void {
    Wrapped::new(HELLOWORLD_CLASS.expect("hello world class not set"))
}

pub unsafe extern "C" fn helloworld_bang_trampoline(x: *mut Wrapped) {
    let x = &mut *x;
    x.wrapped().bang();
}

pub unsafe extern "C" fn helloworld_float_trampoline(x: *mut Wrapped, arg: puredata_sys::t_float) {
    let x = &mut *x;
    x.wrapped().float(arg);
}

#[no_mangle]
pub unsafe extern "C" fn helloworld_setup() {
    let name = CString::new("helloworld").expect("CString::new failed");
    let mut c = Class::<Wrapped>::register_new(name, helloworld_new, None);
    c.add_method(Method::Bang(helloworld_bang_trampoline));

    let name = CString::new("blah").expect("CString::new failed");
    c.add_method(Method::SelF1(name, helloworld_float_trampoline, 1));

    HELLOWORLD_CLASS = Some(c.into());
}
