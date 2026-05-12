// SUMMARY: Statistical validation module computing Shannon Entropy of clock skews.
use std::collections::HashMap;

/// Calculates the Shannon Entropy of the captured delta values to validate uniqueness.
/// Formula: H(X) = -sum p(x_i) log(p(x_i))
pub fn calculate_entropy(deltas: &[f64]) -> f64 {
    if deltas.is_empty() {
        return 0.0;
    }

    let mut frequencies: HashMap<i64, usize> = HashMap::new();
    let total = deltas.len() as f64;

    for &delta in deltas {
        let bin = (delta * 10_000.0).round() as i64;
        *frequencies.entry(bin).or_insert(0) += 1;
    }

    let mut entropy = 0.0;
    for &count in frequencies.values() {
        let p = count as f64 / total;
        if p > 0.0 {
            entropy -= p * p.log2();
        }
    }

    entropy
}

/// Computes a normalized confidence score [0.0, 1.0] from entropy
pub fn confidence_score(deltas: &[f64]) -> f64 {
    if deltas.is_empty() {
        return 0.0;
    }
    let entropy = calculate_entropy(deltas);
    let max_entropy = (deltas.len() as f64).log2();
    if max_entropy == 0.0 {
        return 1.0; 
    }
    (entropy / max_entropy).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_uniform() {
        let deltas = vec![1.0, 2.0, 3.0, 4.0];
        let entropy = calculate_entropy(&deltas);
        assert!((entropy - 2.0).abs() < 1e-6); // log2(4) = 2.0
    }

    #[test]
    fn test_entropy_identical() {
        let deltas = vec![1.0, 1.0, 1.0, 1.0];
        let entropy = calculate_entropy(&deltas);
        assert!((entropy - 0.0).abs() < 1e-6);
    }
}