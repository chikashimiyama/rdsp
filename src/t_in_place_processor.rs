pub trait TInPlaceProcessor
{
    fn process(&self, input: &mut Vec<f32>);
}
