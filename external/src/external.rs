use crate::builder::ExternalBuilder;
trait External {
    fn new(builder: &mut dyn ExternalBuilder<Self>) -> Self;
}
