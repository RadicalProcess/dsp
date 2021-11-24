use crate::counter::Counter;
use crate::processor::Processor;

pub struct ZeroX{
    peak_amp: f32,
    skip: usize,
    skip_count: usize,
    min_length: usize,
    length_count: usize,
    sample_counter: Counter
}

impl Processor for ZeroX {
    fn process(&mut self, buffer:&mut [f32] ){

        fn check_zerox(peak_amp:f32, value:f32)->bool{
            peak_amp < 0.0 && value > 0.0 || peak_amp >= 0.0 && value < 0.0
        }

        for sample in buffer.iter_mut() {
            if check_zerox(self.peak_amp, *sample) && self.min_length <= self.length_count{
                if self.skip_count == 0 {
                    let val = *sample;
                    *sample = self.peak_amp.abs();
                    self.peak_amp = val;
                    self.sample_counter.reset();
                    self.skip_count = self.skip;
                } else {
                    self.peak_amp =  *sample;
                    self.skip_count -= 1;
                    *sample = 0.0;
                }
                self.length_count = 0;
            }
            else {
                if self.peak_amp.abs() < (*sample).abs() {
                    self.peak_amp = *sample;
                }
                *sample = 0.0;
                self.length_count += 1;
            }

            if self.sample_counter.count() {
                *sample = -1.0;
                self.skip_count = 0;
            }
        }
    }
}

impl ZeroX
{
    pub fn new(timeout: usize) -> Self{
        if timeout == 0 {
            panic!("timeout has to be longer than 0");
        }
        ZeroX{
            peak_amp: -0.00001,
            skip: 0,
            skip_count: 0,
            min_length: 0,
            length_count: 0,
            sample_counter: Counter::new(timeout)
        }
    }

    pub fn set_skip(&mut self, skip: usize) {
        self.skip = skip;
        self.skip_count = self.skip;
    }

    pub fn set_min_length(&mut self, min_length: usize){
        self.min_length = min_length;
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    #[should_panic]
    fn construction_too_short_timeout(){
        ZeroX::new(0);
    }

    #[test]
    fn construction() {
        let zerox = ZeroX::new(2048);
        assert_eq!(zerox.peak_amp, -0.00001);
    }

    #[test]
    fn detect_silence(){
        let mut zerox = ZeroX::new(2048);
        let mut sample : Vec<f32> = vec![0.0; 6];
        zerox.process(&mut sample[..]);

        assert_eq!(sample, vec![0.0; 6]);
    }

    #[test]
    fn detect_positive_first_sample(){
        let mut zerox = ZeroX::new(2048);
        let mut sample : Vec<f32> = vec![0.1, 0.2, 0.3, 0.2, 0.4, -0.5];
        zerox.process(&mut sample[..]);

        assert_eq!(sample, vec![0.00001, 0.0, 0.0, 0.0, 0.0, 0.4]);
    }

    #[test]
    fn detect_upward_zerox(){
        let mut zerox = ZeroX::new(2048);

        let mut sample : Vec<f32> = vec![-0.2, -0.4, -0.9, 0.1, 0.5, 0.3 ];
        zerox.process(&mut sample[..]);

        assert_eq!(sample, vec![0.0, 0.0, 0.0, 0.9, 0.0, 0.0]);
    }

    #[test]
    fn detect_upward_downward_zerox(){
        let mut zerox = ZeroX::new(2048);

        let mut sample : Vec<f32> = vec![-0.2, -0.4, 0.1, 0.5, -0.2, -0.3 ];
        zerox.process(&mut sample[..]);

        assert_eq!(sample, vec![0.0, 0.0, 0.4, 0.0, 0.5, 0.0]);
    }

    #[test]
    fn detect_upward_downward_zerox_with_skip_count(){
        let mut zerox = ZeroX::new(2048);

        let mut sample : Vec<f32> = vec![-0.2, -0.4, 0.1, 0.5, -0.2, -0.3 ];
        zerox.set_skip(1);
        zerox.process(&mut sample[..]);

        assert_eq!(sample, vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.0]);
    }

    #[test]
    fn detect_in_multiple_blocks(){
        let mut zerox = ZeroX::new(2048);

        let mut block1 : Vec<f32> = vec![-0.2, -0.4, 0.1, 0.5 ];
        let mut block2 : Vec<f32> = vec![0.4, -0.2, -0.3, -0.1];

        zerox.process(&mut block1[..]);
        zerox.process(&mut block2[..]);

        assert_eq!(block1, vec![0.0, 0.0, 0.4, 0.0]);
        assert_eq!(block2, vec![0.0, 0.5, 0.0, 0.0]);
    }

    #[test]
    fn detect_timeout(){
        let mut zerox = ZeroX::new(4);

        let mut block : Vec<f32> = vec![-0.2, -0.4, -0.2, -0.5];

        zerox.process(&mut block[..]);

        assert_eq!(block, vec![0.0, 0.0, 0.0, -1.0]);
    }

    #[test]
    fn detect_timeout_reset_by_zerox(){
        let mut zerox = ZeroX::new(4);

        let mut block : Vec<f32> = vec![-0.2, -0.4, 0.2, 0.5];

        zerox.process(&mut block[..]);

        assert_eq!(block, vec![0.0, 0.0, 0.4, 0.0]);
    }
    #[test]
    fn detect_long_silence(){
        let mut zerox = ZeroX::new(256);

        let mut a:Vec<f32> = vec![0.0; 128];
        let mut b:Vec<f32> = vec![0.0; 128];
        zerox.process(&mut a[..]);
        zerox.process(&mut b[..]);

        assert_eq!(a, vec![0.0; 128]);

        let mut expect_b: Vec<f32> = vec![0.0; 128];
        expect_b[127] = -1.0;

        assert_eq!(b, expect_b);
    }
}
