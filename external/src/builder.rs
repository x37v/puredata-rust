use crate::atom::Atom;
use crate::inlet::passive::FloatInlet;
use crate::inlet::*;
use crate::obj::AsObject;
use crate::outlet::{Outlet, OutletSend, OutletSignal, OutletType, SignalOutlet};
use crate::post::{PdPost, Post};
use crate::symbol::Symbol;
use std::ops::Deref;

pub type BoxInletSignal = Box<dyn InletSignal>;
pub type BoxOutletSignal = Box<dyn OutletSignal>;
pub type BoxFloatInlet<T> = Box<dyn Fn(&mut T, pd_sys::t_float)>;

pub type IntoBuiltControl<T> = Vec<BoxFloatInlet<T>>;
pub type IntoBuiltGenerator<T> = (Vec<BoxFloatInlet<T>>, Vec<BoxOutletSignal>);
pub type IntoBuiltProcessor<T> = (
    Vec<BoxFloatInlet<T>>,
    Vec<BoxOutletSignal>,
    Vec<BoxInletSignal>,
);

pub trait ControlExternalBuilder<T> {
    fn obj(&mut self) -> &mut dyn AsObject;
    fn poster(&mut self) -> Box<dyn PdPost>;
    fn instance_name(&self) -> &Option<Symbol>;
    fn creation_args(&self) -> &[Atom];
    fn new_passive_float_inlet(
        &mut self,
        initial_value: pd_sys::t_float,
    ) -> Box<dyn Deref<Target = pd_sys::t_float>>;
    fn new_float_inlet(&mut self, func: Box<dyn Fn(&mut T, pd_sys::t_float)>);
    fn new_message_outlet(&mut self, t: OutletType) -> Box<dyn OutletSend>;
}

pub trait SignalGeneratorExternalBuilder<T>: ControlExternalBuilder<T> {
    fn new_signal_outlet(&mut self);
}

pub trait SignalProcessorExternalBuilder<T>: SignalGeneratorExternalBuilder<T> {
    fn new_signal_inlet(&mut self);
}

pub struct Builder<'a, T> {
    obj: &'a mut dyn AsObject,
    args: &'a [Atom],
    name: Option<Symbol>,
    signal_inlets: Vec<BoxInletSignal>,
    signal_outlets: Vec<BoxOutletSignal>,
    float_inlets: Vec<Box<dyn Fn(&mut T, pd_sys::t_float)>>,
}

impl<'a, T> Builder<'a, T> {
    pub fn new(obj: &'a mut dyn AsObject, args: &'a [Atom], name: Option<Symbol>) -> Self {
        Self {
            obj,
            name,
            args,
            signal_inlets: Vec::new(),
            signal_outlets: Vec::new(),
            float_inlets: Vec::new(),
        }
    }

    pub fn signal_inlets(&self) -> usize {
        self.signal_inlets.len()
    }

    pub fn signal_outlets(&self) -> usize {
        self.signal_outlets.len()
    }
}

impl<'a, T> Into<IntoBuiltControl<T>> for Builder<'a, T> {
    fn into(self) -> IntoBuiltControl<T> {
        (self.float_inlets)
    }
}

impl<'a, T> Into<IntoBuiltGenerator<T>> for Builder<'a, T> {
    fn into(self) -> IntoBuiltGenerator<T> {
        (self.float_inlets, self.signal_outlets)
    }
}

impl<'a, T> Into<IntoBuiltProcessor<T>> for Builder<'a, T> {
    fn into(self) -> IntoBuiltProcessor<T> {
        (self.float_inlets, self.signal_outlets, self.signal_inlets)
    }
}

impl<'a, T> ControlExternalBuilder<T> for Builder<'a, T> {
    fn obj(&mut self) -> &mut dyn AsObject {
        self.obj
    }

    fn poster(&mut self) -> Box<dyn PdPost> {
        Box::new(Post::new(self.obj()))
    }

    fn instance_name(&self) -> &Option<Symbol> {
        &self.name
    }

    fn creation_args(&self) -> &[Atom] {
        &self.args
    }

    fn new_passive_float_inlet(
        &mut self,
        initial_value: pd_sys::t_float,
    ) -> Box<dyn Deref<Target = pd_sys::t_float>> {
        Box::new(FloatInlet::new(self.obj, initial_value))
    }

    fn new_float_inlet(&mut self, func: Box<dyn Fn(&mut T, pd_sys::t_float)>) {
        self.float_inlets.push(func);
    }

    fn new_message_outlet(&mut self, t: OutletType) -> Box<dyn OutletSend> {
        Box::new(Outlet::new(t, self.obj))
    }
}

impl<'a, T> SignalGeneratorExternalBuilder<T> for Builder<'a, T> {
    fn new_signal_outlet(&mut self) {
        self.signal_outlets
            .push(Box::new(SignalOutlet::new(self.obj)));
    }
}

impl<'a, T> SignalProcessorExternalBuilder<T> for Builder<'a, T> {
    fn new_signal_inlet(&mut self) {
        self.signal_inlets
            .push(Box::new(SignalInlet::new(self.obj)));
    }
}

#[cfg(test)]
mod tests {

    /*
    pub struct A;
    pub struct TestBuilder<T>
    where
        T: Sized,
    {
        float_inlets: Vec<Box<Fn(&mut T, pd_sys::t_float)>>,
    }

    impl A {
        pub fn print_float(&mut self, f: pd_sys::t_float) {
            println!("{}", f);
        }
    }

    impl<T> TestBuilder<T>
    where
        T: Sized,
    {
        pub fn call_floats<'a>(&mut self, a: &'a mut T, f: pd_sys::t_float) {
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
            initial_value: pd_sys::t_float,
        ) -> Box<dyn Deref<Target = pd_sys::t_float>> {
            Box::new(Box::new(initial_value))
        }

        fn new_float_inlet(&mut self, func: Box<Fn(&mut T, pd_sys::t_float)>) {
            self.float_inlets.push(func);
        }

        fn new_outlet(&mut self, t: OutletType) -> Box<dyn OutletSend> {
            Box::new(())
        }
    }

    #[test]
    fn can_build() {
        let mut a = A;
        let mut builder = TestBuilder::new();

        builder.new_float_inlet(Box::new(|a: &mut A, f: pd_sys::t_float| {
            a.print_float(f);
        }));
        builder.call_floats(&mut a, 4f32);
    }
    */
}
