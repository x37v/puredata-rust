pub enum Atom {
    Float(puredata_sys::t_float),
    Symbol(puredata_sys::t_symbol),
    //XXX todo, Pointer?
}
