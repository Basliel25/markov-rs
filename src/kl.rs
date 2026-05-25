//! KL divergence and Laplace smoothing utilities.

/// Laplace add-α smoothing. Converts raw counts to normalized probabilities.
///
/// # Panics
/// Panics if `counts.len() != out.len()`

pub fn laplace_normalize(counts: &[u64], alpha: f64, out: &mut [f64]) {

    debug_assert_eq!(counts.len(), out.len());

    /// We use laplace smoothing to smooth out zero probabilities
    /// for events with count 0.
    ///
    /// The fix is to add alpha extra observations
    /// for every possible outcome.
    ///
    ///         c[i] + α
    // p[i] = ----------
    //         N + α * k
    // where N = total observed counts
    // k = number of outcomes
    // α = smoothing param.
    //
    // α = 1.0 for this case
    // Alpha can be played with, the higher the alpha 
    // the more the data is pulled towards uniformity
    let n = counts.len() as f64;
    let total: f64 = counts.iter()
        .sum::<u64>() as f64;

    let denominator = total + alpha * n; // Normalizing constant

    for (o, &c) in out.
        iter_mut()
            .zip(counts.iter()) {
                *o = (c as f64) / denominator;
            }
}

/// Kullback-Leibler divergence: KL(p || q).
///
/// # Preconditions
/// - `p.len() == q.len()`
/// - `q[i] > 0.0` for all i 
pub fn kl_divergence(p: &[f64], q: &[f64]) -> f64 {
    // KL divergence measures how much information is lost when 
    // distribution Q is used  
    // to approximate the true distribution P.
    //
    //   KL(P || Q) = Σ p[i] * log(p[i] / q[i])
    debug_assert_eq!(p.len(), q.len());

    p.iter()
    .zip(q.iter())
    .filter(|(&pi, _)| pi > 0.0) // drop 0*log(0/q) terms which  is 0 by convention
    .map(|(&pi, &qi)| pi * (pi / qi).ln())
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kl_zero_on_identical_uniform() {
        // TODO
    }

    #[test]
    fn test_kl_zero_on_identical_skewed() {
        // TODO
    }

    #[test]
    fn test_kl_known_value() {
        // TODO: hand-computed value
    }

    #[test]
    fn test_kl_asymmetry() {
        // TODO
    }

    #[test]
    fn test_laplace_edge_case() {
        // TODO
    }
}
