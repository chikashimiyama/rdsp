#[cfg(test)]

use rdsp::fourier_transform::*;
use rstest::*;
use assert_approx_eq::*;
use rustfft::num_complex::Complex32;

#[rstest]
#[case(32)]
#[case(64)]
#[case(128)]
#[case(512)]
#[case(2048)]
fn forward_inverse(#[case] size: usize) {
    let mut fft: Fft = Fft::new(size);
    let mut buffer: Vec<f32> = vec![0.0; size];
    buffer[30] = 1.0;

    let complex_buffer = fft.forward(&buffer);
    let result = fft.inverse(complex_buffer);

    assert_approx_eq!(result[30], 1.0, 0.0001);
}

#[test]
fn dirac() {
    let mut fft: Fft = Fft::new(256);
    let mut buffer: Vec<f32> = vec![0.0; 256];
    buffer[6] = 0.5;

    let complex_buffer_a = fft.forward(&buffer);
    let complex_buffer_b = fft.forward(&buffer);
    let mut complex_buffer_c: Vec<Complex32> = vec![ Complex32::new(0.0, 0.0); 256];

    for i in 0..256 {
        complex_buffer_c[i] = complex_buffer_a[i] * complex_buffer_b[i];
    }

    let result = fft.inverse(complex_buffer_c);

    assert_approx_eq!(result[12] * 256.0, 0.25, 0.0001);
}
