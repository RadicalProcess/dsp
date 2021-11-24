use core::f32::consts::PI;

pub struct Waveform{}

impl Waveform {
    pub fn sine(phase: f32) -> f32 {
        Self::process_if_in_range(phase, |p| { (p * 2.0 * PI).sin() })
    }

    pub fn saw(phase: f32) -> f32 {
        Self::process_if_in_range(phase, |p| { 1.0 - p * 2.0 })
    }

    pub fn triangle(phase: f32) -> f32 {
        Self::process_if_in_range(phase, |p| {
            if p < 0.25 { p * 4.0 } else if 0.25 <= p && p <= 0.75 { 1.0 - (p - 0.25) * 4.0 } else { return p * 4.0 - 4.0 }
        })
    }

    pub fn square(phase: f32) -> f32 {
        Self::process_if_in_range(phase, |phase| { if phase > 0.5 { -1.0 } else { 1.0 } })
    }

    fn process_if_in_range<F>(phase: f32, f: F) -> f32 where
        F: Fn(f32) -> f32 { if 0.0 <= phase && phase <= 1.0 { f(phase) } else { 0.0 } }
}


#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn sine(){
        assert_eq!(Waveform::sine(0.0), 0.0);
        assert_eq!(Waveform::sine(0.25), (PI * 0.5).sin());
        assert_eq!(Waveform::sine(0.5), PI.sin());
        assert_eq!(Waveform::sine(0.75), (PI * 1.5).sin());
        assert_eq!(Waveform::sine(1.0), (2.0 * PI).sin());
    }

    #[test]
    fn saw(){
        assert_eq!(Waveform::saw(0.0), 1.0);
        assert_eq!(Waveform::saw(0.25), 0.5);
        assert_eq!(Waveform::saw(0.5), 0.0);
        assert_eq!(Waveform::saw(0.75), -0.5);
        assert_eq!(Waveform::saw(1.0), -1.0);
    }

    #[test]
    fn triangle(){
        assert_eq!(Waveform::triangle(0.0), 0.0);
        assert_eq!(Waveform::triangle(0.25), 1.0);
        assert_eq!(Waveform::triangle(0.5), 0.0);
        assert_eq!(Waveform::triangle(0.75), -1.0);
        assert_eq!(Waveform::triangle(1.0), 0.0);
    }

    #[test]
    fn square(){
        assert_eq!(Waveform::square(0.0), 1.0);
        assert_eq!(Waveform::square(0.25), 1.0);
        assert_eq!(Waveform::square(0.5), 1.0);
        assert_eq!(Waveform::square(0.5001), -1.0);
        assert_eq!(Waveform::square(0.75), -1.0);
        assert_eq!(Waveform::square(1.0), -1.0);
    }
    
}