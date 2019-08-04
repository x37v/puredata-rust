use itertools::izip;
use pd_ext::builder::SignalProcessorExternalBuilder;
use pd_ext::external::SignalProcessorExternal;
use pd_ext_macros::external;
use std::ops::Deref;
use std::rc::Rc;

external! {
    //based on pan~ from: https://github.com/pure-data/externals-howto#a-signal-external-pan
    pub struct XFade {
        //a passive float input, the xfade position
        pos: Rc<dyn Deref<Target = pd_sys::t_float>>,
    }

    impl SignalProcessorExternal for XFade {
        //build the object
        fn new(builder: &mut dyn SignalProcessorExternalBuilder<Self>) -> Self {
            //2 signal inlets (1 default)
            //1 float inlet (position)
            //1 signal outlet
            builder.new_signal_inlet();
            let pos = builder.new_passive_float_inlet(0f32);
            builder.new_signal_outlet();
            Self {
                pos
            }
        }

        //compute the audio
        fn process(&mut self, _frames: usize, inputs: &[&mut [pd_sys::t_float]], outputs: &mut [&mut [pd_sys::t_float]],) {
            //read the value of our position setting
            let pos = num::clamp(**self.pos, 0f32.into(), 1f32.into());

            //compute!
            let lpan = 1f32 - pos;
            let rpan = pos;
            for (o, l, r) in izip!(outputs[0].iter_mut(), inputs[0].iter(), inputs[1].iter()) {
                *o = *l * lpan + *r * rpan;
            }
        }
    }
}
