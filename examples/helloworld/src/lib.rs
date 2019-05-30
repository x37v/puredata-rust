use puredata_external::builder::ExternalBuilder;
use puredata_external::external::External;
use puredata_external::outlet::{OutletSend, OutletType};
use puredata_external::pd;
use puredata_external::wrapper::ExternalWrapper;

use std::ffi::CString;
use std::ops::Deref;
use std::rc::Rc;

pub type Wrapped = ExternalWrapper<HelloWorldExternal>;

static mut HELLOWORLD_CLASS: Option<*mut puredata_sys::_class> = None;

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
}

pub unsafe extern "C" fn helloworld_new() -> *mut ::std::os::raw::c_void {
    Wrapped::new(HELLOWORLD_CLASS.unwrap())
}

pub unsafe extern "C" fn helloworld_bang(x: &mut Wrapped) {
    //XXX sketchy, but works
    if let Some(e) = &mut x.external {
        e.bang();
    }
}

pub unsafe extern "C" fn helloworld_float(x: &mut Wrapped, arg: puredata_sys::t_float) {
    let m = CString::new(format!("got float {}", arg).to_string()).expect("CString::new failed");
    puredata_sys::post(m.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn helloworld_setup() {
    let name = CString::new("helloworld").expect("CString::new failed");
    let c = Wrapped::register(name, helloworld_new, None);
    HELLOWORLD_CLASS = Some(c);
    puredata_sys::class_addbang(
        c,
        Some(std::mem::transmute::<
            unsafe extern "C" fn(&mut Wrapped),
            unsafe extern "C" fn(),
        >(helloworld_bang)),
    );

    let name = CString::new("blah").expect("CString::new failed");
    puredata_sys::class_addmethod(
        c,
        Some(std::mem::transmute::<
            unsafe extern "C" fn(&mut Wrapped, puredata_sys::t_float),
            unsafe extern "C" fn(),
        >(helloworld_float)),
        puredata_sys::gensym(name.as_ptr()),
        puredata_sys::t_atomtype::A_DEFFLOAT,
        0,
    );
}
