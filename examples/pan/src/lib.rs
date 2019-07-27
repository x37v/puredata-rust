use itertools::izip;
use puredata_external::builder::SignalProcessorExternalBuilder;
use puredata_external::external::SignalProcessorExternal;
use puredata_external_macros::external;
use std::ops::Deref;
use std::rc::Rc;

external! {
    //based on pan~ from: https://github.com/pure-data/externals-howto#a-signal-external-pan
    pub struct Pan {
        //a passive float input, the pan position
        pan: Rc<dyn Deref<Target = puredata_sys::t_float>>,
    }

    impl SignalProcessorExternal for Pan {
        //build the object
        fn new(builder: &mut dyn SignalProcessorExternalBuilder<Self>) -> Self {
            //2 signal inlets (1 default)
            //1 float inlet (pan position)
            //1 signal outlet
            builder.new_signal_inlet();
            let pan = builder.new_passive_float_inlet(0f32);
            builder.new_signal_outlet();
            Self {
                pan
            }
        }

        //compute the audio
        fn process(
            &mut self,
            _frames: usize,
            inputs: &[&mut [puredata_sys::t_float]],
            outputs: &mut [&mut [puredata_sys::t_float]],
            ) {
            //read the value of our pan setting
            let pan = num::clamp(**self.pan, 0f32.into(), 1f32.into());

            //compute!
            let lpan = 1f32 - pan;
            let rpan = pan;
            for (o, l, r) in izip!(outputs[0].iter_mut(), inputs[0].iter(), inputs[1].iter()) {
                *o = *l * lpan + *r * rpan;
            }
        }
    }
}
