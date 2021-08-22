#[cfg(test)]
use rstest::*;

use rdsp::complex_ir::ComplexIR;
use rdsp::traits::{MockTFft, TIterator};
use rdsp::complex::Complex;

#[rstest]
#[case::short_ir_data(63, 1)]
#[case::matched_ir_data(64, 1)]
#[case::ir_longer_than_blocksize(65, 2)]
#[case::multiple_blocks_long_ir(257, 5)]
fn construction(#[case] ir_data_size: usize, #[case] num_blocks: usize) {
    let mut ir_data : Vec<f32> = Vec::new();
    ir_data.resize(ir_data_size, 0.0);
    let mut mock_fft = MockTFft::new();

    mock_fft.expect_forward()
        .times(num_blocks)
        .returning(|_| {
            let mut complex : Vec<Complex>= Vec::new();
            complex.resize(64, Complex::new(0.0, 0.0));
            complex
        });

    ComplexIR::new(64, &ir_data, &mock_fft);
}

#[test]
fn next()
{
    let mut ir :  Vec<f32> = Vec::new();
    ir.resize(65, 0.0);
    let mut mock_fft = MockTFft::new();
    mock_fft.expect_forward()
        .returning(|_| {
            let mut complex : Vec<Complex>= Vec::new();
            complex.resize(64, Complex::new(0.0, 0.0));
            complex
        });

    let mut ir_data =  ComplexIR::new(64, &ir, &mock_fft);

    assert_eq!(true, ir_data.next().is_some());
    assert_eq!(true, ir_data.next().is_some());
    assert_eq!(false, ir_data.next().is_some());
}

#[test]
fn next_buffer_doubled()
{
    let mut ir :  Vec<f32> = Vec::new();
    ir.resize(64, 0.0);
    let mut mock_fft = MockTFft::new();
    mock_fft.expect_forward()
        .returning(|_| {
            let mut complex : Vec<Complex>= Vec::new();
            complex.resize(128, Complex::new(0.0, 0.0));
            complex
        });

    let mut ir_data =  ComplexIR::new(64, &ir, &mock_fft);
    let block_len = ir_data.next().unwrap().len();
    assert_eq!(128, block_len);
}

#[rstest]
#[case::size_63(63, 1)]
#[case::size_64(64, 1)]
#[case::size_65(65, 2)]
#[case::size_128(128, 2)]
#[case::size_129(129, 3)]
fn len(#[case] size: usize, #[case] expected_blocks: usize) {

    let ir :  Vec<f32> = vec![0.0; size];
    let mut mock_fft = MockTFft::new();
    mock_fft.expect_forward()
        .returning(|_| {
            let mut complex : Vec<Complex>= Vec::new();
            complex.resize(64, Complex::new(0.0, 0.0));
            complex
        });

    let complex_ir =  ComplexIR::new(64, &ir, &mock_fft);
    assert_eq!(expected_blocks, complex_ir.len());
}
