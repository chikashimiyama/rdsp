use crate::traits::{TIterator, TFft, TComplexIR};
use crate::complex::Complex;
use crate::fourier_transform::Fft;
use crate::utility::*;

pub struct ComplexIR {
    complex_blocks : Vec<Vec<Complex>>,
    count : usize
}

impl ComplexIR {
    pub fn new<T : TFft>(block_size: usize, ir_data: &Vec<f32>, fft: T)->Self{

        let mut num_blocks = get_num_blocks(block_size, ir_data.len());

        let mut padded_ir : Vec<f32>;
        {
            padded_ir = ir_data.to_vec();
            padded_ir.resize(block_size * num_blocks, 0.0);
        }

        let mut complex_blocks : Vec<Vec<Complex>>;
        {
            complex_blocks = Vec::with_capacity(num_blocks);
            for i in 0..num_blocks {
                let mut block = padded_ir[i * block_size..(i + 1) * block_size].to_vec();
                block.resize(block_size * 2, 0.0);
                complex_blocks.push(fft.forward(&block));
            }
        }

        ComplexIR {
            complex_blocks,
            count: 0
        }
    }
}

impl TComplexIR for ComplexIR {}

impl TIterator<Complex> for ComplexIR {
    fn next(&mut self) -> Option<&Vec<Complex>> {
        if self.count >= self.complex_blocks.len(){
            return None
        }
        let complex = &self.complex_blocks[self.count];
        self.count += 1;
        Some(complex)
    }

    fn reset(&mut self) {
        self.count = 0;
    }

    fn len(&self) -> usize {
        self.complex_blocks.len()
    }
}

