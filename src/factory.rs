use crate::traits::*;
use crate::fourier_transform::Fft;
use crate::block_ring::BlockRing;

pub struct Factory{
}

impl TFactory for Factory{
    fn create_fft(&self, size: usize)-> Box<dyn TFft> {
        Box::from(Fft::new(size))
    }

    fn create_block_ring(&self, block_size: usize, num_blocks: usize)-> Box<dyn TBlockRing<Item = Vec<f32>>> {
        Box::from(BlockRing::new(block_size, num_blocks))
    }


}
