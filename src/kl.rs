//! KL divergence and Laplace smoothing utilities.

/// Laplace add-α smoothing. Converts raw counts to normalized probabilities.
///
/// # Panics
/// Panics if `counts.len() != out.len()`

pub fn laplace_normalize(counts: &[u64], alpha: f64, out: &mut [f64]) {
    debug_assert_eq!(counts.len(), out.len);
    todo!()
}

/// Kullback-Leibler divergence: KL(p || q).
///
/// # Preconditions
/// - `p.len() == q.len()`
/// - `q[i] > 0.0` for all i 
pub fn kl_divergence(p: &[f64], q: &[f64]) -> f64 {
    debug_assert_eq!(p.len(), q.len());
    todo!()
}
