//! KL divergence and Laplace smoothing utilities.

/// Laplace add-α smoothing. Converts raw counts to normalized probabilities.
///
/// # Panics
/// Panics if `counts.len() != out.len()`

pub fn laplace_normalize(counts: &[u64], alpha: f64, out: &mut [f64]) {

    debug_assert_eq!(counts.len(), out.len());

    // We use laplace smoothing to smooth out zero probabilities
    // for events with count 0.
    //
    // The fix is to add alpha extra observations
    // for every possible outcome.
    //
    //         c[i] + α
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
                *o = (c as f64 + alpha) / denominator;
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

    p.iter().copied()
    .zip(q.iter().copied())
    .filter(|(pi, _)| *pi > 0.0) // drop 0*log(0/q) terms which  is 0 by convention
    .map(|(pi, qi)| pi * (pi / qi).ln())
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-9;

    #[test]
    fn test_kl_zero_on_identical_uniform() {
        // KL(P || P) = Σ p[i] * log(1) = 0 for any P.
        let p = [1.0 / 3.0; 3];
        assert!((kl_divergence(&p, &p)).abs() < EPSILON);
  }

    #[test]
    fn test_kl_zero_on_identical_skewed() {
        // Same identity check on a non-uniform distribution
        let p = [0.7, 0.2, 0.1];
        assert!((kl_divergence(&p, &p)).abs() < EPSILON);
    }

    #[test]
    fn test_kl_known_value() {
        // P = [0.7, 0.2, 0.1], Q = uniform [1/3, 1/3, 1/3]
        // Expected = 0.7*ln(0.7/0.333..) + 0.2*ln(0.2/0.333..) + 0.1*ln(0.1/0.333..)
        let p = [0.7, 0.2, 0.1];
        let q = [1.0 / 3.0; 3];
        let expected: f64 = p.iter().copied()
            .zip(q.iter().copied()).map(|(pi, qi): (f64, f64)| pi * (pi / qi).ln()).sum();
        let got = kl_divergence(&p, &q);
        assert!((got - expected).abs() < EPSILON);
    }

    #[test]
    fn test_kl_asymmetry() {
        // KL divergence is assymetric fundamentally
        let p: [f64; 3] = [0.7, 0.2, 0.1];
        let q: [f64; 3] = [0.5, 0.3, 0.2];
        assert!((kl_divergence(&p, &q) - kl_divergence(&q, &p)).abs() > EPSILON);
    }

    #[test]
    fn test_laplace_edge_case() {
        // counts = [10, 0, 0], α = 1.0
        // N = 10, k = 3, denom = 10 + 1*3 = 13
        // p = [11/13, 1/13, 1/13]
        let counts = [10u64, 0, 0];
        let mut out = [0.0f64; 3];
        laplace_normalize(&counts, 1.0, &mut out);
        assert!((out[0] - 11.0 / 13.0).abs() < EPSILON);
        assert!((out[1] - 1.0 / 13.0).abs() < EPSILON);
        assert!((out[2] - 1.0 / 13.0).abs() < EPSILON);
    }
}
