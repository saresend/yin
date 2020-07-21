# Yin: Frequency Estimation For Rust

Yin provides a function for estimating the fundamental frequency of a signal, based on the following article:

[1] De Cheveigné, A., & Kawahara, H. (2002). YIN, a fundamental frequency estimator for speech and music. The Journal of the Acoustical Society of America, 111(4), 1917-1930.

[Link to Paper](http://audition.ens.fr/adc/pdf/2002_JASA_YIN.pdf)

# Installation

Add the following to your `Cargo.toml`

```
yin = "0.1.0"
```

# Usage

```rust
        // Configure params for our estimator, see docs for details
        let estimator = Yin::init(0.1, 10.0, 30.0, 80);
        let mut example = vec![];
        let mut prev_value = -1.0;
        // Periodic over every 4 values of i, giving us a frequency of: 80 / 4 == 20
        for i in 0..80 {
            if i % 2 != 0 {
                example.push(0.0);
            } else {
                prev_value *= -1.0;
                example.push(prev_value);
            }
        }
        let freq = estimator.estimate_freq(&example);
        assert_eq!(freq, 20.0);
```

### Benchmarks

At the time of writing I have not taken too much time to optimize this implementation. That said it is still quite snappy, especially if you can provide a reasonably small frequency range to search over:

```
44100hz sample rate, 100hz frequency, 160hz search range: [ 2.4157 ms 2.4244 ms 2.4330 ms ]
```

Benchmarks are done using criterion, and can be run by cloning the repository and using `cargo bench`
