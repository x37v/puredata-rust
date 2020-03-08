use crate::obj::AsObject;

pub trait PdPost {
    fn post(&self, s: String);
    fn post_error(&self, s: String);
}

pub struct Post {
    x_obj: *mut pd_sys::t_object,
}

impl Post {
    pub fn new(owner: &mut dyn AsObject) -> Self {
        Self {
            x_obj: owner.as_obj(),
        }
    }
    pub fn error(s: String) {
        for l in s.lines() {
            let c = std::ffi::CString::new(l);
            if let Ok(c) = c {
                unsafe {
                    pd_sys::error(c.as_ptr());
                }
            }
        }
    }
}

impl PdPost for Post {
    fn post(&self, s: String) {
        for l in s.lines() {
            let c = std::ffi::CString::new(l);
            if let Ok(c) = c {
                unsafe {
                    pd_sys::post(c.as_ptr());
                }
            }
        }
    }
    fn post_error(&self, s: String) {
        for l in s.lines() {
            let c = std::ffi::CString::new(l);
            if let Ok(c) = c {
                unsafe {
                    pd_sys::pd_error(std::mem::transmute::<_, _>(self.x_obj), c.as_ptr());
                }
            }
        }
    }
}
