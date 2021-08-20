use crate::traits::{TBlockRing, TIterator};

pub struct BlockRing<T> {
    buffer : Vec<Vec<T>>,
    write_index : i32,
    count : usize,
}

impl <T> BlockRing<T> {
    pub fn new(num_blocks: usize) -> Self{
        let mut buf: Vec<Vec<T>> = Vec::new();
        for _ in 0..num_blocks {
            let block : Vec<T> = Vec::new();
            buf.push(block);
        }
        Self{
            buffer : buf,
            write_index : (num_blocks - 1) as i32,
            count : 0
        }
    }
}

impl <T> TBlockRing<T> for BlockRing<T> {
    fn push(&mut self, block: Vec<T>) {
        self.write_index += 1;
        if self.write_index == self.buffer.len() as i32 {
            self.write_index = 0;
        }
        self.buffer[self.write_index as usize] = block;
    }
}

impl <T> TIterator<T> for BlockRing<T>{
    fn next(&mut self) -> Option<&Vec<T>> {
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
