use criterion::{black_box, criterion_group, criterion_main, Criterion};
use yin::Yin;

pub fn norm_sine_benchmark(c: &mut Criterion) {
    let sample = {
        use dasp::{signal, Signal};
        let mut signal = signal::rate(44100.0).const_hz(44100.0).sine();
        let sample: Vec<f64> = (0..44100).map(|_| signal.next()).collect();
        sample
    };
    let yin = Yin::init(0.1, 40.0, 200.0, 1000);
    c.bench_function("1000 sr, 100.0 freq", |b| {
        b.iter(|| yin.estimate_freq(&sample))
    });
}

criterion_group!(benches, norm_sine_benchmark);
criterion_main!(benches);
