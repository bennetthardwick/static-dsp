use super::{AllPass, Comb, Node};
use sample::Sample;

const FIXED_GAIN: f64 = 0.015;

const SCALE_WET: f64 = 3.0;
const SCALE_DAMPENING: f64 = 0.4;

const SCALE_ROOM: f64 = 0.28;
const OFFSET_ROOM: f64 = 0.7;

const STEREO_SPREAD: usize = 23;

const COMB_TUNING_L1: usize = 1116;
const COMB_TUNING_R1: usize = 1116 + STEREO_SPREAD;
const COMB_TUNING_L2: usize = 1188;
const COMB_TUNING_R2: usize = 1188 + STEREO_SPREAD;
const COMB_TUNING_L3: usize = 1277;
const COMB_TUNING_R3: usize = 1277 + STEREO_SPREAD;
const COMB_TUNING_L4: usize = 1356;
const COMB_TUNING_R4: usize = 1356 + STEREO_SPREAD;
const COMB_TUNING_L5: usize = 1422;
const COMB_TUNING_R5: usize = 1422 + STEREO_SPREAD;
const COMB_TUNING_L6: usize = 1491;
const COMB_TUNING_R6: usize = 1491 + STEREO_SPREAD;
const COMB_TUNING_L7: usize = 1557;
const COMB_TUNING_R7: usize = 1557 + STEREO_SPREAD;
const COMB_TUNING_L8: usize = 1617;
const COMB_TUNING_R8: usize = 1617 + STEREO_SPREAD;

const ALLPASS_TUNING_L1: usize = 556;
const ALLPASS_TUNING_R1: usize = 556 + STEREO_SPREAD;
const ALLPASS_TUNING_L2: usize = 441;
const ALLPASS_TUNING_R2: usize = 441 + STEREO_SPREAD;
const ALLPASS_TUNING_L3: usize = 341;
const ALLPASS_TUNING_R3: usize = 341 + STEREO_SPREAD;
const ALLPASS_TUNING_L4: usize = 225;
const ALLPASS_TUNING_R4: usize = 225 + STEREO_SPREAD;

pub struct Freeverb<T: Sample> {
    comb_1: (Comb<T, COMB_TUNING_L1>, Comb<T, COMB_TUNING_R1>),
    comb_2: (Comb<T, COMB_TUNING_L2>, Comb<T, COMB_TUNING_R2>),
    comb_3: (Comb<T, COMB_TUNING_L3>, Comb<T, COMB_TUNING_R3>),
    comb_4: (Comb<T, COMB_TUNING_L4>, Comb<T, COMB_TUNING_R4>),
    comb_5: (Comb<T, COMB_TUNING_L5>, Comb<T, COMB_TUNING_R5>),
    comb_6: (Comb<T, COMB_TUNING_L6>, Comb<T, COMB_TUNING_R6>),
    comb_7: (Comb<T, COMB_TUNING_L7>, Comb<T, COMB_TUNING_R7>),
    comb_8: (Comb<T, COMB_TUNING_L8>, Comb<T, COMB_TUNING_R8>),

    allpass_1: (AllPass<T, ALLPASS_TUNING_L1>, AllPass<T, ALLPASS_TUNING_R1>),
    allpass_2: (AllPass<T, ALLPASS_TUNING_L2>, AllPass<T, ALLPASS_TUNING_R2>),
    allpass_3: (AllPass<T, ALLPASS_TUNING_L3>, AllPass<T, ALLPASS_TUNING_R3>),
    allpass_4: (AllPass<T, ALLPASS_TUNING_L4>, AllPass<T, ALLPASS_TUNING_R4>),

    wet_gains: (f64, f64),
    wet: f64,
    width: f64,
    dry: f64,
    input_gain: f64,
    dampening: f64,
    room_size: f64,
    frozen: bool,
}

impl<T: Sample> Freeverb<T> {
    pub fn new() -> Self {
        let mut freeverb = Self {
            comb_1: (Comb::new(), Comb::new()),
            comb_2: (Comb::new(), Comb::new()),
            comb_3: (Comb::new(), Comb::new()),
            comb_4: (Comb::new(), Comb::new()),
            comb_5: (Comb::new(), Comb::new()),
            comb_6: (Comb::new(), Comb::new()),
            comb_7: (Comb::new(), Comb::new()),
            comb_8: (Comb::new(), Comb::new()),

            allpass_1: (AllPass::new(), AllPass::new()),
            allpass_2: (AllPass::new(), AllPass::new()),
            allpass_3: (AllPass::new(), AllPass::new()),
            allpass_4: (AllPass::new(), AllPass::new()),

            wet_gains: (0.0, 0.0),
            wet: 0.0,
            width: 0.0,
            dry: 0.0,
            input_gain: 0.0,
            dampening: 0.0,
            room_size: 0.0,
            frozen: false,
        };

        freeverb.set_wet(1.0);
        freeverb.set_width(0.5);
        freeverb.set_dampening(0.5);
        freeverb.set_room_size(0.5);
        freeverb.set_frozen(false);

        freeverb
    }

    pub fn set_dampening(&mut self, value: f64) {
        self.dampening = value * SCALE_DAMPENING;
        self.update_combs();
    }

    pub fn set_freeze(&mut self, frozen: bool) {
        self.frozen = frozen;
        self.update_combs();
    }

    pub fn set_wet(&mut self, value: f64) {
        self.wet = value * SCALE_WET;
        self.update_wet_gains();
    }

    pub fn set_width(&mut self, value: f64) {
        self.width = value;
        self.update_wet_gains();
    }

    fn update_wet_gains(&mut self) {
        self.wet_gains = (
            self.wet * (self.width / 2.0 + 0.5),
            self.wet * ((1.0 - self.width) / 2.0),
        )
    }

    fn set_frozen(&mut self, frozen: bool) {
        self.frozen = frozen;
        self.input_gain = if frozen { 0.0 } else { 1.0 };
        self.update_combs();
    }

    pub fn set_room_size(&mut self, value: f64) {
        self.room_size = value * SCALE_ROOM + OFFSET_ROOM;
        self.update_combs();
    }

    fn update_combs(&mut self) {
        let (feedback, dampening) = if self.frozen {
            (1.0.to_sample::<T::Float>(), 0.0.to_sample::<T::Float>())
        } else {
            (
                self.room_size.to_sample::<T::Float>(),
                self.dampening.to_sample::<T::Float>(),
            )
        };

        self.comb_1.0.set_feedback(feedback);
        self.comb_1.0.set_dampening(dampening);
        self.comb_1.1.set_feedback(feedback);
        self.comb_1.1.set_dampening(dampening);

        self.comb_2.0.set_feedback(feedback);
        self.comb_2.0.set_dampening(dampening);
        self.comb_2.1.set_feedback(feedback);
        self.comb_2.1.set_dampening(dampening);

        self.comb_3.0.set_feedback(feedback);
        self.comb_3.0.set_dampening(dampening);
        self.comb_3.1.set_feedback(feedback);
        self.comb_3.1.set_dampening(dampening);

        self.comb_4.0.set_feedback(feedback);
        self.comb_4.0.set_dampening(dampening);
        self.comb_4.1.set_feedback(feedback);
        self.comb_4.1.set_dampening(dampening);

        self.comb_5.0.set_feedback(feedback);
        self.comb_5.0.set_dampening(dampening);
        self.comb_5.1.set_feedback(feedback);
        self.comb_5.1.set_dampening(dampening);

        self.comb_6.0.set_feedback(feedback);
        self.comb_6.0.set_dampening(dampening);
        self.comb_6.1.set_feedback(feedback);
        self.comb_6.1.set_dampening(dampening);

        self.comb_7.0.set_feedback(feedback);
        self.comb_7.0.set_dampening(dampening);
        self.comb_7.1.set_feedback(feedback);
        self.comb_7.1.set_dampening(dampening);

        self.comb_8.0.set_feedback(feedback);
        self.comb_8.0.set_dampening(dampening);
        self.comb_8.1.set_feedback(feedback);
        self.comb_8.1.set_dampening(dampening);
    }

    pub fn set_dry(&mut self, value: f64) {
        self.dry = value;
    }
}

impl<T: Sample> Node<(T, T), (T, T)> for Freeverb<T> {
    #[inline]
    fn process(&mut self, input: (T, T)) -> (T, T) {
        let input_mixed = input
            .0
            .add_amp(input.1.to_signed_sample())
            .mul_amp(FIXED_GAIN.to_sample::<T::Float>())
            .mul_amp(self.input_gain.to_sample::<T::Float>());

        let mut output = (T::equilibrium(), T::equilibrium());

        output.0 = output
            .0
            .add_amp(self.comb_1.0.process(input_mixed).to_signed_sample());
        output.1 = output
            .1
            .add_amp(self.comb_1.1.process(input_mixed).to_signed_sample());

        output.0 = output
            .0
            .add_amp(self.comb_2.0.process(input_mixed).to_signed_sample());
        output.1 = output
            .1
            .add_amp(self.comb_2.1.process(input_mixed).to_signed_sample());

        output.0 = output
            .0
            .add_amp(self.comb_3.0.process(input_mixed).to_signed_sample());
        output.1 = output
            .1
            .add_amp(self.comb_3.1.process(input_mixed).to_signed_sample());

        output.0 = output
            .0
            .add_amp(self.comb_4.0.process(input_mixed).to_signed_sample());
        output.1 = output
            .1
            .add_amp(self.comb_4.1.process(input_mixed).to_signed_sample());

        output.0 = output
            .0
            .add_amp(self.comb_5.0.process(input_mixed).to_signed_sample());
        output.1 = output
            .1
            .add_amp(self.comb_5.1.process(input_mixed).to_signed_sample());

        output.0 = output
            .0
            .add_amp(self.comb_6.0.process(input_mixed).to_signed_sample());
        output.1 = output
            .1
            .add_amp(self.comb_6.1.process(input_mixed).to_signed_sample());

        output.0 = output
            .0
            .add_amp(self.comb_7.0.process(input_mixed).to_signed_sample());
        output.1 = output
            .1
            .add_amp(self.comb_7.1.process(input_mixed).to_signed_sample());

        output.0 = output
            .0
            .add_amp(self.comb_8.0.process(input_mixed).to_signed_sample());
        output.1 = output
            .1
            .add_amp(self.comb_8.1.process(input_mixed).to_signed_sample());

        output.0 = self.allpass_1.0.process(output.0);
        output.1 = self.allpass_1.1.process(output.1);

        output.0 = self.allpass_2.0.process(output.0);
        output.1 = self.allpass_2.1.process(output.1);

        output.0 = self.allpass_3.0.process(output.0);
        output.1 = self.allpass_3.1.process(output.1);

        output.0 = self.allpass_4.0.process(output.0);
        output.1 = self.allpass_4.1.process(output.1);

        (
            output
                .0
                .mul_amp(self.wet_gains.0.to_sample::<T::Float>())
                .add_amp(
                    output
                        .1
                        .mul_amp(self.wet_gains.1.to_sample::<T::Float>())
                        .to_signed_sample(),
                )
                .add_amp(
                    input
                        .0
                        .mul_amp(self.dry.to_sample::<T::Float>())
                        .to_signed_sample(),
                ),
            output
                .1
                .mul_amp(self.wet_gains.1.to_sample::<T::Float>())
                .add_amp(
                    output
                        .0
                        .mul_amp(self.wet_gains.0.to_sample::<T::Float>())
                        .to_signed_sample(),
                )
                .add_amp(
                    input
                        .1
                        .mul_amp(self.dry.to_sample::<T::Float>())
                        .to_signed_sample(),
                ),
        )
    }
}
