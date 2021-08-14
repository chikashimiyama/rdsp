pub trait TBlockManager {
    fn len(&self) -> usize;
    fn at(&self, index: usize) -> &Vec<f32>;
}

pub struct BlockManager {
    buffer : Vec<Vec<f32>>
}

impl BlockManager {
    pub fn new(block_size: usize, num_blocks: usize) -> Self{
        let mut buf: Vec<Vec<f32>> = Vec::new();
        for _ in 0..num_blocks {
            let mut block : Vec<f32> = Vec::new();
            block.resize(block_size, 0.0);
            buf.push(block);
        }
        BlockManager{ buffer : buf }
    }
}

impl TBlockManager for BlockManager {
    fn len(&self) -> usize {
        self.buffer.len()
    }

    fn at(&self, index: usize) -> &Vec<f32> {
        &self.buffer[index]
    }
}
