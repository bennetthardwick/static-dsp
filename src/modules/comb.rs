use super::{DelayLine, IntoSample, Node, ReadableNode, Sample};

pub struct Comb<T: Sample, const N: usize> {
    delay_line: DelayLine<T, N>,
    feedback: T,
    filter_state: T,
    dampening: T,
    dampening_inverse: T,
}

impl<T: Sample, const N: usize> Comb<T, N> {
    pub fn new() -> Self {
        Self {
            delay_line: DelayLine::new(),
            feedback: 0.5.into_sample(),
            filter_state: T::equilibrium(),
            dampening: 0.5.into_sample(),
            dampening_inverse: 0.5.into_sample(),
        }
    }

    pub fn set_dampening(&mut self, value: T) {
        self.dampening = value;
        self.dampening_inverse = T::peak() - value;
    }

    pub fn set_feedback(&mut self, value: T) {
        self.feedback = value;
    }
}

impl<T: Sample, const N: usize> Node<T, T> for Comb<T, N> {
    fn process(&mut self, input: T) -> T {
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
