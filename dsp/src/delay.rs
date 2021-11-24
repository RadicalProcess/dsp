use std::collections::vec_deque::VecDeque;
use crate::processor::Processor;

pub struct Delay{
    ring_buffer: VecDeque<f32>
}

impl Processor for Delay{
    fn process(&mut self, buffer: &mut [f32]) {
        self.push(buffer);
        self.pop(buffer);
    }
}

impl Delay {
    pub fn new(delay_time_in_sample: usize) -> Self {
        let mut queue: VecDeque<f32> = VecDeque::with_capacity(delay_time_in_sample * 2);
        let mut padding: VecDeque<f32> = VecDeque::from(vec![0.0; delay_time_in_sample]);
        queue.append(&mut padding);
        Self {
            ring_buffer: queue
        }
    }

    fn push(&mut self, buffer: &[f32]){
        for sample in buffer{
            self.ring_buffer.push_back(*sample);
        }
    }

    fn pop(&mut self, buffer: &mut [f32]){
        let length = buffer.len();
        for i in 0..length{
            buffer[i] = self.ring_buffer.pop_front().unwrap();
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn process() {
        let mut delay = Delay::new(4);
        let mut samples: Vec<f32> = vec![0.1, 0.2, 0.3, 0.4];

        delay.process(&mut samples[..]);
        assert_eq!(samples, vec![0.0, 0.0, 0.0, 0.0]);

        delay.process(&mut samples[..]);
        assert_eq!(samples, vec![0.1, 0.2, 0.3, 0.4]);
    }
}

