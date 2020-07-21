# Yin: Frequency Estimation For Rust

Yin provides a function for estimating the fundamental frequency of a signal, based on the following article:

[1] De Cheveigné, A., & Kawahara, H. (2002). YIN, a fundamental frequency estimator for speech and music. The Journal of the Acoustical Society of America, 111(4), 1917-1930.

[Link to Paper](http://audition.ens.fr/adc/pdf/2002_JASA_YIN.pdf)

# Installation

## TODO: Put on crates.io

# Usage

```rust
use yin::compute_sample_frequency;

let example_audio_sample = [0.0, 1.0, 0.0, -1.0];
let frequency = compute_sample_frequency(&example_audio_sample, 2);

assert_eq!(frequency, 1.0);
```
