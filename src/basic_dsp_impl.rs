use basic_dsp::CrossCorrelationArgumentOps;
use basic_dsp::CrossCorrelationOps;
use basic_dsp::DspVec;
use basic_dsp::ReorganizeDataOps;
use basic_dsp::SingleBuffer;
use basic_dsp::TimeToFrequencyDomainOperations;
use basic_dsp::ToComplexVector;
use basic_dsp::ToRealVector;
use basic_dsp::Vector;

pub fn f(x: f32) -> f32 {
    let position = 128.0;
    if x != position {
        (x - position).sin() * 100.0 / (x - position).abs()
    } else {
        100.0
    }
}

pub fn gcc_with_basic_dsp(shift: f32) -> f32 {
    let mut buffer = SingleBuffer::new();

    let mut sample1 = (0..256)
        .map(|i| f(i as f32))
        .collect::<Vec<_>>()
        .to_complex_time_vec();
    let sample2 = (0..256)
        .map(|i| f(i as f32 + shift))
        .collect::<Vec<_>>()
        .to_complex_time_vec();
    // sample1.swap_halves();

    let argument = sample2.prepare_argument_padded(&mut buffer);
    let result = sample1.correlate(&mut buffer, &argument);
    // println!("{:?}", result);
    // println!("{:?}", sample1.data.len());
    // println!(
    //     "Argmax, Max: {:?}",
    //     sample1
    //         .data
    //         .iter()
    //         .enumerate()
    //         .max_by(|&(_, first), &(_, second)| first.partial_cmp(second).unwrap())
    //         .map(|(index, value)| (index - (256 - 2), value))
    //         .expect("No values in result?")
    // );

    let (argmax, _max) = sample1
        .data
        .iter()
        .enumerate()
        .max_by(|&(_, first), &(_, second)| first.partial_cmp(second).unwrap())
        .map(|(index, value)| (index as f32 - (256.0 - 2.0), value))
        .expect("No values in result?");
    // println!("basic_dsp: {}, {}", shift, argmax);
    argmax
}
