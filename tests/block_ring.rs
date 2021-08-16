#[cfg(test)]

use rdsp::traits::*;
use rdsp::block_ring::*;

#[test]
fn new_blocks_filled_with_zeros() {
    let block_ring = BlockRing::new(3, 2);
    let mut index = 0;
    for block in block_ring {
        assert_eq!(0.0, block[index]);
        index += 1;
    }
}

#[test]
fn push() {
    let content : Vec<f32> = vec![0.0, 1.0, 2.0];
    let mut block_ring = BlockRing::new(1, 3);
    block_ring.push(vec![content[0]]);
    block_ring.push(vec![content[1]]);
    block_ring.push(vec![content[2]]);

    let mut index : i32 = 2;
    block_ring.into_iter().for_each(|block| {
        assert_eq!(block[0], content[index as usize]);
        index -= 1;
    });
}

#[test]
fn push_ring_buffer() {

    let content : Vec<f32> = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let mut block_ring = BlockRing::new(1, 3);
    for i in 0..6{
        block_ring.push(vec![content[i]]);
    }

    let mut index : i32 = 2;
    for block in block_ring {
        assert_eq!(block[0], content[(index + 3) as usize]);
        index -= 1;
    }
}
