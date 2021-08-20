#[cfg(test)]

use rdsp::factory::*;
use rdsp::traits::TProcessor;

#[test]
fn process_five_samples_delay(){
    let mut ir: Vec<f32> = vec![0.0; 256];
    ir[5] = 1.0;

    let mut rng = rand::thread_rng();
    let mut input: Vec<f32> = Vec::with_capacity(512);
    for _ in 0..512{
        input.push(rng.gen::<f32>());
    }

    let mut processor = create_convolution_processor(64, &ir);
    let mut result: Vec<f32> = Vec::new();
    for i in 0..8{
        let offset = i * 64;
        let mut in_buf =  input[offset..offset + 64].to_vec();
        processor.process(&mut in_buf);
    }
}
