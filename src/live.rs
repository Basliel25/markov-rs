//! Sliding-window live tracker for first-order Markov chain divergence.

use crate::baseline::TransitionMatrix;
use crate::kl;
use std::collections::{HashMap, VecDeque};

/// Tracks recent transitions in a sliding window and scores per-state
/// divergence against a baseline.
///
/// The window holds the most recent `window_size` transitions. Older
/// transitions are evicted FIFO as new ones arrive.
pub struct LiveTracker<'a> {
    baseline: &'a TransitionMatrix,
    window_size: usize,
    alpha: f64, 
    min_observations: usize,

    /// FIFO queue of (from, to) transitions.
    window: VecDeque<(u64, u64)>,

    // Number of items appearing in window
    counts: HashMap<u64, HashMap<u64, u64>>
}

impl LiveTracker {
    pub fn new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baseline::Baseline;

    const EPS: f64 = 1e-9;
}
