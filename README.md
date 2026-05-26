# markov-rs

[![Crates.io](https://img.shields.io/crates/v/markov-rs.svg)][https://crates.io/crates/markov-rs](https://crates.io/crates/markovTrans-rs)
[![License](https://img.shields.io/crates/l/markov-rs.svg)](https://github.com/Basliel25/markov-rs#license)
First-order Markov chain library with KL divergence analysis.

- Build discrete Markov chains from event sequences.
- Compute baseline transition distributions
- Detect anomalies via Kullback-Leibler divergence on observed vs. expected behavior.

## Usage

```rust
use markov_rs::baseline::Baseline;
use markov_rs::live::LiveTracker;

// Train baseline on event sequence
let mut baseline = Baseline::new(1.0);
baseline.observe(1, 2);
baseline.observe(1, 2);
baseline.observe(1, 3);
let matrix = baseline.finalize();

// Score live behavior in a sliding window
let mut tracker = LiveTracker::new(&matrix, 100, 1.0, 10);
for _ in 0..30 { tracker.observe(1, 3); }  // unusual behavior

if let Some(score) = tracker.divergence_for(1) {
    println!("divergence: {}", score);
}
```
