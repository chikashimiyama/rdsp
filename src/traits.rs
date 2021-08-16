use crate::complex::Complex;

pub trait TInPlaceProcessor
{
    fn process(&self, input: &mut Vec<f32>);
}

pub trait TBlockRing {
    fn push(&mut self, block: Vec<f32>);
    fn next(&mut self)->&Vec<f32>;
    fn reset(&mut self);
}

pub trait TFft {
    fn forward(&self, buffer: &Vec<f32>) -> Vec<Complex>;
    fn inverse(&self, complex_buffer: Vec<Complex>)-> Vec<f32>;
}

pub trait TFactory {
    fn create_fft(&self, size: usize) -> Box<dyn TFft>;
    fn create_block_ring(&self, block_size: usize, num_blocks: usize)-> Box<dyn TBlockRing>;
}
