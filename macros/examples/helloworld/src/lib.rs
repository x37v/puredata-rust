use pd_ext::builder::ControlExternalBuilder;
use pd_ext::external::ControlExternal;
use pd_ext::pd;
use pd_ext_macros::external;
use std::ffi::CString;

//based on https://github.com/pure-data/externals-howto#my-first-external-helloworld
external! {
    //the struct doesn't need to keep track of anything
    pub struct HelloWorld;

    impl ControlExternal for HelloWorld {
        fn new(_builder: &mut dyn ControlExternalBuilder<Self>) -> Result<Self, String> {
            Ok(Self { })
        }
    }

    impl HelloWorld {
        #[bang] //indicates that a bang in Pd should call this
        pub fn bang(&mut self) {
            pd::post(CString::new("Hello world !!").unwrap());
        }
    }
}
