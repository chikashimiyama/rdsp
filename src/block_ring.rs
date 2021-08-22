use crate::traits::{TBlockRing, TIterator};
use crate::complex::Complex;

pub struct BlockRing {
    buffer : Vec<Vec<Complex>>,
    write_index : usize,
    count : usize,
}

impl BlockRing {
    pub fn new(block_size: usize, num_blocks: usize) -> Self{
        debug_assert!(block_size > 0 && num_blocks > 0, "both block size and num blocks should be greater than 0");

        Self{
            buffer : vec![vec![Complex::new(0.0, 0.0); block_size]; num_blocks],
            write_index : num_blocks - 1,
            count : 0
        }
    }
}

impl TBlockRing for BlockRing {
    fn push(&mut self, block: Vec<Complex>) {
        self.write_index = (self.write_index + 1) % self.buffer.len();
        self.buffer[self.write_index] = block;
    }
}

impl TIterator<Complex> for BlockRing{
    fn next(&mut self) -> &Vec<Complex> {
        let index : usize;
        if self.count > self.write_index{
            index =  self.buffer.len() - (self.count - self.write_index);
        } else {
            index = self.write_index - self.count;
        }

        let block = &self.buffer[index];
        self.count = (self.count + 1) % self.buffer.len();
        return block;
    }
}
