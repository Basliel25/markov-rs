//! Baseline transition learner for first-order Markov chains.

use std::collections::{HashMap, HashSet};

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
    pub fn observe(&mut self, from: u64, to: u64){
        *self.counts
            .entry(from)
            .or_default()
            .entry(to)
            .or_insert(0) += 1;
    }

    /// Produce a frozen transition matrix from accumlated count
    pub fn finalize(&self) -> TransitionMatrix {
        // Build columns
        // Every transition id becomes now a column in the matrix
        // Collect to a hash set and apply determnisntic ordering
        let mut column_set: HashSet<u64> = HashSet::new();
        for inner in self.counts.values() {
            column_set.extend(inner.keys().copied());
        }
        let mut column_idx: Vec<u64> = column_set.into_iter().collect();
        column_idx.sort_unstable();

        // for each observed `from`, build a dense count vector
        // aligned to the column space then Laplace-smooth it into a
        // probability row.

        let k = column_idx.len();
        let mut rows: HashMap<u64, Vec<f64>> = HashMap::new();

        for (&from, inner) in &self.counts {
            let counts: Vec<u64> = column_idx
                .iter()
                .map(|to| inner.get(to).copied().unwrap_or(0))
                .collect();

            let mut probs = vec![0.0; k];
            crate::kl::laplace_normalize(&counts, self.alpha, &mut probs);

            rows.insert(from, probs);
        }

        TransitionMatrix { column_idx, rows }
    }
}

/// A frozen, row-stochastic transition matrix with Laplace smoothing applied.
pub struct TransitionMatrix {
    column_idx: Vec<u64>,
    rows: HashMap<u64, Vec<f64>>,
}

impl TransitionMatrix {
    /// Smoothed transition row for `from`. None if `from` was never observed.
    pub fn row(&self, from: u64) -> Option<&[f64]> {
        self.rows.get(&from).map(|v| v.as_slice())
    }

    /// Sorted `to` template IDs in column order
    pub fn columns(&self) -> &[u64] {
        &self.column_idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-9;

    #[test]
    fn unobserved_from_returns_none() {
        // A `from` ID that never appeared in training has no row.
        let b = Baseline::new(1.0);
        let m = b.finalize();
        assert!(m.row(42).is_none());
    }

    #[test]
    fn single_transition_smooths_correctly() {
        // observe (1, 2) once, finalize, check row(1)
        let mut b = Baseline::new(1.0);
        b.observe(1, 2);
        let m = b.finalize();
        let row = m.row(1).expect("row(1) should exist");
        assert_eq!(row.len(), 1);
        assert!((row[0] - 1.0).abs() < EPSILON)
    }
    #[test]
    fn rows_sum_to_one() {
        // sanity: any returned row sums to 1.0 (within EPS)
        // TODO
    }

    #[test]
    fn continues_after_finalize() {
        // finalize, observe more, finalize again, verify changes reflected
        // TODO
    }

    #[test]
    fn columns_are_sorted_and_unique() {
        // observe several transitions with overlapping `to`s, check columns() output
        // TODO
    }
}
