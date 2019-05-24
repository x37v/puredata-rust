pub mod atom;
pub mod args;
pub mod builder;
pub mod class;
pub mod external;
pub mod inlet;
pub mod method;
pub mod obj;
pub mod outlet;
pub mod pd;
pub mod wrapper;

/*
pub trait Bang {
    fn bang(&mut self);
}

pub trait Signal {
    fn perform(
        &mut self,
        inputs: &[&[puredata_sys::t_sample]],
        outputs: &[&mut [puredata_sys::t_sample]],
    );
}

pub trait WithBangMethod<T> {
    fn got_bang(&mut self, v: T);
}

pub trait WithFloat<T> {
    fn got_float(&mut self, v: T, f: puredata_sys::t_float);
}

static mut HELLOWORLD_CLASS: Option<*mut puredata_sys::_class> = None;

#[repr(C)]
pub struct t_helloworld {
    x_obj: puredata_sys::t_object,
}

impl AsObject for t_helloworld {
    fn as_obj(&mut self) -> *mut puredata_sys::t_object {
        &mut self.x_obj
    }
}

pub unsafe extern "C" fn helloworld_bang(_x: t_helloworld) {
    let m = CString::new("Hello, from RUST world !!").expect("CString::new failed");
    puredata_sys::post(m.as_ptr());
}

pub unsafe extern "C" fn helloworld_new() -> *mut ::std::os::raw::c_void {
    let obj = std::mem::transmute::<*mut puredata_sys::t_pd, *mut t_helloworld>(
        puredata_sys::pd_new(HELLOWORLD_CLASS.unwrap()),
    );
    //let _ = Outlet::new(OutletType::Bang, (*obj).as_obj());
    obj as *mut ::std::os::raw::c_void
}

#[no_mangle]
pub unsafe extern "C" fn helloworld_setup() {
    let name = CString::new("helloworld").expect("CString::new failed");
    let c = puredata_sys::class_new(
        puredata_sys::gensym(name.as_ptr()),
        Some(helloworld_new),
        None,
        std::mem::size_of::<t_helloworld>(),
        0,
        0,
    );
    HELLOWORLD_CLASS = Some(c);
    puredata_sys::class_addbang(
        c,
        Some(std::mem::transmute::<
            unsafe extern "C" fn(t_helloworld),
            unsafe extern "C" fn(),
        >(helloworld_bang)),
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/
