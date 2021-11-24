pub struct Point{
    pub x: f32,
    pub y: f32
}

impl Point {
    pub fn new(x: f32, y: f32)->Self{
        Self{ x, y }
    }

    pub fn distance(&self, another: &Point )-> f32{
        ((another.x - self.x).powi(2) + (another.y - self.y).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn new(){
        let point = Point::new(0.2, 0.3);
        assert_eq!(point.x, 0.2);
        assert_eq!(point.y, 0.3);
    }

    #[test]
    fn distance(){
        let b = Point::new(0.4, 0.4);
        let a = Point::new(0.2, 0.1);

        assert_eq!((a.distance(&b) - ((0.2 * 0.2 + 0.3 * 0.3) as f32).sqrt()).abs() < 0.0001, true);
    }

    #[test]
    fn distance_reverse(){
        let a = Point::new(0.4, 0.4);
        let b = Point::new(0.2, 0.1);

        assert_eq!((a.distance(&b) - ((0.2 * 0.2 + 0.3 * 0.3) as f32).sqrt()).abs() < 0.0001, true);
    }
}