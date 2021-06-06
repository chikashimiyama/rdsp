use super::t_in_place_processor::TInPlaceProcessor;

pub trait TConvolution : TInPlaceProcessor {
    fn new() -> Self;
    fn update_filter(&mut self, ir_data: Vec<f32>);
}

pub struct Convolution {
    block_size: u16,
    filter_ir: Vec<f32>
}

impl TInPlaceProcessor for Convolution {
    fn process(input: &mut Vec<f32>) {

    }
}

impl TConvolution for Convolution {
    fn new() -> Self {
        Self{
            block_size: 128,
            filter_ir: Vec::new()
        }
    }

    fn update_filter(&mut self, ir_data: Vec<f32>) {
        self.filter_ir = ir_data;
    }
}
