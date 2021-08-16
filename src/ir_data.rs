use crate::traits::{TIterator, TFactory};
use crate::complex::Complex;

pub struct IRData{
    padded_ir: Vec<f32>,
    complex_blocks : Vec<Vec<Complex>>,
    count : usize
}

impl IRData {
    fn new(block_size: usize, ir_data: &Vec<f32>, factory: &dyn TFactory)->Self{

        let num_blocks = get_num_blocks(ir_data.len(), block_size);
        let padded_ir = get_padded_ir(ir_data, block_size, num_blocks);
        let complex_blocks = get_complex_blocks(factory, &padded_ir, block_size, num_blocks);

        return IRData{
            padded_ir,
            complex_blocks,
            count: 0
        };

        fn get_num_blocks(samples : usize, block_size : usize)->usize{
            let mut num_blocks = samples/block_size;
            if samples % block_size > 0 {
                num_blocks += 1;
            }
            num_blocks
        }

        fn get_padded_ir(ir_data: &Vec<f32>, block_size:usize, num_blocks:usize)->Vec<f32>{
            let mut padded_ir = ir_data.to_vec();
            padded_ir.resize(block_size * num_blocks, 0.0);
            padded_ir
        }

        fn get_complex_blocks(factory : &dyn TFactory, padded_ir: &Vec<f32>, block_size:usize, num_blocks: usize)->Vec<Vec<Complex>>{
            let fft = factory.create_fft(block_size);
            let mut complex_blocks : Vec<Vec<Complex>> = Vec::with_capacity(num_blocks);
            for i in 0..num_blocks-1 {
                let block = padded_ir[i * block_size..(i+1) * block_size].to_vec();
                complex_blocks.push(fft.forward(&block));
            }
            complex_blocks
        }
    }
}

impl TIterator<Complex> for IRData{
    fn next(&mut self) -> Option<&Vec<Complex>> {
        if self.count >= self.complex_blocks.len(){
            return None
        }
        self.count += 1;
        Some(&self.complex_blocks[self.count])
    }

    fn reset(&mut self) {
        self.count = 0;
    }
}
