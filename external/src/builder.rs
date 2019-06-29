use crate::inlet::passive::FloatInlet;
use crate::inlet::InletSignal;
use crate::obj::AsObject;
use crate::outlet::{Outlet, OutletSend, OutletSignal, OutletType};
use std::ops::Deref;
use std::rc::Rc;

pub trait ExternalBuilder<T> {
    fn new_passive_float_inlet(
        &mut self,
        initial_value: puredata_sys::t_float,
    ) -> Rc<dyn Deref<Target = puredata_sys::t_float>>;
    fn new_float_inlet(&mut self, func: Box<Fn(&mut T, puredata_sys::t_float)>);
    fn new_message_outlet(&mut self, t: OutletType) -> Rc<dyn OutletSend>;
}

pub trait SignalExternalBuilder<T> {
    fn with_dsp(&mut self, inputs: usize, outputs: usize) -> &mut dyn ExternalBuilder<T>;
}

pub trait SignalGeneratorExternalBuilder<T>: ExternalBuilder<T> {
    fn new_signal_outlet(&mut self);
}

pub trait SignalProcessorExternalBuilder<T>: SignalGeneratorExternalBuilder<T> {
    fn new_signal_inlet(&mut self);
}

pub struct Builder<'a, T> {
    obj: &'a mut dyn AsObject,
    dsp_inputs: usize,
    dsp_outputs: usize,
    float_inlets: Vec<Box<Fn(&mut T, puredata_sys::t_float)>>,
}

impl<'a, T> Builder<'a, T> {
    pub fn new(obj: &'a mut dyn AsObject) -> Self {
        Self {
            obj,
            dsp_inputs: 0,
            dsp_outputs: 0,
            float_inlets: Vec::new(),
        }
    }

    pub fn dsp_inputs(&self) -> usize {
        self.dsp_inputs
    }

    pub fn dsp_outputs(&self) -> usize {
        self.dsp_outputs
    }
}

impl<'a, T> ExternalBuilder<T> for Builder<'a, T> {
    fn new_passive_float_inlet(
        &mut self,
        initial_value: puredata_sys::t_float,
    ) -> Rc<dyn Deref<Target = puredata_sys::t_float>> {
        Rc::new(FloatInlet::new(self.obj, initial_value))
    }

    fn new_float_inlet(&mut self, func: Box<Fn(&mut T, puredata_sys::t_float)>) {
        self.float_inlets.push(func);
    }

    fn new_message_outlet(&mut self, t: OutletType) -> Rc<dyn OutletSend> {
        Rc::new(Outlet::new(t, self.obj))
    }
}

impl<'a, T> SignalExternalBuilder<T> for Builder<'a, T> {
    fn with_dsp(&mut self, inputs: usize, outputs: usize) -> &mut dyn ExternalBuilder<T> {
        self.dsp_inputs = inputs;
        self.dsp_outputs = outputs;
        self as &mut dyn ExternalBuilder<T>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::boxed::Box;

    pub struct A;
    pub struct TestBuilder<T>
    where
        T: Sized,
    {
        float_inlets: Vec<Box<Fn(&mut T, puredata_sys::t_float)>>,
    }

    impl A {
        pub fn print_float(&mut self, f: puredata_sys::t_float) {
            println!("{}", f);
        }
    }

    impl<T> TestBuilder<T>
    where
        T: Sized,
    {
        pub fn call_floats<'a>(&mut self, a: &'a mut T, f: puredata_sys::t_float) {
            for cb in &self.float_inlets {
                (cb)(a, f);
            }
        }

        pub fn new() -> Self {
            Self {
                float_inlets: Vec::new(),
            }
        }
    }

    impl<T> ExternalBuilder<T> for TestBuilder<T> {
        fn new_passive_float_inlet(
            &mut self,
            initial_value: puredata_sys::t_float,
        ) -> Rc<dyn Deref<Target = puredata_sys::t_float>> {
            Rc::new(Box::new(initial_value))
        }

        fn new_float_inlet(&mut self, func: Box<Fn(&mut T, puredata_sys::t_float)>) {
            self.float_inlets.push(func);
        }

        fn new_outlet(&mut self, t: OutletType) -> Rc<dyn OutletSend> {
            Rc::new(())
        }
    }

    #[test]
    fn can_build() {
        let mut a = A;
        let mut builder = TestBuilder::new();

        builder.new_float_inlet(Box::new(|a: &mut A, f: puredata_sys::t_float| {
            a.print_float(f);
        }));
        builder.call_floats(&mut a, 4f32);
    }
}
