use std::collections::VecDeque;
use crate::processor::Processor;
use crate::cycle::Cycle;

struct WaveSet{
    peak_amp: f32,
    length: usize
}

pub struct Hold{
    value: f32,
    gate: f32,
    thresh: f32,
    boost: f32,
    queue : VecDeque<WaveSet>,
    count: usize,
    cycle: Cycle
}

impl Processor for Hold {
    fn process(&mut self, buffer: &mut [f32]) {
        self.read(buffer);
        self.write(buffer);
    }
}

impl Hold{
    pub fn new()->Self{
        Self{
            value: 0.0,
            gate: 0.0,
            thresh: 0.0,
            boost: 0.0,
            queue: VecDeque::new(),
            count : 0,
            cycle: Cycle::new(0, 1.0)
        }
    }

    fn read(&mut self, buffer:&[f32]){
        for sample in buffer.iter(){
            if *sample != 0.0 {
                let amp = if *sample < 0.0 { 0.0 } else {*sample};
                self.queue.push_back(WaveSet{ peak_amp: amp, length: self.count});
                self.count = 0;
            }
            self.count += 1;
        }
    }

    fn write(&mut self, buffer: &mut[f32]){
        for sample in buffer.iter_mut(){
            if self.cycle.is_ended(){
                match self.queue.pop_front(){
                    Some(waveset) => {
                        self.cycle = Cycle::new(waveset.length, 1.0);
                        self.value = self.cook_amplitude(waveset.peak_amp);
                    },
                    None => self.value = 0.0
                }
            }
            self.cycle.advance();
            *sample = self.value;
        }
    }

    fn cook_amplitude(&mut self, value: f32) ->f32{
         if value < self.gate {
              return 0.0;
         }

        return if value > 0.0 {
            if value < self.thresh {
                value
            } else {
                let to_ceil = 1.0 - value;
                to_ceil * self.boost + value
            }
        } else {
            0.0
        }
    }

    pub fn set_gate(&mut self, gate: f32){
        self.gate = gate;
    }

    pub fn set_thresh(&mut self, thresh: f32){
        self.thresh = thresh;
    }

    pub fn set_boost(&mut self, boost: f32){
        self.boost = boost;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new(){
        let hold = Hold::new();
        assert_eq!(hold.value, 0.0);
    }

    #[test]
    fn process_silence(){
        let mut hold = Hold::new();
        let mut samples: Vec<f32> =vec![0.0, 0.0, 0.0];
        hold.process(&mut samples[..]);
        assert_eq!(samples, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn process_hold(){
        let mut hold = Hold::new();
        let mut samples: Vec<f32> =vec![0.0, 0.5, 0.0, 0.2];
        hold.process(&mut samples[..]);
        assert_eq!(samples, vec![0.5, 0.2, 0.2, 0.0]);
    }

    #[test]
    fn process_hold_update(){
        let mut hold = Hold::new();
        let mut samples: Vec<f32> =vec![0.0, 0.5, 0.0, 0.0, 0.2, 0.3];
        hold.process(&mut samples[..]);
        assert_eq!(samples, vec![0.5, 0.2, 0.2, 0.2, 0.3, 0.0]);
    }

    #[test]
    fn process_hold_reset(){
        let mut hold = Hold::new();
        let mut samples: Vec<f32> =vec![0.0, 0.5, 0.0, -1.0, 0.0, 0.2];
        hold.process(&mut samples[..]);
        assert_eq!(samples, vec![0.5, 0.0, 0.0, 0.2, 0.2, 0.0]);
    }

    #[test]
    fn process_boost(){
        let mut hold = Hold::new();
        hold.set_boost(0.5);
        let mut samples: Vec<f32> =vec![0.0, 0.5];
        hold.process(&mut samples[..]);
        assert_eq!(samples, vec![0.75, 0.0]);
    }

    #[test]
    fn process_boost_threshold(){
        let mut hold = Hold::new();
        hold.set_boost(0.5);
        hold.set_thresh(0.51);

        let mut samples: Vec<f32> =vec![0.0, 0.5];
        hold.process(&mut samples[..]);
        assert_eq!(samples, vec![0.5, 0.0]);
    }
}