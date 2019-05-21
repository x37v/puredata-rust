use crate::builder::{Builder, ExternalBuilder};
use crate::external::External;
use crate::obj::AsObject;

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
