use std::fmt;

use std::ffi::OsStr;

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

#[repr(transparent)]
pub struct Symbol(*mut pd_sys::t_symbol);

impl Symbol {
    pub fn inner(&self) -> *mut pd_sys::t_symbol {
        self.0
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
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

impl std::convert::TryFrom<&'static str> for Symbol {
    type Error = &'static str;
    fn try_from(s: &'static str) -> Result<Self, Self::Error> {
        match std::ffi::CString::new(s) {
            Ok(i) => Ok(i.into()),
            Err(_) => Err("fail"),
        }
    }
}

impl std::convert::From<std::ffi::CString> for Symbol {
    fn from(s: std::ffi::CString) -> Self {
        unsafe { Self(pd_sys::gensym(s.as_ptr())) }
    }
}

impl<'a> std::convert::Into<&'a std::ffi::CStr> for &Symbol {
    fn into(self) -> &'a std::ffi::CStr {
        unsafe { std::ffi::CStr::from_ptr((*self.0).s_name) }
    }
}

//https://stackoverflow.com/questions/46342644/how-can-i-get-a-path-from-a-raw-c-string-cstr-or-const-u8
#[cfg(unix)]
impl<'a> std::convert::Into<&'a std::path::Path> for &Symbol {
    fn into(self) -> &'a std::path::Path {
        let slice: &std::ffi::CStr = self.into();
        let osstr = OsStr::from_bytes(slice.to_bytes());
        osstr.as_ref()
    }
}

#[cfg(windows)]
impl<'a> std::convert::Into<&'a std::path::Path> for &Symbol {
    fn into(self) -> &'a std::path::Path {
        let slice: &std::ffi::CStr = self.into();
        let str = ::std::str::from_utf8(slice.to_bytes())
            .expect("failed to get utf8 from puredata::symbol");
        let path: &Path = str.as_ref();
    }
}

impl std::convert::AsRef<std::ffi::CStr> for Symbol {
    fn as_ref(&self) -> &std::ffi::CStr {
        self.into()
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

impl std::convert::AsRef<std::path::Path> for Symbol {
    fn as_ref(&self) -> &std::path::Path {
        self.into()
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

//pd sends symbols between threads, must be thread safe
unsafe impl Send for Symbol {}
unsafe impl Sync for Symbol {}
