use crate::complex::Complex;
use mockall::*;

pub trait TProcessor
{
    fn process(&mut self, input: &mut Vec<f32>);
}

pub trait TIterator<T> {
    fn next(&mut self)->Option<&Vec<T>>;
    fn reset(&mut self);
}

pub trait TBlockRing<T> : TIterator<T>{
    fn push(&mut self, block: Vec<T>);
}

pub trait TComplexIR: TIterator<Complex>{
}

#[automock]
pub trait TFft {
    fn forward(&self, buffer: &Vec<f32>) -> Vec<Complex>;
    fn inverse(&self, complex_buffer: Vec<Complex>)-> Vec<f32>;
}
