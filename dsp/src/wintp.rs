use crate::point::Point;
use crate::waveform::Waveform;
use crate::processor::Processor;

pub struct WaveInterpolator{
    point : Point,

    left_top: Point,
    right_top: Point,
    left_bottom: Point,
    right_bottom: Point
}

impl Processor for WaveInterpolator  {
    fn process(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut(){
            let mut value: f32 = 0.0;
            value += Waveform::sine(*sample) * self.proximity(&self.left_top);
            value += Waveform::triangle(*sample) * self.proximity(&self.left_bottom);
            value += Waveform::saw(*sample) * self.proximity(&self.right_top);
            value += Waveform::square(*sample) * self.proximity(&self.right_bottom);
            *sample = value;
        }
    }
}

impl WaveInterpolator {
    pub fn new() -> Self {
        Self {
            point: Point::new(0.0, 0.0),
            left_top: Point::new(0.0, 0.0),
            right_top: Point::new(1.0, 0.0),
            left_bottom: Point::new(0.0, 1.0),
            right_bottom: Point::new(1.0, 1.0)
        }
    }

    pub fn set(&mut self, x: f32, y: f32)
    {
        self.point.x = x;
        self.point.y = y;
    }

    fn proximity(&self, another: &Point) -> f32{
        let mut distance = self.point.distance(another);
        if distance > 1.0 {
            distance = 1.0;
        }
        return 1.0 - distance;
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn proximity(){
        let mut wave_interpolator = WaveInterpolator::new();
        wave_interpolator.set(0.7, 0.6);

        assert_eq!(wave_interpolator.proximity(&Point::new(1.0, 1.0)), 1.0 - (0.3_f32.powi(2) + 0.4_f32.powi(2)).sqrt());
        assert_eq!(wave_interpolator.proximity(&Point::new(0.0, 0.0)), 1.0 - (0.7_f32.powi(2) + 0.6_f32.powi(2)).sqrt());
    }

    #[test]
    fn sine(){
        let mut wave_interpolator = WaveInterpolator::new();
        wave_interpolator.set(0.0, 0.0);

        let mut buffer = vec![0.32];
        wave_interpolator.process(&mut buffer[..]);
        assert_eq!(buffer[0], Waveform::sine(0.32));
    }

    #[test]
    fn saw(){
        let mut wave_interpolator = WaveInterpolator::new();
        wave_interpolator.set(1.0, 0.0);

        let mut buffer = vec![0.32];
        wave_interpolator.process(&mut buffer[..]);
        assert_eq!(buffer[0], Waveform::saw(0.32));
    }

    #[test]
    fn triangle(){
        let mut wave_interpolator = WaveInterpolator::new();
        wave_interpolator.set(0.0, 1.0);

        let mut buffer = vec![0.32];
        wave_interpolator.process(&mut buffer[..]);
        assert_eq!(buffer[0], Waveform::triangle(0.32));
    }

    #[test]
    fn square(){
        let mut wave_interpolator = WaveInterpolator::new();
        wave_interpolator.set(1.0, 1.0);

        let mut buffer = vec![0.32];
        wave_interpolator.process(&mut buffer[..]);
        assert_eq!(buffer[0], Waveform::square(0.32));
    }

    #[test]
    fn sine_triangle(){
        let mut wave_interpolator = WaveInterpolator::new();
        wave_interpolator.set(0.0, 0.5);

        let mut buffer = vec![0.32];
        wave_interpolator.process(&mut buffer[..]);
        assert_eq!(buffer[0], Waveform::sine(0.32) * 0.5 + Waveform::triangle(0.32) * 0.5);
    }
}