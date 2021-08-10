use rustfft::{algorithm::Radix4, Fft as Rfft, FftDirection};
use rustfft::num_complex::Complex32;

pub trait TFft {
    fn forward(&self, buffer: &Vec<f32>) -> Vec<Complex32>;
    fn inverse(&self, complex_buffer: Vec<Complex32>)-> Vec<f32>;
}

pub struct Fft {
    forward: Radix4<f32>,
    inverse: Radix4<f32>
}

impl Fft{
    pub fn new(size: usize) -> Self {
        Fft {
            forward: Radix4::new(size, FftDirection::Forward),
            inverse: Radix4::new(size, FftDirection::Inverse),
        }
    }
}

impl TFft for Fft {
    fn forward(&self, buffer: &Vec<f32>) -> Vec<Complex32> {
        let mut complex_buffer= to_complex_buffer(buffer);
        self.forward.process(&mut complex_buffer);
        complex_buffer
    }

    fn inverse(&self, mut complex_buffer: Vec<Complex32>) -> Vec<f32> {
        self.inverse.process(&mut complex_buffer);
        to_buffer(&complex_buffer)
    }
}

fn to_complex_buffer(buffer: &Vec<f32>) -> Vec<Complex32>{
    let length = buffer.len();
    let mut complex_buffer : Vec<Complex32> = vec![Complex32::new(0.0, 0.0); length];
    for i in 0..length {
        complex_buffer[i].re = buffer[i] / length as f32;
        complex_buffer[i].im = 0.0;
    }
    complex_buffer
}

fn to_buffer(complex_buffer: &Vec<Complex32>) -> Vec<f32> {
    let length = complex_buffer.len();
    let mut buffer : Vec<f32> = vec![0.0; length];
    for i in 0..length {
        buffer[i] = complex_buffer[i].re;
    }
    buffer
}
