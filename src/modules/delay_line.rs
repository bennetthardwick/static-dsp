use super::{Node, ReadableNode};

pub struct DelayLine<const N: usize> {
    buffer: [f32; N],
    index: usize,
}

impl<const N: usize> DelayLine<N> {
    pub fn new() -> Self {
        Self {
            buffer: [0.; N],
            index: 0,
        }
    }
}

impl<const N: usize> Node<f32, f32> for DelayLine<N> {
    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        self.buffer[self.index] = input;

        if (self.index + 1) < self.buffer.len() {
            self.index += 1;
        } else {
            self.index = 0
        }

        self.read()
    }
}

impl<const N: usize> ReadableNode<f32> for DelayLine<N> {
    #[inline]
    fn read(&self) -> f32 {
        self.buffer[self.index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_instansiate() {
        let delay: DelayLine<10> = DelayLine::new();

        for x in delay.buffer.iter() {
            assert_eq!(x, &f32::default());
        }
    }
}
