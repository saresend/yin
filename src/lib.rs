pub struct Yin {
    threshold: f64,
    tau_max: usize,
    tau_min: usize,
    sample_rate: usize,
}

impl Yin {
    pub fn init(threshold: f64, freq_min: f64, freq_max: f64, sample_rate: usize) -> Yin {
        let tau_max = sample_rate / freq_min as usize;
        let tau_min = sample_rate / freq_max as usize;
        Yin {
            threshold,
            tau_max,
            tau_min,
            sample_rate,
        }
    }

    pub fn estimate_freq(&self, audio_sample: &[f64]) -> f64 {
        let sample_frequency = compute_sample_frequency(
            audio_sample,
            self.tau_min,
            self.tau_max,
            self.sample_rate,
            self.threshold,
        );
        sample_frequency
    }
}

fn diff_function(audio_sample: &[f64], tau_max: usize) -> Vec<f64> {
    let mut diff_function = vec![0.0; tau_max];
    for tau in 1..tau_max {
        for j in 0..(audio_sample.len() - tau_max) {
            let tmp = audio_sample[j] - audio_sample[j + tau];
            diff_function[tau] += tmp * tmp;
        }
    }
    diff_function
}

fn cmndf(raw_diff: &[f64]) -> Vec<f64> {
    let mut cmndf_diff: Vec<f64> = raw_diff[1..]
        .iter()
        .enumerate()
        .scan(0.0, |state, x| {
            *state = *state + x.1;
            let result = *x.1 * (x.0 + 1) as f64 / *state;
            Some(result)
        })
        .collect();
    cmndf_diff.insert(0, 1.0);
    cmndf_diff
}

fn compute_diff_min(diff_fn: &[f64], min_tau: usize, max_tau: usize, harm_threshold: f64) -> usize {
    let mut tau = 1;
    while tau < max_tau {
        if diff_fn[tau] < harm_threshold {
            while tau + 1 < max_tau && diff_fn[tau + 1] < diff_fn[tau] {
                tau += 1;
            }
            return tau;
        }
        tau += 1;
    }
    0
}

fn convert_to_frequency(sample_period: usize, sample_rate: usize) -> f64 {
    let value: f64 = sample_period as f64 / sample_rate as f64;
    1.0 / value
}

// should return a tau that gives the # of elements of offset in a given sample
pub fn compute_sample_frequency(
    audio_sample: &[f64],
    tau_min: usize,
    tau_max: usize,
    sample_rate: usize,
    threshold: f64,
) -> f64 {
    let diff_fn = diff_function(&audio_sample, tau_max);
    let cmndf = cmndf(&diff_fn);
    let sample_period = compute_diff_min(&cmndf, tau_min, tau_max, threshold);
    convert_to_frequency(sample_period, sample_rate)
}

#[cfg(test)]
mod tests {
    use dasp::{signal, Signal};
    fn produce_sample(sample_rate: usize, frequency: f64, noise_ratio: f64) -> Vec<f64> {
        use rand::prelude::*;
        let mut rng = thread_rng();
        let mut signal = signal::rate(sample_rate as f64).const_hz(frequency).sine();
        let sample: Vec<f64> = (0..sample_rate)
            .map(|_| signal.next() + noise_ratio * rng.gen::<f64>())
            .collect();
        sample
    }
    use super::*;
    #[test]
    fn sanity_basic_sine() {
        let sample = produce_sample(12, 4.0, 0.0);
        let yin = Yin::init(0.1, 2.0, 5.0, 12);
        let computed_frequency = yin.estimate_freq(&sample);
        assert_eq!(computed_frequency, 4.0);
    }

    #[test]
    fn sanity_non_multiple() {
        let sample = produce_sample(44100, 4000.0, 0.0);
        let yin = Yin::init(0.1, 3000.0, 5000.0, 44100);
        let computed_frequency = yin.estimate_freq(&sample);
        let difference = computed_frequency - 4000.0;
        assert!(difference.abs() < 50.0);
    }

    #[test]
    fn sanity_full_sine() {
        let sample = produce_sample(44100, 441.0, 0.0);
        let yin = Yin::init(0.1, 300.0, 500.0, 44100);
        let computed_frequency = yin.estimate_freq(&sample);
        assert_eq!(computed_frequency, 441.0);
    }
}
