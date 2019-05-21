use crate::builder::{ExternalBuilder, SignalExternalBuilder};

pub trait External {
    fn new(builder: &mut dyn ExternalBuilder<Self>) -> Self;
}

pub trait SignalExternal {
    fn new(builder: &mut dyn SignalExternalBuilder<Self>) -> Self;
    //TODO make this const once it becomes stable
    fn has_input() -> bool;
    fn process(
        &mut self,
        inputs: &[&[puredata_sys::t_float]],
        outputs: &[&mut [puredata_sys::t_float]],
    );
}
