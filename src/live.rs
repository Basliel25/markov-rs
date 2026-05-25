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

impl<'a> LiveTracker<'a> {
    pub fn new(
        baseline: &'a TransitionMatrix,
        window_size: usize,
        alpha: f64,
        min_observations: usize,
    ) -> Self {

        /// Create a new live tracker binding to baseline
        Self {
            baseline,
            window_size,
            alpha,
            min_observations,
            window: VecDeque::with_capacity(window_size),
            counts: HashMap::new(),
        }
    }

    /// Record a single observed transition.
    pub fn observe(&mut self, from: u64, to: u64) {todo!()}

    // KL divergence between live and baseline rows for `from`.
    ///
    /// Returns `None` if:
    /// - `from` is not in the baseline 
    /// - fewer than `min_observations` of `from` exist in the current window
    ///   (insufficient signal)
    pub fn divergence_for(&self, from: u64) -> Option<f64> {todo!()}

    // The `to` ID that contributed most to the divergence for `from`.
    /// Returns `(to_id, term_value)` which is the per-column KL contribution.
    pub fn dominant_transition(&self, from: u64) -> Option<(u64, f64)> {todo!()}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baseline::Baseline;

    const EPS: f64 = 1e-9;
}
