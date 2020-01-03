#[repr(transparent)]
pub struct Pointer(pub *mut pd_sys::_gpointer);
