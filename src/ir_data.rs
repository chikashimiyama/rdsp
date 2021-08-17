use crate::traits::{TIterator, TFft, TIRData};
use crate::complex::Complex;
use crate::fourier_transform::Fft;

pub struct IRData{
    complex_blocks : Vec<Vec<Complex>>,
    count : usize
}

impl IRData {
    pub fn new<T : TFft>(block_size: usize, ir_data: &Vec<f32>, fft: T)->Self{

        let mut num_blocks : usize;
        {
            num_blocks = ir_data.len()/block_size;
            if ir_data.len() % block_size > 0 {
                num_blocks += 1;
            }
        }

        let mut padded_ir : Vec<f32>;
        {
            padded_ir = ir_data.to_vec();
            padded_ir.resize(block_size * num_blocks, 0.0);
        }

        let mut complex_blocks : Vec<Vec<Complex>>;
        {
            complex_blocks = Vec::with_capacity(num_blocks);
            for i in 0..num_blocks {
                let block = padded_ir[i * block_size..(i + 1) * block_size].to_vec();
                complex_blocks.push(fft.forward(&block));
            }
        }

        IRData{
            complex_blocks,
            count: 0
        }
    }
}

impl TIRData for IRData{}

impl TIterator<Complex> for IRData{
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
}

