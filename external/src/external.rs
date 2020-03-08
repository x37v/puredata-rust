use crate::builder::*;

pub trait ControlExternal: Sized {
    fn new(builder: &mut dyn ControlExternalBuilder<Self>) -> Result<Self, String>;
}

pub trait SignalGenerator: Send {
    fn generate(&mut self, frames: usize, outputs: &mut [&mut [pd_sys::t_float]]);
    //called when dsp is set up, can be used to setup resources
    fn setup_generate(&mut self, _frames: usize, _outputs: usize) {}
}

pub trait SignalProcessor: Send {
    fn process(
        &mut self,
        frames: usize,
        inputs: &[&mut [pd_sys::t_float]],
        outputs: &mut [&mut [pd_sys::t_float]],
    );
    //called when dsp is set up, can be used to setup resources
    fn setup_process(&mut self, _frames: usize, _inputs: usize, _outputs: usize) {}
}

pub trait SignalGeneratorExternal: Sized {
    fn new(
        builder: &mut dyn SignalGeneratorExternalBuilder<Self>,
    ) -> Result<(Self, Box<dyn SignalGenerator>), String>;
}

//has 1 default signal inlet
pub trait SignalProcessorExternal: Sized {
    fn new(
        builder: &mut dyn SignalProcessorExternalBuilder<Self>,
    ) -> Result<(Self, Box<dyn SignalProcessor>), String>;
}
