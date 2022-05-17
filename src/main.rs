use realfft::RealFftPlanner;

mod basic_dsp_impl;

use basic_dsp_impl::f;

fn dumb_implementation(shift: f32) -> f32 {
    let length = 256;
    let zeroes = (0..length).map(|_| 0.0).collect::<Vec<_>>();
    let sample1 = (0..length)
        .map(|i| f(i as f32))
        .chain(zeroes.clone().into_iter())
        .collect::<Vec<_>>();

    let sample2 = (0..length)
        .map(|i| f(i as f32 + shift))
        .chain(zeroes.into_iter())
        .collect::<Vec<_>>();

    let correlation = (0..512)
        .map(|x| {
            sample1
                .iter()
                .skip(x)
                .zip(sample2.iter())
                .map(|(a, b)| a * b)
                .sum::<f32>()
        })
        .collect::<Vec<_>>();
    let (argmax, _max) = correlation
        .iter()
        .enumerate()
        .max_by(|&(_, first), &(_, second)| first.partial_cmp(second).unwrap())
        .map(|(index, value)| (index, value))
        .expect("No values in result?");
    argmax as f32
}

fn run_correlation(shift: f32) -> f32 {
    let length = 256;
    let zeroes = (0..length).map(|_| 0.0).collect::<Vec<_>>();
    let sample1 = (0..length)
        .map(|i| f(i as f32))
        .chain(zeroes.clone().into_iter())
        .collect::<Vec<_>>();

    // println!("{:?}", sample1);
    let sample2 = (0..length)
        .map(|i| f(i as f32 + shift))
        .chain(zeroes.into_iter())
        .collect::<Vec<_>>();

    let mut planner = RealFftPlanner::<f32>::new();
    let r2c = planner.plan_fft_forward(length * 2);
    let c2r = planner.plan_fft_inverse(length * 2);

    let mut indata = sample1.clone();

    let mut spectrum1 = r2c.make_output_vec();
    r2c.process(&mut indata, &mut spectrum1).unwrap();

    let mut spectrum2 = r2c.make_output_vec();
    indata = sample2.clone();
    r2c.process(&mut indata, &mut spectrum2).unwrap();

    let mut spectrum_result = spectrum1
        .iter()
        .zip(spectrum2.iter())
        .map(|(c1, c2)| c1.conj() * c2)
        .collect::<Vec<_>>();

    let mut correlation = c2r.make_output_vec();
    c2r.process(&mut spectrum_result, &mut correlation).unwrap();

    // println!("{:?}", spectrum1);
    // println!("{:?}", spectrum2);

    let (argmax, _max) = correlation
        .iter()
        .enumerate()
        .max_by(|&(_, first), &(_, second)| first.partial_cmp(second).unwrap())
        .map(|(index, value)| (index, value))
        .expect("No values in result?");
    // println!("Selfmade:  {}, {}", shift, argmax);
    512.0 - argmax as f32
}

fn main() {
    // basic_dsp_impl::gcc_with_basic_dsp();
    // return;
    for shift in 0..256 {
        let dumb = basic_dsp_impl::gcc_with_basic_dsp(shift as f32);
        let ours = run_correlation(shift as f32);
        let basic = basic_dsp_impl::gcc_with_basic_dsp(shift as f32);
        println!("{}: {}, {}, {}", shift, dumb, ours, basic);
    }
}
