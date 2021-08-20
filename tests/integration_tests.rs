#[cfg(test)]

use assert_approx_eq::*;

use rdsp::factory::*;
use rdsp::traits::TProcessor;
use rand::Rng;

#[test]
fn process_five_samples_delay(){
    let mut ir: Vec<f32> = vec![0.0; 256];
    ir[4] = 1.0; // ir with one dirac at index 4

    let mut generator = rand::thread_rng();
    let mut input: Vec<f32> = Vec::with_capacity(512);
    for _ in 0..512{
        input.push(generator.gen());
    }

    let mut processor = create_convolution_processor(64, &ir);
    let mut result: Vec<f32> = Vec::new();
    for i in 0..8{
        let offset = i * 64;
        let mut in_buf =  input[offset..offset + 64].to_vec();
        processor.process(&mut in_buf);
        result.extend(in_buf);
    }

    for i in 0..4 {
        assert_approx_eq!(0.0, result[i], 0.000001);
    }
    for i in 4..512{
        assert_approx_eq!(input[i-4], result[i], 0.000001);
    }
}
