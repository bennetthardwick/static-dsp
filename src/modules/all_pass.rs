use super::{DelayLine, Node, ReadableNode};
use sample::Sample;

pub struct AllPass<T, const N: usize> {
    delay_line: DelayLine<T, N>,
}

impl<T: Sample, const N: usize> AllPass<T, N> {
    pub fn new() -> Self {
        Self {
            delay_line: DelayLine::new(),
        }
    }
}

impl<T: Sample, const N: usize> Node<T, T> for AllPass<T, N> {
    fn process(&mut self, input: T) -> T {
        let delayed = self.delay_line.read();

        let output = input
            .mul_amp((-1.0).to_sample::<T::Float>())
            .add_amp(delayed.to_signed_sample());

        let feedback: T::Float = 0.5.to_sample();

        self.delay_line
            .process(input.add_amp(delayed.mul_amp(feedback).to_signed_sample()));

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_ticking() {
        let mut allpass: AllPass<f32, 2> = AllPass::new();
        assert_eq!(allpass.process(1.0), -1.0);
        assert_eq!(allpass.process(0.0), 0.0);
        assert_eq!(allpass.process(0.0), 1.0);
        assert_eq!(allpass.process(0.0), 0.0);
        assert_eq!(allpass.process(0.0), 0.5);
        assert_eq!(allpass.process(0.0), 0.0);
        assert_eq!(allpass.process(0.0), 0.25);
    }
}
