use std::mem::size_of;
use std::ops::{Deref, DerefMut};
use std::slice;

///A slice allocated and freed using pd_sys
#[repr(transparent)]
pub struct Slice<T: 'static + Sized + Copy>(pub &'static mut [T]);

impl<T> Slice<T> where T: 'static + Copy {}

impl<T> Default for Slice<T>
where
    T: 'static + Sized + Copy,
{
    fn default() -> Self {
        Self(&mut [])
    }
}

impl<T> Slice<T>
where
    T: 'static + Sized + Copy + Default,
{
    /// Create a new slice.
    ///
    /// # Arguments
    ///
    /// * `len` - the new slice length.
    ///
    /// # Remarks
    ///
    /// This will set all the current content to `Default::default()` for `T`.
    pub fn new(len: usize) -> Self {
        let mut s = Self::default();
        s.resize(len);
        s
    }

    /// Resize the slice.
    ///
    /// # Arguments
    ///
    /// * `len` - the new slice length.
    ///
    /// # Remarks
    ///
    /// If actually resized, this will set all the content to `Default::default()` for `T`.
    pub fn resize(&mut self, len: usize) {
        if self.0.len() != len {
            //TODO use resizebytes?
            self.cleanup();
            self.alloc(len);
        }
    }

    fn alloc(&mut self, len: usize) {
        if len == 0 {
            self.0 = &mut [];
        } else {
            unsafe {
                let bytes = pd_sys::getbytes(len * size_of::<T>());
                let bytes = std::mem::transmute::<_, *mut T>(bytes);
                self.0 = slice::from_raw_parts_mut(bytes, len);
            }
            for i in self.0.iter_mut() {
                *i = Default::default();
            }
        }
    }
}

impl<T> Deref for Slice<T>
where
    T: 'static + Sized + Copy,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Slice<T>
where
    T: 'static + Sized + Copy,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Slice<T>
where
    T: 'static + Sized + Copy,
{
    fn cleanup(&mut self) {
        if self.0.len() > 0 {
            unsafe {
                pd_sys::freebytes(
                    self.0.as_mut_ptr() as *mut ::std::os::raw::c_void,
                    self.0.len() * size_of::<T>(),
                );
                self.0 = slice::from_raw_parts_mut(std::ptr::null_mut(), 0)
            }
        }
    }
}

impl<T> Drop for Slice<T>
where
    T: 'static + Sized + Copy,
{
    fn drop(&mut self) {
        self.cleanup();
    }
}
