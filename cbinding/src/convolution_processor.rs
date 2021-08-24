use rdsp::convolution_processor::ConvolutionProcessor;
use rdsp::factory::*;
use std::vec::*;
use rdsp::traits::TProcessor;

pub struct ConvolutionProcessorObj {
    convolution_processor: ConvolutionProcessor,
}

impl ConvolutionProcessorObj {
    pub fn new(block_size: usize, ir_data: &Vec<f32>)->Self{
        Self {
            convolution_processor: create_convolution_processor(block_size, &ir_data)
        }
    }

    pub fn process(&mut self, input: &mut [f32]){
        self.convolution_processor.process(input);
    }
}

#[no_mangle]
pub extern fn rdsp_convolution_processor_create( block_size: usize, ir_data : *mut f32, ir_size: usize ) -> *mut ConvolutionProcessorObj {
    let ir_buf = unsafe { std::slice::from_raw_parts(ir_data, ir_size)}.to_vec();
    Box::into_raw(Box::new(ConvolutionProcessorObj::new(block_size, &ir_buf)))
}

#[no_mangle]
pub unsafe extern fn rdsp_convolution_processor_destroy( obj: *mut ConvolutionProcessorObj) {
    Box::from_raw(obj);
}

#[no_mangle]
pub extern fn rdsp_convolution_processor_process(obj: &mut ConvolutionProcessorObj, buffer : *mut f32, block_size: usize) {
    let in_buf= unsafe { std::slice::from_raw_parts_mut(buffer, block_size)};
    obj.process( in_buf);
}
