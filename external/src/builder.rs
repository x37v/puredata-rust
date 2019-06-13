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
    fn new_passive_float_inlet(
        &mut self,
        initial_value: puredata_sys::t_float,
    ) -> Rc<dyn Deref<Target = puredata_sys::t_float>>;
    fn new_float_inlet(&mut self, func: Box<Fn(&mut T, puredata_sys::t_float)>);
    fn new_message_outlet(&mut self, t: OutletType) -> Rc<dyn OutletSend>;
    fn new_signal_inlet(&mut self) -> Rc<dyn InletSignal>;
    fn new_signal_outlet(&mut self) -> Rc<dyn OutletSignal>;
}

pub struct Builder<'a, T> {
    obj: &'a mut dyn AsObject,
    float_inlets: Vec<Box<Fn(&mut T, puredata_sys::t_float)>>,
}

pub struct SignalBuilder<'a, T> {
    obj: &'a mut dyn AsObject,
    float_inlets: Vec<Box<Fn(&mut T, puredata_sys::t_float)>>,
}

impl<'a, T> Builder<'a, T> {
    pub fn new(obj: &'a mut dyn AsObject) -> Self {
        Self {
            obj,
            float_inlets: Vec::new(),
        }
    }
}

impl<'a, T> SignalBuilder<'a, T> {
    pub fn new(obj: &'a mut dyn AsObject) -> Self {
        Self {
            obj,
            float_inlets: Vec::new(),
        }
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
