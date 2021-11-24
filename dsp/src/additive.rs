use std::f32::consts::PI;
use crate::processor::Processor;

pub struct Additive{
    harmonics : [f32; 5]
}

impl Processor for Additive  {
    fn process(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            if *sample < 0.0 || *sample > 1.0 {
                *sample = 0.0;
                continue;
            } else {
                let mut accumulate = 0.0_f32;
                let value = *sample;
                for h in 0..5 {
                    accumulate += (value * 2.0 * PI * (h + 1) as f32).sin() * self.harmonics[h];
                }
                *sample = accumulate;
            }
        }
    }
}

impl Additive {
    pub fn new() -> Self {
        Self {
            harmonics : [1.0, 0.0, 0.0, 0.0, 0.0]
        }
    }

    pub fn set_harmonic(&mut self, harmonic: usize, value: f32)
    {
        if harmonic > 4 {
            return;
        }
        self.harmonics[harmonic] = value;
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn sine(){
        let mut additive = Additive::new();
        let mut buffer = vec![0.32];
        additive.process(&mut buffer[..]);
        assert_eq!(buffer[0], (0.32 * 2.0 * PI).sin());
    }

    #[test]
    fn set(){
        let mut additive = Additive::new();
        let mut buffer = vec![0.32];
        additive.set_harmonic(0, 0.5);
        additive.process(&mut buffer[..]);
        assert_eq!(buffer[0], (0.32 * 2.0 * PI).sin() * 0.5);
    }

    #[test]
    fn set_harmonic(){
        let mut additive = Additive::new();
        let mut buffer = vec![0.32];
        additive.set_harmonic(1, 0.5);
        additive.process(&mut buffer[..]);
        assert_eq!(buffer[0], (0.32 * 2.0 * PI).sin() + (0.32 * 4.0 * PI).sin() * 0.5);
    }

    #[test]
    fn out_of_range(){
        let mut additive = Additive::new();
        let mut buffer = vec![-0.00001];
        additive.process(&mut buffer[..]);
        assert_eq!(buffer[0], 0.0);

        let mut buffer = vec![1.0001];
        additive.process(&mut buffer[..]);
        assert_eq!(buffer[0], 0.0);
    }
}