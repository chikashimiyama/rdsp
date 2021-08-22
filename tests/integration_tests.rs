#[cfg(test)]

use rstest::*;
use assert_approx_eq::*;

use rdsp::factory::*;
use rdsp::traits::TProcessor;

#[rstest]
#[case::dirac_delta_4_1(4, 1)]
#[case::dirac_delta_4_70(4, 70)]
#[case::dirac_delta_4_169(4, 160)]
#[case::dirac_delta_63_1(63, 1)]
#[case::dirac_delta_63_70(63, 70)]
#[case::dirac_delta_63_169(63, 160)]
#[case::dirac_delta_200_1(200, 1)]
#[case::dirac_delta_200_70(200, 70)]
#[case::dirac_delta_200_169(200, 169)]
fn dirac(#[case] impulse_ir: usize, #[case] input_ir: usize){
    let mut ir: Vec<f32> = vec![0.0; 256];
    ir[impulse_ir] = 1.0; // ir with one dirac at index 4

    let mut input_stream: Vec<f32> = vec![0.0; 512];
    input_stream[input_ir] = 0.5; // input with one dirac at index 3

    let mut processor = create_convolution_processor(64, &ir);
    let mut result: Vec<f32> = Vec::new();
    for i in 0..8{
        let offset = i * 64;
        let mut input_block =  input_stream[offset..offset + 64].to_vec();
        processor.process(&mut input_block);
        result.extend(input_block);
    }

    for i in 0..512 {
        if i == impulse_ir + input_ir{
            assert_approx_eq!(0.5, result[i], 0.000005);
        }
        else{
            assert_approx_eq!(0.0, result[i], 0.000005);
        }
    }
}

#[rstest]
#[case::dirac_delta_1(1)]
#[case::dirac_delta_6(6)]
#[case::dirac_delta_20(20)]
#[case::dirac_delta_100(100)]
fn sine_delay(#[case] dirac : usize){
    let mut ir: Vec<f32> = vec![0.0; 256];
    ir[dirac] = 1.0;

    let mut input_stream: Vec<f32> = vec![0.0; 512];
    for i in 0..512{
        input_stream[i] = f32::sin(i as f32 / 10.0);
    }

    let mut processor = create_convolution_processor(64, &ir);
    let mut result: Vec<f32> = Vec::new();
    for i in 0..8{
        let offset = i * 64;
        let mut input_block =  input_stream[offset..offset + 64].to_vec();
        processor.process(&mut input_block);
        result.extend(input_block);
    }

    for i in 0..(512-dirac) {
        assert_approx_eq!(input_stream[i], result[i+dirac], 0.000005);
    }
}
