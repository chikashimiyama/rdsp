#[cfg(test)]

use rstest::*;
use assert_approx_eq::*;

use rdsp::complex::Complex;
use rdsp::traits::TFft;
use rdsp::fourier_transform::*;

#[rstest]
#[case::block_size32(32)]
#[case::block_size64(64)]
#[case::block_size128(128)]
#[case::block_size512(512)]
#[case::block_size2048(2048)]
fn forward_inverse(#[case] size: usize) {
    let fft: Fft = Fft::new(size);
    let mut buffer: Vec<f32> = vec![0.0; size * 2];
    buffer[30] = 1.0;

    let complex_buffer = fft.forward(&buffer);
    let result = fft.inverse(&complex_buffer);

    assert_approx_eq!(result[30], 1.0 as f32, 0.0001);
}

#[rstest]
#[case::dirac_1(1)]
#[case::dirac_50(50)]
#[case::dirac_100(100)]
#[case::dirac_127(127)]
fn dirac(#[case] index: usize) {
    let fft: Fft = Fft::new(128);
    let mut buffer: Vec<f32> = vec![0.0; 256];
    buffer[index] = 0.5;

    let complex_buffer_a = fft.forward(&buffer);
    let complex_buffer_b = fft.forward(&buffer);
    let mut complex_buffer_c = vec![ Complex{real:0.0, imaginary:0.0}; 256];

    for i in 0..256 {
        complex_buffer_c[i] = complex_buffer_a[i] * complex_buffer_b[i];
    }

    let result = fft.inverse(&complex_buffer_c);

    assert_approx_eq!(result[index * 2] , 0.25, 0.0001);
}
