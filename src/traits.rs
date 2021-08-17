use crate::complex::Complex;
use mockall::*;

pub trait TInPlaceProcessor
{
    fn process(&self, input: &mut Vec<f32>);
}

pub trait TIterator<T> {
    fn next(&mut self)->Option<&Vec<T>>;
    fn reset(&mut self);
}

pub trait TBlockRing : TIterator<f32>{
    fn push(&mut self, block: Vec<f32>);
}

pub trait TIRData : TIterator<Complex>{
}

#[automock]
pub trait TFft {
    fn forward(&self, buffer: &Vec<f32>) -> Vec<Complex>;
    fn inverse(&self, complex_buffer: Vec<Complex>)-> Vec<f32>;
}
