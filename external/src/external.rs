use crate::builder::{ExternalBuilder, SignalExternalBuilder};

pub trait External {
    fn new(builder: &mut dyn ExternalBuilder<Self>) -> Self;
}

pub trait SignalExternal {
    fn new(builder: &mut dyn SignalExternalBuilder<Self>) -> Self;
    fn process(
        &mut self,
        inputs: &[&[puredata_sys::t_float]],
        outputs: &[&mut [puredata_sys::t_float]],
    );
}
