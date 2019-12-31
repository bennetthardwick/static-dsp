use super::{DelayLine, Node, ReadableNode};
use sample::Sample;

pub struct Comb<T: Sample, const N: usize> {
    delay_line: DelayLine<T, N>,
    feedback: T::Float,
    filter_state: T,
    dampening: T::Float,
    dampening_inverse: T::Float,
}

impl<T: Sample, const N: usize> Comb<T, N> {
    pub fn new() -> Self {
        Self {
            delay_line: DelayLine::new(),
            feedback: 0.5.to_sample(),
            filter_state: T::equilibrium(),
            dampening: 0.5.to_sample(),
            dampening_inverse: 0.5.to_sample(),
        }
    }

    pub fn set_dampening(&mut self, value: T::Float) {
        self.dampening = value;
        self.dampening_inverse = 1.0
            .to_sample::<T::Float>()
            .add_amp(value.mul_amp(-1.0.to_sample::<T::Float>()));
    }

    pub fn set_feedback(&mut self, value: T::Float) {
        self.feedback = value;
    }
}

impl<T: Sample, const N: usize> Node<T, T> for Comb<T, N> {
    fn process(&mut self, input: T) -> T {
        let output = self.delay_line.read();

        self.filter_state = output.mul_amp(self.dampening_inverse).add_amp(
            (self.filter_state.mul_amp(self.dampening).to_signed_sample()).to_signed_sample(),
        );

        self.delay_line
            .process(input.add_amp(self.filter_state.mul_amp(self.feedback).to_signed_sample()));

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_ticking() {
        let mut comb: Comb<f32, 2> = Comb::new();
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
