#[repr(transparent)]
pub struct Pointer(pub *mut pd_sys::t_gpointer);

#[repr(transparent)]
pub struct GList(pub *mut pd_sys::_glist);

#[repr(transparent)]
pub struct Array(pub *mut pd_sys::_array);

impl Pointer {
    pub fn check(&self, headok: bool) -> bool {
        unsafe { pd_sys::gpointer_check(self.0, if headok { 1 } else { 0 }) != 0 }
    }

    pub fn unset(&self) {
        unsafe {
            pd_sys::gpointer_unset(self.0);
        }
    }

    pub fn get_glist(&self) -> Option<GList> {
        unsafe {
            if (*(*self.0).gp_stub).gs_which == pd_sys::GP_GLIST as i32 {
                Some(GList((*((*self.0).gp_stub)).gs_un.gs_glist))
            } else {
                None
            }
        }
    }

    pub fn get_array(&self) -> Option<Array> {
        unsafe {
            if (*(*self.0).gp_stub).gs_which == pd_sys::GP_ARRAY as i32 {
                Some(Array((*((*self.0).gp_stub)).gs_un.gs_array))
            } else {
                None
            }
        }
    }
}
