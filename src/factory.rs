use crate::complex_ir::ComplexIR;
use crate::fourier_transform::Fft;
use crate::block_ring::BlockRing;
use crate::convolver_processor::ConvolutionProcessor;
use crate::utility::*;

pub fn create_convolution_processor(block_size: usize, ir_data: &Vec<f32>)->ConvolutionProcessor{

    let block_size_doubled = block_size * 2;
    let num_blocks = get_num_blocks(block_size, ir_data.len());

    let fft = Fft::new(block_size_doubled);
    let complex_ir = ComplexIR::new(block_size, ir_data, &fft);
    let block_ring = BlockRing::new( block_size_doubled, num_blocks);
    ConvolutionProcessor::new(&block_size, complex_ir, block_ring, fft)
}
