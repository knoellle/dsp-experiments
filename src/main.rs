use std::{fs::File, path::Path};

use realfft::RealFftPlanner;

mod basic_dsp_impl;
mod tools;

use tools::*;

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

fn angles_from_delay(delay: f32, microphone_distance: f32) -> Option<f32> {
    let speed_of_sound = 343.0;
    let sample_rate = 44100.0;

    let maximum_delay = microphone_distance / speed_of_sound * sample_rate;
    println!("Delay: {} Maximum delay: {}", delay, maximum_delay);
    let ratio = delay / maximum_delay;
    if ratio < -1.2 || ratio > 1.2 {
        return None;
    }
    let angle = ratio.clamp(-1.0, 1.0).acos();

    Some(angle)
}

fn run_correlation(sample1: &Vec<f32>, sample2: &Vec<f32>) -> f32 {
    let mut planner = RealFftPlanner::<f32>::new();
    let r2c = planner.plan_fft_forward(sample1.len());
    let c2r = planner.plan_fft_inverse(sample2.len());

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
        .map(|c| c / c.norm())
        .collect::<Vec<_>>();

    let mut correlation = c2r.make_output_vec();
    c2r.process(&mut spectrum_result, &mut correlation).unwrap();

    plot(&correlation).unwrap();

    let (argmax, _max) = correlation
        .iter()
        .enumerate()
        .max_by(|&(_, first), &(_, second)| first.partial_cmp(second).unwrap())
        .map(|(index, value)| (index as f32, value))
        .expect("No values in result?");
    let argmax = if argmax > sample1.len() as f32 / 2.0 {
        -(sample1.len() as f32) + argmax
    } else {
        argmax
    };
    // println!("Selfmade: {} / {}", argmax, sample1.len());
    argmax
}

fn get_angle_from_file(file: &Path) -> Option<f32> {
    let mut file = File::open(file).unwrap();
    let (_header, data) = wav::read(&mut file).unwrap();
    let mut samples = vec![vec![]; 4];

    for wav_samples in data.as_eight().unwrap().chunks_exact(4) {
        samples[0].push(wav_samples[0] as f32 / 256.0 - 0.5);
        samples[1].push(wav_samples[1] as f32 / 256.0 - 0.5);
        samples[2].push(wav_samples[2] as f32 / 256.0 - 0.5);
        samples[3].push(wav_samples[3] as f32 / 256.0 - 0.5);
    }
    let delay = run_correlation(&samples[0], &samples[1]);
    let angle = angles_from_delay(delay, 0.116)?.to_degrees();
    println!("Delay: {}, angle: {}", delay, angle);
    let delay = run_correlation(&samples[0], &samples[2]);
    let angle2 = angles_from_delay(delay, 0.0533)?.to_degrees();
    println!("Delay: {}, angle: {}", delay, angle2);

    let forward = angle2 > 90.0;

    let angle = if forward { -angle } else { angle } + 90.0;

    Some(angle % 360.0)
}

fn main() {
    let angle = get_angle_from_file(Path::new("left_behind_45.wav")).unwrap();
    println!("Final angle: {}", angle);
    // basic_dsp_impl::gcc_with_basic_dsp(52.0);
    // return;
    // for shift in 0..256 {
    //     let dumb = basic_dsp_impl::gcc_with_basic_dsp(shift as f32);
    //     let ours = run_correlation(shift as f32);
    //     let basic = basic_dsp_impl::gcc_with_basic_dsp(shift as f32);
    //     println!("{}: {}, {}, {}", shift, dumb, ours, basic);
    // }
}
