use super::{DelayLine, Node, ReadableNode};

pub struct LowpassFeedbackComb<const N: usize> {
    delay_line: DelayLine<N>,
    feedback: f32,
    filter_state: f32,
    dampening: f32,
    dampening_inverse: f32,
}

impl<const N: usize> LowpassFeedbackComb<N> {
    pub fn new() -> Self {
        Self {
            delay_line: DelayLine::new(),
            feedback: 0.5,
            filter_state: 0.,
            dampening: 0.5,
            dampening_inverse: 0.5,
        }
    }

    pub fn set_dampening(&mut self, value: f32) {
        self.dampening = value;
        self.dampening_inverse = 1. - value;
    }

    pub fn set_feedback(&mut self, value: f32) {
        self.feedback = value;
    }
}

impl<const N: usize> Node<f32, f32> for LowpassFeedbackComb<N> {
    fn process(&mut self, input: f32) -> f32 {
        let output = self.delay_line.read();

        self.filter_state = output * self.dampening_inverse + self.filter_state * self.dampening;

        self.delay_line
            .process(input + self.filter_state * self.feedback);

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_ticking() {
        let mut comb: LowpassFeedbackComb<2> = LowpassFeedbackComb::new();
        assert_eq!(comb.process(1.0), 0.0);
        assert_eq!(comb.process(0.0), 0.0);
        assert_eq!(comb.process(0.0), 1.0);
        assert_eq!(comb.process(0.0), 0.0);
        assert_eq!(comb.process(0.0), 0.25);
        assert_eq!(comb.process(0.0), 0.125);
        assert_eq!(comb.process(0.0), 0.125);
        assert_eq!(comb.process(0.0), 0.09375);
    }
}
