#[repr(C)]
pub struct ExternalWrapper<T> {
    x_obj: puredata_sys::t_object,
    obj: T,
}

impl<T> ExternalWrapper<T> {
    pub fn as_obj(&mut self) -> *mut puredata_sys::t_object {
        &mut self.x_obj
    }
}
