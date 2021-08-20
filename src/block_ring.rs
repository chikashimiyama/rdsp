use crate::traits::{TBlockRing, TIterator};
use crate::complex::Complex;

pub struct BlockRing {
    buffer : Vec<Vec<Complex>>,
    write_index : i32,
    count : usize,
}

impl BlockRing {
    pub fn new(block_size: usize, num_blocks: usize) -> Self{
        Self{
            buffer : vec![vec![Complex::new(0.0, 0.0); block_size]; num_blocks],
            write_index : num_blocks as i32 - 1,
            count : 0
        }
    }
}

impl TBlockRing for BlockRing {
    fn push(&mut self, block: Vec<Complex>) {
        self.write_index += 1;
        if self.write_index == self.buffer.len() as i32 {
            self.write_index = 0;
        }
        self.buffer[self.write_index as usize] = block;
    }
}

impl TIterator<Complex> for BlockRing{
    fn next(&mut self) -> Option<&Vec<Complex>> {
        if self.count >= self.buffer.len(){
            return None;
        }

        let mut index = self.write_index as i32 - self.count as i32;
        if index < 0{
            index += self.buffer.len() as i32;
        }
        let block = &self.buffer[index as usize];
        self.count += 1;
        Some(block)
    }

    fn reset(&mut self){
        self.count = 0;
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }
}
