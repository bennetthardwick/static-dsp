use super::{DelayLine, IntoSample, Node, ReadableNode, Sample};

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

        let output = -input + delayed;

        let feedback: T = 0.5.into_sample();

        self.delay_line.process(input + delayed * feedback);

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
