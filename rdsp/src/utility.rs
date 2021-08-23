
pub fn get_num_blocks(block_size: usize, samples : usize)->usize{
    let mut num_blocks = samples/block_size;
    if samples % block_size > 0 {
        num_blocks += 1;
    }
    num_blocks
}

