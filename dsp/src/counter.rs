pub struct Counter{
    target: usize,
    count: usize,
}

impl Counter{
    pub fn new(target: usize) ->Self {
        Self{
            target,
            count: 0
        }
    }

    pub fn count(&mut self) -> bool{
        self.count += 1;
        if self.count >= self.target  {
            self.reset();
            return true;
        }
        return false;
    }

    pub fn reset(&mut self){
        self.count = 0;
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn count_normal(){
        let mut counter = Counter::new(2);

        assert!(!counter.count())
    }

    #[test]
    fn count_reach(){
        let mut counter = Counter::new(2);
        assert!(!counter.count());
        assert!(counter.count());
    }

    #[test]
    fn reset(){
        let mut counter = Counter::new(2);
        assert!(!counter.count());
        counter.reset();
        assert!(!counter.count());
    }
}
