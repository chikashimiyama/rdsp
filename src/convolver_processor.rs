use crate::traits::{TProcessor, TBlockRing, TComplexIR, TFft};
use crate::block_ring::BlockRing;
use crate::complex_ir::ComplexIR;
use crate::fourier_transform::Fft;
use crate::complex::Complex;

pub struct ConvolutionProcessor<I: TComplexIR = ComplexIR, B:TBlockRing<Complex> = BlockRing<Complex>, F:TFft = Fft>{
    ols_in_buf : Vec<f32>,
    complex_ir : I,
    block_ring : B,
    fft : F
}

impl <I: TComplexIR, B:TBlockRing<Complex>, F:TFft> ConvolutionProcessor<I, B, F>{
    pub fn new(block_size: usize, complex_ir : I, block_ring : B, fft: F)->Self{
        Self{
            ols_in_buf: vec![0.0; block_size * 2],
            complex_ir,
            block_ring,
            fft,
        }
    }
}

impl TProcessor for ConvolutionProcessor{
    fn process(&mut self, input: &mut Vec<f32>) {

        let block_size = input.len();
        // shift second half to first half .. fill second half with incoming signal
        for i in 0..block_size{
            self.ols_in_buf[i] = self.ols_in_buf[i + block_size];
            self.ols_in_buf[i + block_size] = input[i];
        }

        let complex_input = self.fft.forward(&self.ols_in_buf);
        self.block_ring.push(complex_input);



    }
}


