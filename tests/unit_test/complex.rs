#[cfg(test)]

use rdsp::complex::Complex;

#[test]
fn construction() {
    let complex = Complex::new(2.0, 3.0);
    assert_eq!(complex.real, 2.0);
    assert_eq!(complex.imaginary, 3.0);
}

#[test]
fn multiplication() {
    let complex_a = Complex::new(2.0, 3.0);
    let complex_b = Complex::new(4.0, 5.0);
    let result = complex_a * complex_b;

    let real = complex_a.real * complex_b.real - complex_a.imaginary * complex_b.imaginary;
    let imaginary = complex_a.real * complex_b.imaginary + complex_a.imaginary * complex_b.real;

    assert_eq!(result.real, real);
    assert_eq!(result.imaginary, imaginary);
}
