pub trait AsObject {
    fn as_obj(&mut self) -> *mut pd_sys::t_object;
}
