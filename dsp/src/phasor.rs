use std::collections::VecDeque;

use crate::processor::Processor;
use crate::cycle::Cycle;

pub struct Phasor{
    pub duty_cycle: f32,
    queue : VecDeque<usize>,
    count : usize,
    cycle : Cycle
}

impl Processor for Phasor {

    fn process(&mut self, buffer:&mut[f32]){
        self.read(buffer);
        self.write(buffer);
    }
}

impl Phasor {

    pub fn new() -> Self {
        Self {
            duty_cycle: 1.0,
            queue: VecDeque::new(),
            count: 0,
            cycle: Cycle::new(0, 1.0)
        }
    }

    pub fn set_duty_cycle(&mut self, duty_cycle: f32){
        self.duty_cycle = duty_cycle;
    }

    fn read(&mut self, buffer:&[f32]){
        for sample in buffer.iter(){
            if *sample != 0.0 {
                self.queue.push_back(self.count);
                self.count = 0;
            }
            self.count += 1;
        }
    }

    fn write(&mut self, buffer:&mut [f32]){
        for sample in buffer.iter_mut(){
            if self.cycle.is_ended(){
                match self.queue.pop_front(){
                    Some( size) => {
                        self.cycle = Cycle::new(size, self.duty_cycle);
                        *sample = self.cycle.phase();
                        self.cycle.advance();
                    },
                    None => *sample = 0.0
                }
            } else {
                *sample = self.cycle.phase();
                self.cycle.advance();
            }
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn new(){
        let phasor = Phasor::new();
        assert_eq!(phasor.duty_cycle, 1.0);
        assert_eq!(phasor.count, 0);
    }

    #[test]
    fn process(){
        let mut phasor = Phasor::new();
        let mut samples = vec![0.0, 0.0, 0.0, 0.0, 0.5];

        phasor.process(&mut samples[..]);

        assert_eq!(samples, vec![0.0, 0.25, 0.5, 0.75, 0.0]);
    }

    #[test]
    fn process_negative_reset(){
        let mut phasor = Phasor::new();
        let mut samples = vec![0.0, 0.0, 0.0, 0.0, -0.2, 0.0, 0.0, 0.0, 0.5];

        phasor.process(&mut samples[..]);

        assert_eq!(samples, vec![0.0, 0.25, 0.5, 0.75, 0.0, 0.25, 0.5, 0.75, 0.0]);
    }

}