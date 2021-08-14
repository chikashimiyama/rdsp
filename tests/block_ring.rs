#[cfg(test)]

use rdsp::block_ring::*;

#[test]
fn push() {

    let content : Vec<f32> = vec![0.0, 1.0, 2.0];
    let mut block_ring = BlockRing::new(1, 3);
    block_ring.push(vec![content[0]]);
    block_ring.push(vec![content[1]]);
    block_ring.push(vec![content[2]]);

    let mut index : i32 = 2;
    let mut func = |block : &Vec<f32>| {
        assert_eq!(block[0], content[index as usize]);
        index -= 1;
    };
    block_ring.traverse(func);
}
