use crate::builder::{Builder, SignalBuilder};
use crate::external::{External, SignalExternal};
use crate::obj::AsObject;

#[repr(C)]
pub struct ExternalWrapper<T>
where
    T: External,
{
    x_obj: puredata_sys::t_object,
    pub external: Option<T>,
}

#[repr(C)]
pub struct SignalExternalWrapper<T>
where
    T: SignalExternal,
{
    convert: puredata_sys::t_float, //intentionally at the start
    x_obj: puredata_sys::t_object,
    pub external: Option<T>,
}

impl<T> ExternalWrapper<T>
where
    T: External,
{
    pub unsafe fn new(pd_class: *mut puredata_sys::_class) -> *mut ::std::os::raw::c_void {
        let obj = std::mem::transmute::<*mut puredata_sys::t_pd, &mut Self>(puredata_sys::pd_new(
            pd_class,
        ));
        obj.init();
        obj as *mut Self as *mut ::std::os::raw::c_void
    }

    fn init(&mut self) {
        let mut builder = Builder::new(self);
        let e = External::new(&mut builder);
        self.external = Some(e);
    }

    pub fn wrapped(&mut self) -> &mut T {
        self.external.as_mut().expect("external not initialized")
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

impl<T> SignalExternalWrapper<T>
where
    T: SignalExternal,
{
    pub unsafe fn new(pd_class: *mut puredata_sys::_class) -> *mut ::std::os::raw::c_void {
        let obj = std::mem::transmute::<*mut puredata_sys::t_pd, &mut Self>(puredata_sys::pd_new(
            pd_class,
        ));
        obj.init();
        obj as *mut Self as *mut ::std::os::raw::c_void
    }

    fn init(&mut self) {
        let mut builder = SignalBuilder::new(self);
        let e = SignalExternal::new(&mut builder);
        self.external = Some(e);
    }

    pub fn wrapped(&mut self) -> &mut T {
        self.external.as_mut().expect("external not initialized")
    }

    pub fn dsp(&mut self, sv: *mut *mut puredata_sys::t_signal) {}
}

impl<T> AsObject for SignalExternalWrapper<T>
where
    T: SignalExternal,
{
    fn as_obj(&mut self) -> *mut puredata_sys::t_object {
        &mut self.x_obj
    }
}
