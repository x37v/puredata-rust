use crate::builder::Builder;
use crate::external::External;
use crate::obj::AsObject;
use std::ffi::CString;

#[repr(C)]
pub struct ExternalWrapper<T>
where
    T: External,
{
    x_obj: puredata_sys::t_object,
    pub external: Option<T>, //XXX TMP
}

impl<T> ExternalWrapper<T>
where
    T: External,
{
    pub fn new(pd_class: *mut puredata_sys::_class) -> *mut ::std::os::raw::c_void {
        unsafe {
            let obj = std::mem::transmute::<*mut puredata_sys::t_pd, &mut Self>(
                puredata_sys::pd_new(pd_class),
            );
            obj.init();
            obj as *mut Self as *mut ::std::os::raw::c_void
        }
    }

    pub fn init(&mut self) {
        let mut builder = Builder::new(self);
        let e = External::new(&mut builder);
        self.external = Some(e);
    }

    pub fn register(
        name: CString,
        creator: unsafe extern "C" fn() -> *mut ::std::os::raw::c_void,
        destroyer: Option<unsafe extern "C" fn()>,
    ) -> *mut puredata_sys::_class {
        unsafe {
            puredata_sys::class_new(
                puredata_sys::gensym(name.as_ptr()),
                Some(creator),
                destroyer,
                std::mem::size_of::<ExternalWrapper<T>>(),
                0,
                0,
            )
        }
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
