
pub trait Processor{
    fn process(&mut self, buffer: &mut[f32]);
}