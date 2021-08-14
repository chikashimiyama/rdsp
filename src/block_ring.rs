pub trait TBlockRing {
    fn push(&mut self, block: Vec<f32>);
    fn traverse<F>(&self, func: F) where F: FnMut(&Vec<f32>);
}

pub struct BlockRing {
    buffer : Vec<Vec<f32>>,
    write_index : i32,
}

impl BlockRing {
    pub fn new(block_size: usize, num_blocks: usize) -> Self{
        let mut buf: Vec<Vec<f32>> = Vec::new();
        for _ in 0..num_blocks {
            let mut block : Vec<f32> = Vec::new();
            block.resize(block_size, 0.0);
            buf.push(block);
        }
        BlockRing{
            buffer : buf,
            write_index : (num_blocks - 1) as i32,
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

    fn traverse<F>(&self, mut func: F) where F: FnMut(&Vec<f32>) {
        let buffer_length = self.buffer.len() as i32;
        let write_index = self.write_index;

        for i in 0..buffer_length{
            let mut index = write_index - i;
            if index < 0 { index += buffer_length; }
            func(&self.buffer[index as usize]);
        }
    }
}






