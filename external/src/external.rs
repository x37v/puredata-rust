use crate::builder::ExternalBuilder;

pub trait External {
    fn new(builder: &mut dyn ExternalBuilder<Self>) -> Self;
}
