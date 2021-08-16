use crate::traits::TBlockRing;

pub struct BlockRing {
    buffer : Vec<Vec<f32>>,
    write_index : i32,
    count : usize,
}

impl BlockRing {
    pub fn new(block_size: usize, num_blocks: usize) -> Self{
        let mut buf: Vec<Vec<f32>> = Vec::new();
        for _ in 0..num_blocks {
            let mut block : Vec<f32> = Vec::new();
            block.resize(block_size, 0.0);
            buf.push(block);
        }
        Self{
            buffer : buf,
            write_index : (num_blocks - 1) as i32,
            count : 0
        }
    }
}

impl TBlockRing for BlockRing {

    fn push(&mut self, block: Vec<f32>) {
        self.write_index += 1;
        if self.write_index == self.buffer.len() as i32 {
            self.write_index = 0;
        }
        self.buffer[self.write_index as usize] = block;
    }
}

impl Iterator for BlockRing{
    type Item = Vec<f32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= self.buffer.len() {
           return None;
        }

        let mut index = self.write_index - self.count as i32;
        if index < 0 { index += self.buffer.len() as i32; }

        let block = self.buffer[index as usize].to_vec();
        self.count += 1;
        return Some(block);
    }
}





