// src/gating.rs
//! Deterministic gating for Mixture-of-Experts routing.
//! Simple centroid dot-product router: choose the expert with highest score.
//!
//! Usage:
//! - Provide an embedding (Vec<f64>) and a set of centroids (Vec<Vec<f64>>).
//! - Call `route_embedding` to get the chosen expert index and scores.
//!
//! This file is intentionally small and dependency-free so you can add it from a phone.

#[derive(Debug)]
pub struct GateResult {
    /// Index of chosen expert (0..centroids.len()-1)
    pub expert: usize,
    /// Raw scores for each centroid (same order as centroids)
    pub scores: Vec<f64>,
}

/// Compute dot product between two same-length vectors.
fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// L2-normalize a vector in place. If norm is zero, leaves vector unchanged.
fn normalize_inplace(v: &mut [f64]) {
    let norm_sq: f64 = v.iter().map(|x| x * x).sum();
    if norm_sq <= 0.0 {
        return;
    }
    let norm = norm_sq.sqrt();
    for x in v.iter_mut() {
        *x /= norm;
    }
}

/// Route an embedding to an expert index using centroid dot products.
/// - `embedding`: input vector (length d)
/// - `centroids`: list of centroid vectors (each length d)
/// - `normalize`: if true, L2-normalize embedding and centroids before scoring
/// Returns `GateResult` with chosen expert and raw scores.
///
/// Notes:
/// - If centroids is empty, returns expert = 0 and empty scores.
/// - If dimensions mismatch, behavior is to treat missing entries as 0.0 (safe on phone).
pub fn route_embedding(
    embedding: &[f64],
    centroids: &[Vec<f64>],
    normalize: bool,
) -> GateResult {
    if centroids.is_empty() {
        return GateResult {
            expert: 0,
            scores: vec![],
        };
    }

    // Prepare a normalized copy of embedding if requested
    let mut emb = embedding.to_vec();
    if normalize {
        normalize_inplace(&mut emb);
    }

    let mut scores = Vec::with_capacity(centroids.len());
    for c in centroids.iter() {
        // If normalize, use normalized centroid copy
        if normalize {
            let mut ccopy = c.clone();
            normalize_inplace(&mut ccopy);
            scores.push(dot(&emb, &ccopy));
        } else {
            // handle dimension mismatch by dotting up to min length
            let min_len = std::cmp::min(emb.len(), c.len());
            let s = emb[..min_len].iter().zip(c[..min_len].iter()).map(|(x,y)| x*y).sum();
            scores.push(s);
        }
    }

    // Choose argmax
    let mut best_idx = 0usize;
    let mut best_score = scores[0];
    for (i, &s) in scores.iter().enumerate().skip(1) {
        if s > best_score {
            best_score = s;
            best_idx = i;
        }
    }

    GateResult {
        expert: best_idx,
        scores,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_basic() {
        // simple 3-d embedding and 3 centroids
        let emb = vec![1.0, 0.0, 0.0];
        let centroids = vec![
            vec![1.0, 0.0, 0.0], // best match
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        let res = route_embedding(&emb, &centroids, true);
        assert_eq!(res.expert, 0);
        assert_eq!(res.scores.len(), 3);
    }

    #[test]
    fn route_with_mismatch_dims() {
        let emb = vec![1.0, 2.0];
        let centroids = vec![
            vec![1.0, 0.0, 0.0], // dot = 1
            vec![0.0, 1.0, 0.0], // dot = 2
        ];
        let res = route_embedding(&emb, &centroids, false);
        assert_eq!(res.expert, 1);
    }

    #[test]
    fn route_empty_centroids() {
        let emb = vec![0.1, 0.2];
        let centroids: Vec<Vec<f64>> = vec![];
        let res = route_embedding(&emb, &centroids, true);
        assert_eq!(res.expert, 0);
        assert!(res.scores.is_empty());
    }
}