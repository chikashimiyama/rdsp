#[cfg(test)]

use rdsp::traits::*;
use rdsp::block_ring::*;
use rdsp::complex::*;

#[test]
fn new_blocks_are_zero_filled() {
    let mut block_ring : BlockRing = BlockRing::new(1, 2);
    let block = block_ring.next();

    assert_eq!(1, block.len());
    assert_eq!(0.0, block[0].real);
    assert_eq!(0.0, block[0].imaginary);
}

#[test]
fn push() {
    let mut block_ring = BlockRing::new(1,3);
    block_ring.push(vec![Complex::new(0.0, 1.0)]);
    block_ring.push(vec![Complex::new(2.0, 3.0)]);
    block_ring.push(vec![Complex::new(4.0, 5.0)]);

    let a = block_ring.next();
    assert_eq!(4.0, a[0].real); assert_eq!(5.0, a[0].imaginary);
    let b = block_ring.next();
    assert_eq!(2.0, b[0].real); assert_eq!(3.0, b[0].imaginary);
    // let c = block_ring.next();
    // assert_eq!(0.0, c[0].real); assert_eq!(1.0, c[0].imaginary);
}

#[test]
fn push_ring_buffer() {

    let mut block_ring = BlockRing::new(1,2);
    block_ring.push(vec![Complex::new(0.0, 1.0)]);
    block_ring.push(vec![Complex::new(2.0, 3.0)]);
    block_ring.push(vec![Complex::new(4.0, 5.0)]);

    let a = block_ring.next();
    assert_eq!(4.0, a[0].real); assert_eq!(5.0, a[0].imaginary);
    let b = block_ring.next();
    assert_eq!(2.0, b[0].real); assert_eq!(3.0, b[0].imaginary);
}
