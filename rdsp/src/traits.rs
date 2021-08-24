use crate::complex::Complex;
use mockall::*;

pub trait TProcessor {
    fn process(&mut self, input: &mut [f32]);
}

pub trait TIterator<T> {
    fn next(&mut self)->&Vec<T>;
}

pub trait TBlockRing : TIterator<Complex>{
    fn push(&mut self, block: Vec<Complex>);
}

pub trait TComplexIR: TIterator<Complex>{
}

#[automock]
pub trait TFft {
    fn forward(&self, buffer: &Vec<f32>) -> Vec<Complex>;
    fn inverse(&self, complex_buffer: &Vec<Complex>)-> Vec<f32>;
}
