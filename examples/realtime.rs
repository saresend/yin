use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::*;
use yin::*;

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("failed to find input device");
    let config = device.default_input_config().unwrap();
    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
    }
}

fn run<T: Sample>(device: &Device, config: &StreamConfig) {
    let yin = yin::Yin::init(0.01, 10.0, 100.0, 44100);
    let err_fn = |err| println!("{}", err);
    let stream = device
        .build_input_stream(
            &config,
            move |data, _| write_input_data::<T>(data, &yin),
            err_fn,
        )
        .unwrap();

    stream.play().unwrap();
    loop {}
}

fn write_input_data<T: Sample>(input: &[T], yin: &yin::Yin) {
    let f64_vals: Vec<f64> = input.iter().map(|x| x.to_f32() as f64).collect();
    let estimate = yin.estimate_freq(&f64_vals).unwrap_or(-1.0);
    println!("Estimated Frequency: {}", estimate);
}
