use super::{AllPass, LowpassFeedbackComb, Node};

const FIXED_GAIN: f32 = 0.015;

const SCALE_WET: f32 = 3.0;
const SCALE_DAMPENING: f32 = 0.4;

const SCALE_ROOM: f32 = 0.28;
const OFFSET_ROOM: f32 = 0.7;

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

pub struct Freeverb<
    const COMB_TUNING_L1: usize,
    const COMB_TUNING_R1: usize,
    const COMB_TUNING_L2: usize,
    const COMB_TUNING_R2: usize,
    const COMB_TUNING_L3: usize,
    const COMB_TUNING_R3: usize,
    const COMB_TUNING_L4: usize,
    const COMB_TUNING_R4: usize,
    const COMB_TUNING_L5: usize,
    const COMB_TUNING_R5: usize,
    const COMB_TUNING_L6: usize,
    const COMB_TUNING_R6: usize,
    const COMB_TUNING_L7: usize,
    const COMB_TUNING_R7: usize,
    const COMB_TUNING_L8: usize,
    const COMB_TUNING_R8: usize,
    const ALLPASS_TUNING_L1: usize,
    const ALLPASS_TUNING_R1: usize,
    const ALLPASS_TUNING_L2: usize,
    const ALLPASS_TUNING_R2: usize,
    const ALLPASS_TUNING_L3: usize,
    const ALLPASS_TUNING_R3: usize,
    const ALLPASS_TUNING_L4: usize,
    const ALLPASS_TUNING_R4: usize,
> {
    comb_1: (
        LowpassFeedbackComb<COMB_TUNING_L1>,
        LowpassFeedbackComb<COMB_TUNING_R1>,
    ),
    comb_2: (
        LowpassFeedbackComb<COMB_TUNING_L2>,
        LowpassFeedbackComb<COMB_TUNING_R2>,
    ),
    comb_3: (
        LowpassFeedbackComb<COMB_TUNING_L3>,
        LowpassFeedbackComb<COMB_TUNING_R3>,
    ),
    comb_4: (
        LowpassFeedbackComb<COMB_TUNING_L4>,
        LowpassFeedbackComb<COMB_TUNING_R4>,
    ),
    comb_5: (
        LowpassFeedbackComb<COMB_TUNING_L5>,
        LowpassFeedbackComb<COMB_TUNING_R5>,
    ),
    comb_6: (
        LowpassFeedbackComb<COMB_TUNING_L6>,
        LowpassFeedbackComb<COMB_TUNING_R6>,
    ),
    comb_7: (
        LowpassFeedbackComb<COMB_TUNING_L7>,
        LowpassFeedbackComb<COMB_TUNING_R7>,
    ),
    comb_8: (
        LowpassFeedbackComb<COMB_TUNING_L8>,
        LowpassFeedbackComb<COMB_TUNING_R8>,
    ),

    allpass_1: (AllPass<ALLPASS_TUNING_L1>, AllPass<ALLPASS_TUNING_R1>),
    allpass_2: (AllPass<ALLPASS_TUNING_L2>, AllPass<ALLPASS_TUNING_R2>),
    allpass_3: (AllPass<ALLPASS_TUNING_L3>, AllPass<ALLPASS_TUNING_R3>),
    allpass_4: (AllPass<ALLPASS_TUNING_L4>, AllPass<ALLPASS_TUNING_R4>),

    wet_gains: (f32, f32),
    wet: f32,
    width: f32,
    dry: f32,
    input_gain: f32,
    dampening: f32,
    room_size: f32,
    frozen: bool,
}

impl<
        const COMB_TUNING_L1: usize,
        const COMB_TUNING_R1: usize,
        const COMB_TUNING_L2: usize,
        const COMB_TUNING_R2: usize,
        const COMB_TUNING_L3: usize,
        const COMB_TUNING_R3: usize,
        const COMB_TUNING_L4: usize,
        const COMB_TUNING_R4: usize,
        const COMB_TUNING_L5: usize,
        const COMB_TUNING_R5: usize,
        const COMB_TUNING_L6: usize,
        const COMB_TUNING_R6: usize,
        const COMB_TUNING_L7: usize,
        const COMB_TUNING_R7: usize,
        const COMB_TUNING_L8: usize,
        const COMB_TUNING_R8: usize,
        const ALLPASS_TUNING_L1: usize,
        const ALLPASS_TUNING_R1: usize,
        const ALLPASS_TUNING_L2: usize,
        const ALLPASS_TUNING_R2: usize,
        const ALLPASS_TUNING_L3: usize,
        const ALLPASS_TUNING_R3: usize,
        const ALLPASS_TUNING_L4: usize,
        const ALLPASS_TUNING_R4: usize,
    >
    Freeverb<
        COMB_TUNING_L1,
        COMB_TUNING_R1,
        COMB_TUNING_L2,
        COMB_TUNING_R2,
        COMB_TUNING_L3,
        COMB_TUNING_R3,
        COMB_TUNING_L4,
        COMB_TUNING_R4,
        COMB_TUNING_L5,
        COMB_TUNING_R5,
        COMB_TUNING_L6,
        COMB_TUNING_R6,
        COMB_TUNING_L7,
        COMB_TUNING_R7,
        COMB_TUNING_L8,
        COMB_TUNING_R8,
        ALLPASS_TUNING_L1,
        ALLPASS_TUNING_R1,
        ALLPASS_TUNING_L2,
        ALLPASS_TUNING_R2,
        ALLPASS_TUNING_L3,
        ALLPASS_TUNING_R3,
        ALLPASS_TUNING_L4,
        ALLPASS_TUNING_R4,
    >
{
    pub fn new() -> Self {
        let mut freeverb = Self {
            comb_1: (LowpassFeedbackComb::new(), LowpassFeedbackComb::new()),
            comb_2: (LowpassFeedbackComb::new(), LowpassFeedbackComb::new()),
            comb_3: (LowpassFeedbackComb::new(), LowpassFeedbackComb::new()),
            comb_4: (LowpassFeedbackComb::new(), LowpassFeedbackComb::new()),
            comb_5: (LowpassFeedbackComb::new(), LowpassFeedbackComb::new()),
            comb_6: (LowpassFeedbackComb::new(), LowpassFeedbackComb::new()),
            comb_7: (LowpassFeedbackComb::new(), LowpassFeedbackComb::new()),
            comb_8: (LowpassFeedbackComb::new(), LowpassFeedbackComb::new()),

            allpass_1: (AllPass::new(), AllPass::new()),
            allpass_2: (AllPass::new(), AllPass::new()),
            allpass_3: (AllPass::new(), AllPass::new()),
            allpass_4: (AllPass::new(), AllPass::new()),

            wet_gains: (0., 0.),
            wet: 0.,
            width: 0.,
            dry: 0.,
            input_gain: 0.,
            dampening: 0.,
            room_size: 0.,
            frozen: false,
        };

        freeverb.set_wet(1.0);
        freeverb.set_width(0.5);
        freeverb.set_dampening(0.5);
        freeverb.set_room_size(0.5);
        freeverb.set_frozen(false);

        freeverb
    }

    pub fn set_dampening(&mut self, value: f32) {
        self.dampening = value * SCALE_DAMPENING;
        self.update_combs();
    }

    pub fn set_freeze(&mut self, frozen: bool) {
        self.frozen = frozen;
        self.update_combs();
    }

    pub fn set_wet(&mut self, value: f32) {
        self.wet = value * SCALE_WET;
        self.update_wet_gains();
    }

    pub fn set_width(&mut self, value: f32) {
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
        self.input_gain = if frozen { 0. } else { 1. };
        self.update_combs();
    }

    pub fn set_room_size(&mut self, value: f32) {
        self.room_size = value * SCALE_ROOM + OFFSET_ROOM;
        self.update_combs();
    }

    fn update_combs(&mut self) {
        let (feedback, dampening) = if self.frozen {
            (1., 0.)
        } else {
            (self.room_size, self.dampening)
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

    pub fn set_dry(&mut self, value: f32) {
        self.dry = value;
    }
}

impl<
        const COMB_TUNING_L1: usize,
        const COMB_TUNING_R1: usize,
        const COMB_TUNING_L2: usize,
        const COMB_TUNING_R2: usize,
        const COMB_TUNING_L3: usize,
        const COMB_TUNING_R3: usize,
        const COMB_TUNING_L4: usize,
        const COMB_TUNING_R4: usize,
        const COMB_TUNING_L5: usize,
        const COMB_TUNING_R5: usize,
        const COMB_TUNING_L6: usize,
        const COMB_TUNING_R6: usize,
        const COMB_TUNING_L7: usize,
        const COMB_TUNING_R7: usize,
        const COMB_TUNING_L8: usize,
        const COMB_TUNING_R8: usize,
        const ALLPASS_TUNING_L1: usize,
        const ALLPASS_TUNING_R1: usize,
        const ALLPASS_TUNING_L2: usize,
        const ALLPASS_TUNING_R2: usize,
        const ALLPASS_TUNING_L3: usize,
        const ALLPASS_TUNING_R3: usize,
        const ALLPASS_TUNING_L4: usize,
        const ALLPASS_TUNING_R4: usize,
    > Node<(f32, f32), (f32, f32)>
    for Freeverb<
        COMB_TUNING_L1,
        COMB_TUNING_R1,
        COMB_TUNING_L2,
        COMB_TUNING_R2,
        COMB_TUNING_L3,
        COMB_TUNING_R3,
        COMB_TUNING_L4,
        COMB_TUNING_R4,
        COMB_TUNING_L5,
        COMB_TUNING_R5,
        COMB_TUNING_L6,
        COMB_TUNING_R6,
        COMB_TUNING_L7,
        COMB_TUNING_R7,
        COMB_TUNING_L8,
        COMB_TUNING_R8,
        ALLPASS_TUNING_L1,
        ALLPASS_TUNING_R1,
        ALLPASS_TUNING_L2,
        ALLPASS_TUNING_R2,
        ALLPASS_TUNING_L3,
        ALLPASS_TUNING_R3,
        ALLPASS_TUNING_L4,
        ALLPASS_TUNING_R4,
    >
{
    #[inline]
    fn process(&mut self, input: (f32, f32)) -> (f32, f32) {
        let input_mixed = (input.0 + input.1) * FIXED_GAIN * self.input_gain;

        let mut output = (0., 0.);

        output.0 += self.comb_1.0.process(input_mixed);
        output.1 += self.comb_1.1.process(input_mixed);

        output.0 += self.comb_2.0.process(input_mixed);
        output.1 += self.comb_2.1.process(input_mixed);

        output.0 += self.comb_3.0.process(input_mixed);
        output.1 += self.comb_3.1.process(input_mixed);

        output.0 += self.comb_4.0.process(input_mixed);
        output.1 += self.comb_4.1.process(input_mixed);

        output.0 += self.comb_5.0.process(input_mixed);
        output.1 += self.comb_5.1.process(input_mixed);

        output.0 += self.comb_6.0.process(input_mixed);
        output.1 += self.comb_6.1.process(input_mixed);

        output.0 += self.comb_7.0.process(input_mixed);
        output.1 += self.comb_7.1.process(input_mixed);

        output.0 += self.comb_8.0.process(input_mixed);
        output.1 += self.comb_8.1.process(input_mixed);

        output.0 = self.allpass_1.0.process(output.0);
        output.1 = self.allpass_1.1.process(output.1);

        output.0 = self.allpass_2.0.process(output.0);
        output.1 = self.allpass_2.1.process(output.1);

        output.0 = self.allpass_3.0.process(output.0);
        output.1 = self.allpass_3.1.process(output.1);

        output.0 = self.allpass_4.0.process(output.0);
        output.1 = self.allpass_4.1.process(output.1);

        (
            output.0 * self.wet_gains.0 + output.1 * self.wet_gains.1 + input.0 * self.dry,
            output.1 * self.wet_gains.1 + output.0 * self.wet_gains.0 + input.1 * self.dry,
        )
    }
}

pub type Freeverb44100<T> = Freeverb<
    COMB_TUNING_L1,
    COMB_TUNING_R1,
    COMB_TUNING_L2,
    COMB_TUNING_R2,
    COMB_TUNING_L3,
    COMB_TUNING_R3,
    COMB_TUNING_L4,
    COMB_TUNING_R4,
    COMB_TUNING_L5,
    COMB_TUNING_R5,
    COMB_TUNING_L6,
    COMB_TUNING_R6,
    COMB_TUNING_L7,
    COMB_TUNING_R7,
    COMB_TUNING_L8,
    COMB_TUNING_R8,
    ALLPASS_TUNING_L1,
    ALLPASS_TUNING_R1,
    ALLPASS_TUNING_L2,
    ALLPASS_TUNING_R2,
    ALLPASS_TUNING_L3,
    ALLPASS_TUNING_R3,
    ALLPASS_TUNING_L4,
    ALLPASS_TUNING_R4,
>;
