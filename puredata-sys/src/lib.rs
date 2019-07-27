#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::fmt;

impl fmt::Display for &mut crate::_symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let c = std::ffi::CStr::from_ptr(self.s_name);
            if let Ok(c) = c.to_str() {
                write!(f, "{}", c)
            } else {
                Err(std::fmt::Error {})
            }
        }
    }
}

impl std::convert::From<std::ffi::CString> for &mut crate::_symbol {
    fn from(s: std::ffi::CString) -> Self {
        unsafe {
            let s = crate::gensym(s.as_ptr());
            &mut *s
        }
    }
}

impl crate::_symbol {
    pub fn as_ptr(&mut self) -> *mut crate::_symbol {
        self as *mut crate::_symbol
    }
}

impl std::cmp::PartialEq for crate::_symbol {
    fn eq(&self, other: &Self) -> bool {
        let s = self as *const _;
        let o = other as *const _;
        s == o
    }
}

impl std::cmp::Eq for crate::_symbol {}
