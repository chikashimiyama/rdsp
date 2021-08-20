use crate::traits::{TProcessor, TBlockRing, TComplexIR, TFft, TIterator};
use crate::block_ring::BlockRing;
use crate::complex_ir::ComplexIR;
use crate::fourier_transform::Fft;
use crate::complex::Complex;

pub struct ConvolutionProcessor<I: TComplexIR = ComplexIR, B:TBlockRing<Complex> = BlockRing<Complex>, F:TFft = Fft>{
    double_size_buffer: Vec<f32>,
    previous_second_half : Vec<f32>,
    complex_ir : I,
    block_ring : B,
    fft : F,
}

impl <I: TComplexIR, B:TBlockRing<Complex>, F:TFft> ConvolutionProcessor<I, B, F>{
    pub fn new(block_size: usize, complex_ir : I, block_ring : B, fft: F)->Self{
        Self{
            double_size_buffer: vec![0.0; block_size * 2],
            previous_second_half: vec![0.0; block_size],
            complex_ir,
            block_ring,
            fft
        }
    }
}

impl TProcessor for ConvolutionProcessor{
    fn process(&mut self, input: &mut Vec<f32>) {

        let block_size = input.len();
        for i in 0..block_size {
            self.double_size_buffer[i] = input[i];
            self.double_size_buffer[i+block_size] = 0.0;
        }

        let complex_input = self.fft.forward(&self.double_size_buffer);
        self.block_ring.push(complex_input);


        self.block_ring.reset();
        self.complex_ir.reset();
        let mut accum : Vec<f32> = vec![0.0; block_size * 2];

        let mut maybe_input_block = self.block_ring.next();
        while let Some(input_block) = maybe_input_block {
            let filter_block = self.complex_ir.next().unwrap();
            let mut convoluted: Vec<Complex> = Vec::with_capacity(block_size * 2);
            for sample_pair in input_block.iter().zip(filter_block.iter()){
                let (input, filter) = sample_pair;
                let result = input * filter; // complex multiplication
                convoluted.push(result);
            }
            let processed = self.fft(convoluted);
            for i in block_size{
                accum[i] += processed[i];
            }
        }

        for i in 0..block_size{
            input[i] = accum[i] + self.previous_second_half[i];
            self.previous_second_half[i] = accum[i + block_size];
        }
    }
}


