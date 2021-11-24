pub struct LookAhead{
}

impl LookAhead {
    pub fn calculate(sample_rate: usize, lowest_freq: usize) -> usize{
        let wavelength = sample_rate / lowest_freq ;
        let buffer_length= wavelength * 2;
        LookAhead::find_pow_of_two(buffer_length)
    }

    fn find_pow_of_two(target: usize) -> usize{
        for n in 6..17{
            let buffer_size = 2_usize.pow(n);
            if buffer_size > target {
                return buffer_size
            }
        }
        panic!("too high target");
    }
}

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn calculate(){
        assert_eq!(LookAhead::calculate(44100, 20), 8192);
        assert_eq!(LookAhead::calculate(44100, 40), 4096);
        assert_eq!(LookAhead::calculate(44100, 50), 2048);
        assert_eq!(LookAhead::calculate(44100, 100), 1024);
        assert_eq!(LookAhead::calculate(44100, 200), 512);

        assert_eq!(LookAhead::calculate(48000, 20), 8192);
        assert_eq!(LookAhead::calculate(48000, 40), 4096);
        assert_eq!(LookAhead::calculate(48000, 50), 2048);
        assert_eq!(LookAhead::calculate(48000, 100), 1024);
        assert_eq!(LookAhead::calculate(48000, 200), 512);

        assert_eq!(LookAhead::calculate(88200, 20), 16384);
        assert_eq!(LookAhead::calculate(88200, 40), 8192);
        assert_eq!(LookAhead::calculate(88200, 50), 4096);
        assert_eq!(LookAhead::calculate(88200, 100), 2048);
        assert_eq!(LookAhead::calculate(88200, 200), 1024);

        assert_eq!(LookAhead::calculate(96000, 20), 16384);
        assert_eq!(LookAhead::calculate(96000, 40), 8192);
        assert_eq!(LookAhead::calculate(96000, 50), 4096);
        assert_eq!(LookAhead::calculate(96000, 100), 2048);
        assert_eq!(LookAhead::calculate(96000, 200), 1024);
    }
}
