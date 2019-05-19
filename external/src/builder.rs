use crate::outlet::{OutletSend, OutletType};
use std::ops::DerefMut;
use std::rc::Rc;

pub trait ExternalBuilder<T> {
    fn new_passive_float_inlet(&mut self) -> Rc<dyn DerefMut<Target = puredata_sys::t_float>>;
    fn new_float_inlet(&mut self, func: Box<Fn(&mut T, puredata_sys::t_float)>);
    fn new_outlet(&mut self, t: OutletType) -> Rc<dyn OutletSend>;
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
        fn new_passive_float_inlet(&mut self) -> Rc<dyn DerefMut<Target = puredata_sys::t_float>> {
            Rc::new(Box::new(0 as puredata_sys::t_float))
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
