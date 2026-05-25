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
    pub fn observe(&mut self, from: u64, to: u64) {
        // Filter: drop if either ID is outside baslien columns
        if self.baseline.row(from).is_none() {
            return;
        }
        if !self.baseline.columns().contains(&to) {
            return;
        }

        // Push to the back of the window and increment the mirrored count.
        self.window.push_back((from, to));
        *self.counts
            .entry(from)
            .or_default()
            .entry(to)
            .or_insert(0) += 1;

        // Evicting from the front if over capacity
        // Mirror the decrement so
        // `counts` stays in sync with `window`. 
        if self.window.len() > self.window_size {
            let (old_from, old_to) = self.window.pop_front()
                .expect("window non-empty: just checked len > window_size");

            if let Some(inner) = self.counts.get_mut(&old_from) {
                if let Some(c) = inner.get_mut(&old_to) {
                    *c -= 1;
                    if *c == 0 {
                        inner.remove(&old_to);
                    }
                }
                if inner.is_empty() {
                    self.counts.remove(&old_from);
                }
            }
        }
    }
    // KL divergence between live and baseline rows for `from`.
    ///
    /// Returns `None` if:
    /// - `from` is not in the baseline 
    /// - fewer than `min_observations` of `from` exist in the current window
    ///   (insufficient signal)
    pub fn divergence_for(&self, from: u64) -> Option<f64> {
        // Baseline must have a row for `from`
        let basline_row = self.baseline.row(from)?;

        // Have minimum distributions,
        // under this threshold the distribution is
        // just laplace prior instead of actual data.
        // Explicit none!!!!
        let inner = self.counts.get(&from)?;
        let total: u64 = inner.values().sum();
        if (total as usize) < self.min_observations {
            return None;
        }

        let columns = self.baseline.columns();
        let counts: Vec<u64> = columns
            .iter()
            .map(|to| inner.get(to).copied().unwrap_or(0))
            .collect();

        // Smooth the live counts into a probability row using the same alpha
    // as the baseline.


    }

    // The `to` ID that contributed most to the divergence for `from`.
    /// Returns `(to_id, term_value)` which is the per-column KL contribution.
    pub fn dominant_transition(&self, from: u64) -> Option<(u64, f64)> {todo!()}

    // GETTERS
    pub fn window_len(&self) -> usize {
        self.window.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baseline::Baseline;

    const EPS: f64 = 1e-9;

    // Helper function for running modular tests
    fn make_baseline() -> TransitionMatrix {
        let mut b = Baseline::new(1.0);
        // Establish column space {2, 3} and from {1}.
        b.observe(1, 2);
        b.observe(1, 2);
        b.observe(1, 3);
        b.finalize()
    }

    #[test]
    fn window_evicts_fifo() {
        let baseline = make_baseline();
        let mut t = LiveTracker::new(&baseline, 3, 1.0, 1);

        t.observe(1, 2);
        t.observe(1, 2);
        t.observe(1, 3);
        assert_eq!(t.window_len(), 3);

        t.observe(1, 2); // should evict the first (1, 2)
        assert_eq!(t.window_len(), 3);
    }

    #[test]
    fn unknown_from_dropped() {
        let baseline = make_baseline();
        let mut t = LiveTracker::new(&baseline, 10, 1.0, 1);
        t.observe(99, 2); // 99 not a baseline `from`
        assert_eq!(t.window_len(), 0);
    }

    #[test]
    fn unknown_to_dropped() {
        let baseline = make_baseline();
        let mut t = LiveTracker::new(&baseline, 10, 1.0, 1);
        t.observe(1, 999); // 999 not in baseline columns
        assert_eq!(t.window_len(), 0);
    }
}
