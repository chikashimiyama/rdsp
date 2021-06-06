pub trait TInPlaceProcessor
{
    fn process(input: &mut Vec<f32>);
}
