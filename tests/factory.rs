#[cfg(test)]

use rdsp::factory::*;
use rdsp::fft::FftProcessor;
use std::ptr::null;

#[test]
fn create_fft() {

    let factory = Box::from(Factory{});
    let fft = factory.create_fft(64);


}

