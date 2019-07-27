use puredata_external::builder::ControlExternalBuilder;
use puredata_external::external::ControlExternal;
use puredata_external::pd;
use puredata_external_macros::external;
use std::ffi::CString;

//based on https://github.com/pure-data/externals-howto#my-first-external-helloworld
external! {
    //the struct doesn't need to keep track of anything
    pub struct HelloWorld;

    impl ControlExternal for HelloWorld {
        fn new(_builder: &mut dyn ControlExternalBuilder<Self>) -> Self {
            Self { }
        }
    }

    impl HelloWorld {
        #[bang] //indicates that a bang in Pd should call this
        pub fn bang(&mut self) {
            pd::post(CString::new("Hello world !!").unwrap());
        }
    }
}
