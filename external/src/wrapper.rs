use crate::builder::*;
use crate::external::*;
use crate::inlet::InletSignal;
use crate::method::PdDspPerform;
use crate::obj::AsObject;
use crate::outlet::{OutletSignal, SignalOutlet};
use std::rc::Rc;
use std::slice;

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
    x_obj: puredata_sys::t_object,
    pub convert: puredata_sys::t_float,
    wrapped: Option<SignalProcessorExternalWrapperInternal<T>>,
}

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
    signal_outlets: Vec<RcOutletSignal>,
    outlet_buffer: Vec<&'static mut [puredata_sys::t_float]>,
}

struct SignalProcessorExternalWrapperInternal<T>
where
    T: SignalProcessorExternal,
{
    wrapped: T,
    signal_outlets: Vec<RcOutletSignal>,
    signal_inlets: Vec<RcInletSignal>,
    outlet_buffer: Vec<&'static mut [puredata_sys::t_float]>,
    inlet_buffer: Vec<&'static [puredata_sys::t_float]>,
}

impl<T> ControlExternalWrapperInternal<T>
where
    T: ControlExternal,
{
    pub fn new<'a>(wrapped: T, _builder: Builder<T>) -> Self {
        Self { wrapped }
    }

    pub fn wrapped(&mut self) -> &mut T {
        &mut self.wrapped
    }
}

impl<T> SignalGeneratorExternalWrapperInternal<T>
where
    T: SignalGeneratorExternal,
{
    pub fn new<'a>(wrapped: T, builder: Builder<T>) -> Self {
        let temp: IntoBuiltGenerator<T> = builder.into();
        let outlets = temp.1.len();
        let mut outlet_buffer = Vec::new();
        unsafe {
            for _ in 0..outlets {
                outlet_buffer.push(slice::from_raw_parts_mut(std::ptr::null_mut(), 0));
            }
        }
        Self {
            wrapped,
            signal_outlets: temp.1,
            outlet_buffer,
        }
    }

    pub fn wrapped(&mut self) -> &mut T {
        &mut self.wrapped
    }

    pub fn signal_iolets(&self) -> usize {
        self.signal_outlets.len()
    }

    pub fn generate(&mut self, nframes: usize, buffer: *mut puredata_sys::t_int) {
        //assign the slices
        unsafe {
            let outlets = self.outlet_buffer.len();
            let buffer = slice::from_raw_parts(buffer, outlets);
            for i in 0..outlets {
                let output = std::mem::transmute::<_, *mut puredata_sys::t_sample>(buffer[i]);
                let output = slice::from_raw_parts_mut(output, nframes);
                self.outlet_buffer[i] = output;
            }
        }
        let output_slice = self.outlet_buffer.as_mut();
        self.wrapped.generate(nframes, output_slice);
    }
}

impl<T> SignalProcessorExternalWrapperInternal<T>
where
    T: SignalProcessorExternal,
{
    pub fn new<'a>(wrapped: T, builder: Builder<T>) -> Self {
        let temp: IntoBuiltProcessor<T> = builder.into();
        let inlets = temp.2.len() + 1; //one default
        let outlets = temp.1.len();
        let mut inlet_buffer = Vec::new();
        let mut outlet_buffer = Vec::new();

        println!(
            "SignalProcessorExternalWrapperInternal {} {}",
            inlets, outlets
        );

        //reserve space for slices, 0 len for now
        unsafe {
            for _ in 0..inlets {
                inlet_buffer.push(slice::from_raw_parts(std::ptr::null(), 0));
            }
            for _ in 0..outlets {
                outlet_buffer.push(slice::from_raw_parts_mut(std::ptr::null_mut(), 0));
            }
        }
        Self {
            wrapped,
            signal_outlets: temp.1,
            signal_inlets: temp.2,
            inlet_buffer,
            outlet_buffer,
        }
    }

    pub fn wrapped(&mut self) -> &mut T {
        &mut self.wrapped
    }

    pub fn signal_iolets(&self) -> usize {
        self.inlet_buffer.len() + self.outlet_buffer.len()
    }

    pub fn process(&mut self, nframes: usize, buffer: *mut puredata_sys::t_int) {
        let inlets = self.inlet_buffer.len();
        let outlets = self.outlet_buffer.len();
        //assign the slices
        //inputs first
        unsafe {
            let buffer = slice::from_raw_parts(buffer, inlets + outlets);
            for i in 0..inlets {
                let input = std::mem::transmute::<_, *const puredata_sys::t_sample>(buffer[i]);
                let input = slice::from_raw_parts(input, nframes);
                self.inlet_buffer[i] = input;
            }

            let offset = inlets;
            for i in 0..outlets {
                let output =
                    std::mem::transmute::<_, *mut puredata_sys::t_sample>(buffer[i + offset]);
                let output = slice::from_raw_parts_mut(output, nframes);
                self.outlet_buffer[i] = output;
            }
        }
        println!(
            "wrapper::process inlets {} outlets {} nframes {}",
            self.inlet_buffer.len(),
            self.outlet_buffer.len(),
            nframes
        );
        let output_slice = self.outlet_buffer.as_mut();
        let input_slice = self.inlet_buffer.as_ref();
        self.wrapped.process(nframes, input_slice, output_slice);
    }
}

impl<T> ControlExternalWrapper<T>
where
    T: ControlExternal,
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
        let e = ControlExternal::new(&mut builder);
        self.wrapped = Some(ControlExternalWrapperInternal::new(e, builder));
    }

    pub fn wrapped(&mut self) -> &mut T {
        self.wrapped
            .as_mut()
            .expect("external not initialized")
            .wrapped()
    }
}

impl<T> SignalGeneratorExternalWrapper<T>
where
    T: SignalGeneratorExternal,
{
    pub unsafe fn new(pd_class: *mut puredata_sys::_class) -> *mut ::std::os::raw::c_void {
        let obj = std::mem::transmute::<*mut puredata_sys::t_pd, &mut Self>(puredata_sys::pd_new(
            pd_class,
        ));
        obj.init()
    }

    fn init(&mut self) -> *mut ::std::os::raw::c_void {
        let mut builder = Builder::new(self);
        let e = SignalGeneratorExternal::new(&mut builder);
        //make sure we have some output
        let r = if builder.signal_outlets() == 0 {
            //XXX indicate error
            std::ptr::null_mut::<Self>()
        } else {
            self.wrapped = Some(SignalGeneratorExternalWrapperInternal::new(e, builder));
            self as *mut Self
        };
        r as *mut ::std::os::raw::c_void
    }

    pub fn wrapped(&mut self) -> &mut T {
        self.wrapped
            .as_mut()
            .expect("external not initialized")
            .wrapped()
    }

    pub fn signal_iolets(&self) -> usize {
        self.wrapped
            .as_ref()
            .expect("external not initialized")
            .signal_iolets()
    }

    pub fn dsp(&mut self, sv: *mut *mut puredata_sys::t_signal, trampoline: PdDspPerform) {
        let iolets = self.signal_iolets();
        setup_dsp(self, iolets, sv, trampoline);
    }

    pub fn perform(&mut self, w: *mut puredata_sys::t_int) -> *mut puredata_sys::t_int {
        unsafe {
            let iolets = self.signal_iolets();
            let nframes = *std::mem::transmute::<_, *const usize>(w.offset(2));
            self.wrapped
                .as_mut()
                .expect("external not initialized")
                .generate(nframes, w.offset(3));
            w.offset((3 + iolets) as isize)
        }
    }
}

impl<T> SignalProcessorExternalWrapper<T>
where
    T: SignalProcessorExternal,
{
    pub unsafe fn new(pd_class: *mut puredata_sys::_class) -> *mut ::std::os::raw::c_void {
        let obj = std::mem::transmute::<*mut puredata_sys::t_pd, &mut Self>(puredata_sys::pd_new(
            pd_class,
        ));
        println!("SignalProcessorExternalWrapper::new({:p})", &mut obj.x_obj);
        obj.init();

        obj as *mut Self as *mut ::std::os::raw::c_void
    }

    fn init(&mut self) {
        let mut builder = Builder::new(self);
        let e = SignalProcessorExternal::new(&mut builder);
        self.wrapped = Some(SignalProcessorExternalWrapperInternal::new(e, builder));
    }

    pub fn wrapped(&mut self) -> &mut T {
        self.wrapped
            .as_mut()
            .expect("external not initialized")
            .wrapped()
    }

    pub fn signal_iolets(&self) -> usize {
        self.wrapped
            .as_ref()
            .expect("external not initialized")
            .signal_iolets()
    }

    pub fn dsp(&mut self, sv: *mut *mut puredata_sys::t_signal, trampoline: PdDspPerform) {
        let iolets = self.signal_iolets();
        setup_dsp(self, iolets, sv, trampoline);
    }

    pub fn perform(&mut self, w: *mut puredata_sys::t_int) -> *mut puredata_sys::t_int {
        unsafe {
            let iolets = self.signal_iolets();
            let nframes = *std::mem::transmute::<_, *const usize>(w.offset(2));
            self.wrapped
                .as_mut()
                .expect("external not initialized")
                .process(nframes, w.offset(3));
            w.offset((3 + iolets) as isize)
        }
    }
}

fn setup_dsp<T>(
    obj: &mut T,
    iolets: usize,
    sv: *mut *mut puredata_sys::t_signal,
    trampoline: PdDspPerform,
) {
    unsafe {
        let sv = slice::from_raw_parts(sv, iolets);
        let len = (*sv[0]).s_n as usize;

        //ptr to self, nframes, inputs, outputs
        let vecsize = 2 + iolets;
        let vecnbytes = vecsize * std::mem::size_of::<*mut puredata_sys::t_int>();
        let vecp = puredata_sys::getbytes(vecnbytes);
        let vec = std::mem::transmute::<_, *mut *mut puredata_sys::t_int>(vecp);
        assert!(!vecp.is_null(), "null pointer from puredata_sys::getbytes",);

        let vec: &mut [*mut puredata_sys::t_int] = slice::from_raw_parts_mut(vec, vecsize);
        vec[1] = std::mem::transmute::<_, _>(len);
        for i in 0..iolets {
            vec[2 + i] = std::mem::transmute::<_, _>((*sv[i]).s_vec);
        }

        vec[0] = std::mem::transmute::<_, _>(obj);

        puredata_sys::dsp_addv(
            Some(trampoline),
            vecsize as std::os::raw::c_int,
            std::mem::transmute::<_, *mut puredata_sys::t_int>(vecp),
        );
        puredata_sys::freebytes(vecp, vecnbytes);
    }
}

impl<T> AsObject for ControlExternalWrapper<T>
where
    T: ControlExternal,
{
    fn as_obj(&mut self) -> *mut puredata_sys::t_object {
        &mut self.x_obj
    }
}

impl<T> AsObject for SignalGeneratorExternalWrapper<T>
where
    T: SignalGeneratorExternal,
{
    fn as_obj(&mut self) -> *mut puredata_sys::t_object {
        &mut self.x_obj
    }
}

impl<T> AsObject for SignalProcessorExternalWrapper<T>
where
    T: SignalProcessorExternal,
{
    fn as_obj(&mut self) -> *mut puredata_sys::t_object {
        &mut self.x_obj
    }
}

impl<T> Drop for SignalGeneratorExternalWrapper<T>
where
    T: SignalGeneratorExternal,
{
    fn drop(&mut self) {
        //TODO
    }
}

impl<T> Drop for SignalProcessorExternalWrapper<T>
where
    T: SignalProcessorExternal,
{
    fn drop(&mut self) {
        //TODO
    }
}
