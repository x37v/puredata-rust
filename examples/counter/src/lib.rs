use puredata_external::builder::ControlExternalBuilder;
use puredata_external::external::ControlExternal;
use puredata_external::pd;
use puredata_external_macros::external;
use std::ffi::CString;

//based on https://github.com/pure-data/externals-howto#a-simple-external-counter
external! {
    //the struct doesn't need to keep track of anything
    pub struct Counter {
    }

    impl ControlExternal for Counter {
        fn new(builder: &mut dyn ControlExternalBuilder<Self>) -> Self {
            if let Some(name) = builder.instance_name() {
                println!("name: {}", name);
            }
            let args = builder.creation_args();
            println!("args len {}", args.len());
            for a in args.iter() {
                println!("got arg");
                if let Some(f) = a.get_float() {
                    println!("got arg {}", f);
                }
            }
            Self { }
        }
    }

    impl Counter {
        #[bang] //indicates that a bang in Pd should call this
        pub fn bang(&mut self) {
            pd::post(CString::new("Hello world !!").unwrap());
        }
    }
}
