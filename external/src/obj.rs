pub trait AsObject {
    fn as_obj(&mut self) -> *mut puredata_sys::t_object;
}
