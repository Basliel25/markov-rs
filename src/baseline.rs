//! Baseline transition learner for first-order Markov chains.

use std::collections::HashMap;

/// Accumulates observed (from, to) transition counts.
/// Alpha is the laplace smooting parameter
pub struct Baseline {
    alpha: f64,
    counts: HashMap<u64, HashMap<u64, u64>>,
}

impl Baseline {

    /// Create a new baseline learner
    pub fn new(alpha: f64) -> Self {
        Self {
            alpha,
            counts: HashMap::new(),
        }
    }

    /// Record a transition
    pub fn observe(&mut self, from: u64, to: u64){todo!()}

    /// Produce a frozen transition matrix from accumlated count
    pub fn finalize(&self) -> TransitionMatrix {todo!()}

}

