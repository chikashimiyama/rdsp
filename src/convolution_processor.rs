use super::t_in_place_processor::TInPlaceProcessor;
use super::fourier_transform::TFft;
use super::factory::TFactory;
use rustfft::num_complex::Complex32;

pub struct ConvolutionProcessor {
    ir_data: Vec<Complex32>,
    fft: Box<dyn TFft>
}

impl ConvolutionProcessor {
    pub fn new(mut orig_ir_data: Vec<f32>, fft_size: usize, factory: Box<dyn TFactory>) -> Box<Self> {
        let reminder = orig_ir_data.len() % fft_size;
        for _ in 0..reminder {
            orig_ir_data.push(0.0);
        }
        let fft = factory.create_fft(fft_size);
        Box::from(ConvolutionProcessor {
            ir_data: fft.forward(&orig_ir_data),
            fft
        })
    }
}

impl TInPlaceProcessor for ConvolutionProcessor {
    fn process(&self, input: &mut Vec<f32>) {
        let complex_input = self.fft.forward(input);

    }
}
