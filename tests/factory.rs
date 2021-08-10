#[cfg(test)]

use rdsp::factory::*;

#[test]
fn create_fft() {

    let factory = Box::from(Factory{});
    let fft = factory.create_fft(64);


}

