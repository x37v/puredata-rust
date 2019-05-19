use crate::outlet::{OutletSend, OutletType};
use std::ops::DerefMut;
use std::rc::Rc;

trait ExternalBuilder<T> {
    fn new_passive_float_inlet(&mut self) -> Rc<dyn DerefMut<Target = puredata_sys::t_float>>;
    fn new_float_inlet(&mut self, func: FnMut(&mut T, puredata_sys::t_float));
    fn new_outlet(&mut self, t: OutletType) -> Rc<dyn OutletSend>;
}
