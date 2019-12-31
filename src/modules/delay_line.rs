use super::{Node, ReadableNode};
use core::mem::MaybeUninit;
use sample::Sample;

pub struct DelayLine<T, const N: usize> {
    buffer: [T; N],
    index: usize,
}

impl<T: Sample, const N: usize> DelayLine<T, N> {
    pub fn new() -> Self {
        let mut delay = unsafe {
            Self {
                buffer: MaybeUninit::uninit().assume_init(),
                index: 0,
            }
        };

        delay.buffer.iter_mut().for_each(|x| *x = T::equilibrium());

        delay
    }
}

impl<T: Sample, const N: usize> Node<T, T> for DelayLine<T, N> {
    #[inline]
    fn process(&mut self, input: T) -> T {
        self.buffer[self.index] = input;

        if self.index == self.buffer.len() - 1 {
            self.index = 0
        } else {
            self.index += 1;
        }

        self.read()
    }
}

impl<T: Sample, const N: usize> ReadableNode<T> for DelayLine<T, N> {
    #[inline]
    fn read(&self) -> T {
        self.buffer[self.index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_instansiate() {
        let delay: DelayLine<f32, 10> = DelayLine::new();

        for x in delay.buffer.iter() {
            assert_eq!(x, &f32::default());
        }
    }
}
