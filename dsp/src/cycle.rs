pub struct Cycle{
    size: usize,
    phase: usize,
    increment: f32,
    scale: f32
}

impl Cycle {
    pub fn new(size: usize, duty_cycle: f32) -> Self{
        let cooked_dc = duty_cycle.clamp(0.001, 1.0);
        Self{
            size,
            phase: 0,
            increment: 1.0 / size as f32,
            scale: 1.0/cooked_dc
        }
    }

    pub fn phase(&self) -> f32 {
        self.phase as f32 * self.increment * self.scale
    }

    pub fn advance(&mut self){
        self.phase += 1;
    }

    pub fn is_ended(&self) -> bool{
        self.phase >= self.size
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn new(){
        let cycle = Cycle::new(6, 1.0 );
        assert_eq!(cycle.size, 6);
        assert_eq!(cycle.phase, 0);
    }

    #[test]
    fn phase(){
        let mut cycle = Cycle::new(6,  1.0);
        cycle.advance();
        cycle.advance();
        cycle.advance();

        assert_eq!(cycle.phase(), 0.5);
    }

    #[test]
    fn radian_too_high_dc(){
        let cycle = Cycle::new(6,  1.001);
        assert_eq!(cycle.scale, 1.0)
    }

    #[test]
    fn radian_too_zero_dc(){
        let cycle = Cycle::new(6,  0.0);
        assert_eq!(cycle.scale, 1.0/0.001);
    }

    #[test]
    fn radian_half_dc_schrunk(){
        let mut cycle = Cycle::new(8,  0.5);
        cycle.advance();
        assert_eq!(cycle.phase(), 1.0 / 8.0 * 1.0 / 0.5);
    }

    #[test]
    fn advance(){
        let mut cycle = Cycle::new(6, 1.0);
        cycle.advance();
        assert_eq!(cycle.phase, 1);
    }

    #[test]
    fn is_ended(){
        let mut cycle = Cycle::new(2,  1.0);
        assert_eq!(cycle.is_ended(), false);
        cycle.advance();
        assert_eq!(cycle.is_ended(), false);
        cycle.advance();
        assert_eq!(cycle.is_ended(), true);
    }
}