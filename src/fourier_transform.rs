use rustfft::{algorithm::Radix4, Fft as Rfft, FftDirection};
use rustfft::num_complex::Complex32;

use crate::traits::TFft;
use crate::complex::Complex;

pub struct Fft {
    forward: Radix4<f32>,
    inverse: Radix4<f32>
}

impl Fft{
    pub fn new(block_size: usize) -> Self {
        Self {
            forward: Radix4::new(block_size * 2, FftDirection::Forward),
            inverse: Radix4::new(block_size * 2, FftDirection::Inverse),
        }
    }
}

impl TFft for Fft {
    fn forward(&self, buffer: &Vec<f32>) -> Vec<Complex> {
        let mut complex_buffer= to_complex_buffer(buffer);
        self.forward.process(&mut complex_buffer);
        return convert_from_rustfft(&complex_buffer);

        fn to_complex_buffer(buffer: &Vec<f32>) -> Vec<Complex32>{
            let length = buffer.len();
            let mut complex_buffer : Vec<Complex32> = vec![Complex32::new(0.0, 0.0); length];
            for i in 0..length {
                complex_buffer[i].re = buffer[i];
                complex_buffer[i].im = 0.0;
            }
            complex_buffer
        }

        fn convert_from_rustfft(buf: &Vec<Complex32>) -> Vec<Complex>{
            let length = buf.len();
            let mut vec : Vec<Complex> = Vec::with_capacity(length);
            let iter = buf.iter();
            for cpl in iter {
                vec.push(Complex{ real: cpl.re,  imaginary: cpl.im});
            }
            return vec;
        }
    }

    fn inverse(&self, complex_buffer: &Vec<Complex>) -> Vec<f32> {
        let mut rustfft_complex_buffer = convert_to_rustfft(&complex_buffer);
        self.inverse.process(&mut rustfft_complex_buffer);
        return to_buffer(&rustfft_complex_buffer);

        fn to_buffer(complex_buffer: &Vec<Complex32>) -> Vec<f32> {
            let length = complex_buffer.len();
            let mut buffer : Vec<f32> = vec![0.0; length];
            for i in 0..length {
                buffer[i] = complex_buffer[i].re / length as f32;
            }
            buffer
        }

        fn convert_to_rustfft(buf: &Vec<Complex>) -> Vec<Complex32>{
            let length = buf.len();
            let mut rustfft_buffer : Vec<Complex32> = Vec::with_capacity(length);
            let iter = buf.iter();
            for cpl in iter {
                rustfft_buffer.push(Complex32{ re: cpl.real,  im: cpl.imaginary});
            }
            return rustfft_buffer;
        }
    }
}



