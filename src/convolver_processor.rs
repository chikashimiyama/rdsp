use crate::traits::{TProcessor, TBlockRing, TComplexIR, TFft, TIterator};
use crate::block_ring::BlockRing;
use crate::complex_ir::ComplexIR;
use crate::fourier_transform::Fft;
use crate::complex::Complex;

pub struct ConvolutionProcessor<I: TComplexIR = ComplexIR, B:TBlockRing = BlockRing, F:TFft = Fft>{
    double_size_buffer: Vec<f32>,
    previous_second_half : Vec<f32>,
    accumulation_buffer: Vec<f32>,
    complex_ir : I,
    block_ring : B,
    fft : F,
}

impl <I: TComplexIR, B:TBlockRing, F:TFft> ConvolutionProcessor<I, B, F>{
    pub fn new(block_size: &usize, complex_ir : I, block_ring : B, fft: F)->Self{
        Self{
            double_size_buffer: vec![0.0; block_size * 2],
            previous_second_half: vec![0.0; *block_size],
            accumulation_buffer: vec![0.0; block_size * 2],
            complex_ir,
            block_ring,
            fft
        }
    }
}

impl TProcessor for ConvolutionProcessor{
    fn process(&mut self, input: &mut Vec<f32>) {

        // fill the first half of the dsbuffer with the input and the rest with zeros
        let block_size = input.len();
        for i in 0..block_size {
            self.double_size_buffer[i] = input[i];
            self.double_size_buffer[i+block_size] = 0.0;
        }

        self.block_ring.push(self.fft.forward(&self.double_size_buffer));

        self.accumulation_buffer.fill(0.0);
        self.block_ring.reset();
        self.complex_ir.reset();

        let doubled_block_size = self.double_size_buffer.len();
        for _ in 0..self.block_ring.len(){
            let input_block = self.block_ring.next().unwrap();
            let filter_block = self.complex_ir.next().unwrap();

            let mut convoluted: Vec<Complex> = Vec::with_capacity(doubled_block_size);
            for i in 0..doubled_block_size{
                convoluted.push(input_block[i] * filter_block[i]);
            }
            let processed = self.fft.inverse(convoluted);
            for i in 0..doubled_block_size{
                self.accumulation_buffer[i] += processed[i] / doubled_block_size as f32;
            }
        }

        for i in 0..block_size{
            input[i] = self.accumulation_buffer[i] + self.previous_second_half[i];
            self.previous_second_half[i] = self.accumulation_buffer[i + block_size];
        }
    }
}


