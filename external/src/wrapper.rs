use crate::builder::*;
use crate::external::*;
use crate::method::PdDspPerform;
use crate::obj::AsObject;
use crate::outlet::{OutletSignal, SignalOutlet};
use std::rc::Rc;
use std::slice;

struct ControlExternalWrapperInternal<T>
where
    T: ControlExternal,
{
    wrapped: T,
}

struct SignalGeneratorExternalWrapperInternal<T>
where
    T: SignalGeneratorExternal,
{
    wrapped: T,
    signal_outlets: Vec<Rc<dyn OutletSignal>>,
}

struct SignalProcessorExternalWrapperInternal<T>
where
    T: SignalProcessorExternal,
{
    wrapped: T,
    signal_outlets: Vec<Rc<dyn OutletSignal>>,
    signal_inlets: Vec<Rc<dyn InletSignal>>,
}

#[repr(C)]
pub struct ControlExternalWrapper<T>
where
    T: ControlExternal,
{
    x_obj: puredata_sys::t_object,
    wrapped: Option<ControlExternalWrapperInternal<T>>,
}

#[repr(C)]
pub struct SignalGeneratorExternalWrapper<T>
where
    T: SignalGeneratorExternal,
{
    x_obj: puredata_sys::t_object,
    wrapped: Option<SignalGeneratorExternalWrapperInternal<T>>,
}

#[repr(C)]
pub struct SignalProcessorExternalWrapper<T>
where
    T: SignalProcessorExternal,
{
    convert: puredata_sys::t_float, //intentionally at the start
    x_obj: puredata_sys::t_object,
    wrapped: Option<SignalGeneratorExternalWrapperInternal<T>>,
}

impl<T> ExternalWrapper<T>
where
    T: External,
{
    pub unsafe fn new(pd_class: *mut puredata_sys::_class) -> *mut ::std::os::raw::c_void {
        let obj = std::mem::transmute::<*mut puredata_sys::t_pd, &mut Self>(puredata_sys::pd_new(
            pd_class,
        ));
        obj.init();
        obj as *mut Self as *mut ::std::os::raw::c_void
    }

    fn init(&mut self) {
        let mut builder = Builder::new(self);
        let e = External::new(&mut builder);
        self.external = Some(e);
    }

    pub fn wrapped(&mut self) -> &mut T {
        self.external.as_mut().expect("external not initialized")
    }
}

impl<T> AsObject for ExternalWrapper<T>
where
    T: External,
{
    fn as_obj(&mut self) -> *mut puredata_sys::t_object {
        &mut self.x_obj
    }
}

impl<T> SignalExternalWrapper<T>
where
    T: SignalExternal,
{
    pub unsafe fn new(pd_class: *mut puredata_sys::_class) -> *mut ::std::os::raw::c_void {
        let obj = std::mem::transmute::<*mut puredata_sys::t_pd, &mut Self>(puredata_sys::pd_new(
            pd_class,
        ));
        obj.init()
    }

    fn init(&mut self) -> *mut ::std::os::raw::c_void {
        let mut builder = Builder::new(self);
        let e = SignalExternal::new(&mut builder);
        let dsp_inputs = builder.dsp_inputs();
        let dsp_outputs = builder.dsp_outputs();

        self.external = Some(e);
        self.dsp_inputs = dsp_inputs;
        self.dsp_outputs = dsp_outputs;

        //XXX allocate inputs/outputs
        //self.signal_outlet = Some(Rc::new(SignalOutlet::new(self)));

        let r = if dsp_inputs + dsp_outputs == 0 {
            //XXX indicate error?
            self.external = None;
            std::ptr::null_mut::<Self>()
        } else {
            self as *mut Self
        };
        r as *mut ::std::os::raw::c_void
    }

    pub fn wrapped(&mut self) -> &mut T {
        self.external.as_mut().expect("external not initialized")
    }

    pub fn dsp(&mut self, sv: *mut *mut puredata_sys::t_signal, trampoline: PdDspPerform) {
        unsafe {
            let dsp = self.dsp_inputs + self.dsp_outputs;
            let sv = slice::from_raw_parts(sv, dsp);
            let len = (*sv[0]).s_n as usize;

            //ptr to self, nframes, inputs, outputs
            let vecsize = 2 + dsp;
            let vecnbytes = vecsize * std::mem::size_of::<*mut puredata_sys::t_int>();
            let vecp = puredata_sys::getbytes(vecnbytes);
            let vec = std::mem::transmute::<_, *mut *mut puredata_sys::t_int>(vecp);
            assert!(!vecp.is_null(), "null pointer from puredata_sys::getbytes",);

            let vec: &mut [*mut puredata_sys::t_int] = slice::from_raw_parts_mut(vec, vecsize);
            vec[1] = std::mem::transmute::<_, _>(len);
            for i in 0..dsp {
                vec[2 + i] = std::mem::transmute::<_, _>((*sv[i]).s_vec);
            }

            vec[0] = std::mem::transmute::<_, _>(self);

            puredata_sys::dsp_addv(
                Some(trampoline),
                vecsize as std::os::raw::c_int,
                std::mem::transmute::<_, *mut puredata_sys::t_int>(vecp),
            );
            puredata_sys::freebytes(vecp, vecnbytes);
        }
    }

    pub fn perform(&mut self, w: *mut puredata_sys::t_int) -> *mut puredata_sys::t_int {
        unsafe {
            let dsp = self.dsp_inputs + self.dsp_outputs;

            let nframes = *std::mem::transmute::<_, *const usize>(w.offset(2));
            let input = std::mem::transmute::<_, *const puredata_sys::t_sample>(w.offset(3));
            let input = slice::from_raw_parts(input, nframes);
            self.wrapped().process(nframes, &[input], &mut []);
            w.offset((3 + dsp) as isize)
        }
    }
}

impl<T> AsObject for SignalExternalWrapper<T>
where
    T: SignalExternal,
{
    fn as_obj(&mut self) -> *mut puredata_sys::t_object {
        &mut self.x_obj
    }
}

impl<T> Drop for SignalExternalWrapper<T>
where
    T: SignalExternal,
{
    fn drop(&mut self) {
        //TODO
    }
}
