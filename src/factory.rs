use super::fourier_transform::*;

pub trait TFactory {
    fn create_fft(&self, size: usize) -> Box<dyn TFft>;
}

pub struct Factory{
}

impl TFactory for Factory{
    fn create_fft(&self, size: usize)-> Box<dyn TFft> {
        Box::from(Fft::new(size))
    }
}
