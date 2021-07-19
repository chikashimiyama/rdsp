use super::fft::*;

pub trait TFactory {
    fn create_fft(&self, size: usize) -> Box<dyn TFftProcessor>;

}

pub struct Factory{
}

impl TFactory for Factory{
    fn create_fft(&self, size: usize)-> Box<dyn TFftProcessor> {
        Box::from(FftProcessor::new(size))
    }
}
