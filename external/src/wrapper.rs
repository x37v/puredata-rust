use crate::builder::{Builder, SignalBuilder};
use crate::external::{External, SignalExternal};
use crate::method::PdDspPerform;
use crate::obj::AsObject;
use std::slice;

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

    pub fn dsp(&mut self, sv: *mut *mut puredata_sys::t_signal, trampoline: PdDspPerform) {
        unsafe {
            let sv = slice::from_raw_parts(sv, 1);
            let len = (*sv[0]).s_n as usize;

            let vecsize = 3usize;
            let vecbytes = std::mem::transmute::<_, *mut *mut puredata_sys::t_int>(
                puredata_sys::getbytes(vecsize * std::mem::size_of::<puredata_sys::t_int>()),
            );

            //XXX fixed to 1 input for now
            let vec = slice::from_raw_parts_mut(vecbytes, vecsize);
            vec[0] = std::mem::transmute::<_, _>(self);
            vec[1] = std::mem::transmute::<_, _>(len);
            vec[2] = std::mem::transmute::<_, _>((*sv[0]).s_vec);
            puredata_sys::dsp_addv(
                Some(trampoline),
                vecsize as std::os::raw::c_int,
                std::mem::transmute::<_, *mut puredata_sys::t_int>(vecbytes),
            );
        }
    }
}

impl<T> AsObject for SignalExternalWrapper<T>
where
    T: SignalExternal,
{
    fn as_obj(&mut self) -> *mut puredata_sys::t_object {
        &mut self.x_obj
    }
}
