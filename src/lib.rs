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

fn compute_diff_min(diff_fn: &[f64], max_tau: usize, harm_threshold: f64) -> usize {
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
fn compute_sample_frequency(audio_sample: Vec<f64>, tau_max: usize) -> f64 {
    let diff_fn = diff_function(&audio_sample, tau_max);
    let cmndf = cmndf(&diff_fn);
    let sample_period = compute_diff_min(&cmndf, tau_max, 0.15);
    convert_to_frequency(sample_period, audio_sample.len())
}

#[cfg(test)]
mod tests {
    use dasp::{signal, Signal};
    fn produce_sample(sample_rate: usize, frequency: f64) -> Vec<f64> {}
    use super::*;
    #[test]
    fn sanity_basic_sine() {
        let sample = produce_sample(12, 4.0);
        let computed_frequency = compute_sample_frequency(sample, 6);
        assert_eq!(computed_frequency, 4.0);
    }
}
