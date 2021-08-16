use std::ops::{Mul};
use std::clone::Clone;

#[derive(Copy, Clone)]
pub struct Complex
{
    pub real: f32,
    pub imaginary: f32
}

impl Complex{
    pub fn new( real : f32, imaginary: f32) -> Self
    {
        Self {real, imaginary }
    }
}

impl Mul for Complex{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let k1 = other.real * (self.real + self.imaginary);
        let k2 = self.real * (other.imaginary - other.real);
        let k3 = self.imaginary * (other.real + other.imaginary);
        Self{ real: k1-k3, imaginary: k1+k2}
    }
}
