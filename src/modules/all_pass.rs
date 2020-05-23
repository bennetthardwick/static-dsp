use super::{DelayLine, Node, ReadableNode};

pub struct AllPass<const N: usize> {
    delay_line: DelayLine<N>,
}

impl<const N: usize> AllPass<N> {
    pub fn new() -> Self {
        Self {
            delay_line: DelayLine::new(),
        }
    }
}

impl<const N: usize> Node<f32, f32> for AllPass<N> {
    fn process(&mut self, input: f32) -> f32 {
        let delayed = self.delay_line.read();

        let output = -input + delayed;

        let feedback: f32 = 0.5;

        self.delay_line.process(input + delayed * feedback);

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_ticking() {
        let mut allpass: AllPass<2> = AllPass::new();
        assert_eq!(allpass.process(1.0), -1.0);
        assert_eq!(allpass.process(0.0), 0.0);
        assert_eq!(allpass.process(0.0), 1.0);
        assert_eq!(allpass.process(0.0), 0.0);
        assert_eq!(allpass.process(0.0), 0.5);
        assert_eq!(allpass.process(0.0), 0.0);
        assert_eq!(allpass.process(0.0), 0.25);
    }
}
