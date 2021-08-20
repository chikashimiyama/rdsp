#[cfg(test)]
use rstest::*;
use mockall::predicate;

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
        .returning(|block| {
            let mut complex : Vec<Complex>= Vec::new();
            complex.resize(64, Complex::new(0.0, 0.0));
            complex
        });

    ComplexIR::new(64, &ir_data, mock_fft);
}

#[rstest]
fn next()
{
    let mut ir :  Vec<f32> = Vec::new();
    ir.resize(65, 0.0);
    let mut mock_fft = MockTFft::new();
    mock_fft.expect_forward()
        .returning(|block| {
            let mut complex : Vec<Complex>= Vec::new();
            complex.resize(64, Complex::new(0.0, 0.0));
            complex
        });

    let mut ir_data =  ComplexIR::new(64, &ir, mock_fft);

    assert_eq!(true, ir_data.next().is_some());
    assert_eq!(true, ir_data.next().is_some());
    assert_eq!(false, ir_data.next().is_some());
}
