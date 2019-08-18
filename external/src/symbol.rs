use std::fmt;
use std::ops::Deref;

#[repr(transparent)]
pub struct Symbol(*mut pd_sys::t_symbol);

impl Symbol {
    pub fn inner(&self) -> *mut pd_sys::t_symbol {
        self.0
    }
}

impl std::convert::TryFrom<*mut pd_sys::t_symbol> for Symbol {
    type Error = &'static str;
    fn try_from(s: *mut pd_sys::t_symbol) -> Result<Self, Self::Error> {
        if s.is_null() {
            Err("null ptr")
        } else {
            Ok(Self(s))
        }
    }
}

impl std::convert::From<std::ffi::CString> for Symbol {
    fn from(s: std::ffi::CString) -> Self {
        unsafe { Self(pd_sys::gensym(s.as_ptr())) }
    }
}

impl Into<&'static std::ffi::CStr> for Symbol {
    fn into(self) -> &'static std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr((*self.0).s_name) }
    }
}

impl Into<String> for Symbol {
    fn into(self) -> String {
        unsafe { std::ffi::CStr::from_ptr((*self.0).s_name) }
            .to_str()
            .unwrap()
            .to_owned()
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let c = std::ffi::CStr::from_ptr((*self.0).s_name);
            if let Ok(c) = c.to_str() {
                write!(f, "{}", c)
            } else {
                Err(std::fmt::Error {})
            }
        }
    }
}

impl std::cmp::PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        let s = self.0 as *const _;
        let o = other.0 as *const _;
        s == o
    }
}

impl std::cmp::Eq for Symbol {}

impl Copy for Symbol {}
impl Clone for Symbol {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
