//! Sliding-window live tracker for first-order Markov chain divergence.

use crate::baseline::TransitionMatrix;
use crate::kl;
use std::collections::{HashMap, VecDeque};

pub struct LiveTracker<'a> {}

impl LiveTracker {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baseline::Baseline;

    const EPS: f64 = 1e-9;
}
