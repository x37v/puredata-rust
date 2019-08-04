use crate::builder::*;

pub trait ControlExternal {
    fn new(builder: &mut dyn ControlExternalBuilder<Self>) -> Self;
}

pub trait SignalGeneratorExternal {
    fn new(builder: &mut dyn SignalGeneratorExternalBuilder<Self>) -> Self;
    fn generate(&mut self, frames: usize, outputs: &mut [&mut [pd_sys::t_float]]);
}

//has 1 default signal inlet
pub trait SignalProcessorExternal {
    fn new(builder: &mut dyn SignalProcessorExternalBuilder<Self>) -> Self;
    fn process(
        &mut self,
        frames: usize,
        inputs: &[&mut [pd_sys::t_float]],
        outputs: &mut [&mut [pd_sys::t_float]],
    );
}
