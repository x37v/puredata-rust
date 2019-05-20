use crate::external::External;

#[repr(C)]
pub struct ExternalWrapper<T>
where
    T: External,
{
    x_obj: puredata_sys::t_object,
    external: T,
}

impl<T> ExternalWrapper<T>
where
    T: External,
{
    pub fn as_obj(&mut self) -> *mut puredata_sys::t_object {
        &mut self.x_obj
    }
}
