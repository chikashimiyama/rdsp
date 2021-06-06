#[cfg(test)]

use rdsp::fft::*;
use rstest::*;
use assert_approx_eq::*;

#[rstest]
#[case(32)]
#[case(64)]
#[case(128)]
#[case(512)]
#[case(2048)]
fn forward_inverse(#[case] size: usize) {
    let mut fft_processor : FftProcessor = TFftProcessor::new(size);
    let mut buffer: Vec<f32> = vec![0.0; size];
    buffer[30] = 1.0;

    let complex_buffer = fft_processor.forward(&buffer);
    let result = fft_processor.inverse(complex_buffer);

    assert_approx_eq!(result[30], 1.0, 0.0001);
}
