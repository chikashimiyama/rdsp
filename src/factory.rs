use crate::traits::TProcessor;
use crate::complex_ir::ComplexIR;
use crate::fourier_transform::Fft;
use crate::block_ring::BlockRing;
use crate::convolver_processor::ConvolutionProcessor;
use crate::utility::*;

pub fn create_convolution_processor(block_size: usize, ir_data: &Vec<f32>)->ConvolutionProcessor{

    let fft = Fft::new(block_size);
    let complex_ir = ComplexIR::new(block_size, ir_data, fft);
    let num_blocks = get_num_blocks(block_size, ir_data.len());
    let block_ring = BlockRing::new(block_size, num_blocks);
    ConvolutionProcessor::new(complex_ir, block_ring)
}
