pub struct XFader{
    pub mix: f32 // 0 = completely A, 1 = completely B, 0.5=half&half;
}

impl XFader {
    pub fn new(initial_mix: f32)->Self{
        Self{mix: initial_mix}
    }

    pub fn process(&self, buffer: &mut[f32], reference: &[f32]) {
        if buffer.len() != reference.len(){
            panic!()
        }

        for i in 0..buffer.len(){
            let gap = buffer[i] - reference[i];
            buffer[i] = gap * self.mix + reference[i];
        }
    }
}

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn process(){
        let xfader = XFader::new(0.5);
        let mut buffer : Vec<f32> = vec![1.0];
        xfader.process(&mut(buffer[..]), &(vec![-1.0])[..]);
        assert_eq!(buffer[0], 0.0);
    }

    #[test]
    fn mix(){
        let mut xfader = XFader::new(0.5);
        xfader.mix = 0.0;
        let mut buffer : Vec<f32> = vec![1.0];
        xfader.process(&mut(buffer[..]), &(vec![-1.0])[..]);
        assert_eq!(buffer[0], -1.0);
    }
}
