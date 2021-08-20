#[cfg(test)]

use rdsp::traits::*;
use rdsp::block_ring::*;

#[test]
fn new_blocks_are_empty() {
    let mut block_ring : BlockRing<f32> = BlockRing::new(3, 2);
    while let Some(block) = block_ring.next()  {
        assert_eq!(0, block.len());
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
    while let Some(block) = block_ring.next()  {
        assert_eq!(content[index as usize], block[0]);
        index -= 1;
    };
}

#[test]
fn push_ring_buffer() {

    let content : Vec<f32> = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let mut block_ring = BlockRing::new(1, 3);
    for i in 0..6{
        block_ring.push(vec![content[i]]);
    }

    let mut index : i32 = 2;
    while let Some(block) = block_ring.next()  {
        assert_eq!(content[index as usize + 3], block[0]);
        index -= 1;
    };
}

#[test]
fn reset() {

    let content : Vec<f32> = vec![0.0, 1.0, 2.0];
    let mut block_ring = BlockRing::new(1, 3);
    for i in 0..3{
        block_ring.push(vec![content[i]]);
    }

    assert_eq!(block_ring.next().unwrap()[0], 2.0);
    assert_eq!(block_ring.next().unwrap()[0], 1.0);
    block_ring.reset();
    assert_eq!(block_ring.next().unwrap()[0], 2.0);
}
