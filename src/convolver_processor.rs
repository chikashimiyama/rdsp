use crate::traits::{TProcessor, TBlockRing, TComplexIR};
use crate::block_ring::BlockRing;
use crate::complex_ir::ComplexIR;

pub struct ConvolutionProcessor<I: TComplexIR = ComplexIR, B:TBlockRing = BlockRing>{
    complex_ir : I,
    block_ring : B
}

impl <I: TComplexIR, B:TBlockRing> ConvolutionProcessor<I, B>{
    pub fn new(complex_ir : I, block_ring : B)->Self{
        Self{
            complex_ir,
            block_ring
        }
    }
}

impl TProcessor for ConvolutionProcessor{
    fn process(&self, input: &mut Vec<f32>) {

    }
}
