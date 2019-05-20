use crate::builder::{Builder, ExternalBuilder};
use crate::external::External;
use crate::obj::AsObject;
use crate::outlet::{Outlet, OutletSend, OutletType};

use std::ffi::CString;
use std::ops::Deref;
use std::rc::Rc;

#[repr(C)]
pub struct ExternalWrapper<T>
where
    T: External,
{
    x_obj: puredata_sys::t_object,
    external: Option<T>,
}

impl<T> ExternalWrapper<T>
where
    T: External,
{
    pub fn init(&mut self) {
        let mut builder = Builder::new(self);
        let e = External::new(&mut builder);
        self.external = Some(e);
    }
}

impl<T> AsObject for ExternalWrapper<T>
where
    T: External,
{
    fn as_obj(&mut self) -> *mut puredata_sys::t_object {
        &mut self.x_obj
    }
}

pub type Wrapped = ExternalWrapper<HelloWorldExternal>;

static mut HELLOWORLD_CLASS: Option<*mut puredata_sys::_class> = None;

pub struct HelloWorldExternal {
    inlet: Rc<dyn Deref<Target = puredata_sys::t_float>>,
    outlet: Rc<dyn OutletSend>,
}

impl External for HelloWorldExternal {
    fn new(builder: &mut dyn ExternalBuilder<Self>) -> Self {
        Self {
            inlet: builder.new_passive_float_inlet(4f32),
            outlet: builder.new_outlet(OutletType::Float),
        }
    }
}

pub unsafe extern "C" fn helloworld_new() -> *mut ::std::os::raw::c_void {
    let obj = std::mem::transmute::<*mut puredata_sys::t_pd, &mut Wrapped>(puredata_sys::pd_new(
        HELLOWORLD_CLASS.unwrap(),
    ));

    obj.init();

    //let _ = Outlet::new(OutletType::Bang, (*obj).as_obj());
    obj as *mut Wrapped as *mut ::std::os::raw::c_void
}

pub unsafe extern "C" fn helloworld_bang(_x: Wrapped) {
    let m = CString::new("YES").expect("CString::new failed");
    puredata_sys::post(m.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn helloworld_setup() {
    let name = CString::new("helloworld").expect("CString::new failed");
    let c = puredata_sys::class_new(
        puredata_sys::gensym(name.as_ptr()),
        Some(helloworld_new),
        None,
        std::mem::size_of::<Wrapped>(),
        0,
        0,
    );
    HELLOWORLD_CLASS = Some(c);
    puredata_sys::class_addbang(
        c,
        Some(std::mem::transmute::<
            unsafe extern "C" fn(Wrapped),
            unsafe extern "C" fn(),
        >(helloworld_bang)),
    );
}
