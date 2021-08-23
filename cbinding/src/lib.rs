use rdsp::convolver_processor::ConvolutionProcessor;
use rdsp::factory::*;

use std::vec::*;
use rdsp::traits::TProcessor;

#[no_mangle]
pub unsafe extern fn create( block_size: usize, ir_data : *mut f32, ir_size: usize ) -> *mut ConvolutionProcessor {
    let ir_buf : Vec<f32> = Vec::from_raw_parts(ir_data, ir_size, ir_size);
    Box::into_raw(Box::new(create_convolution_processor(block_size, &ir_buf)))
}

#[no_mangle]
pub unsafe extern fn destroy( processor: *mut ConvolutionProcessor) {
    Box::from_raw(processor);
}

#[no_mangle]
pub unsafe extern fn process( processor: &mut ConvolutionProcessor, buffer : *mut f32, block_size: usize) {
    let mut in_buf = Vec::from_raw_parts(buffer, block_size, block_size);
    processor.process(&mut in_buf);
}

