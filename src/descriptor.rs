// src/descriptor.rs
//! Small, dependency-light descriptor extractor for stroke privacy and recognition.
//! Input: Vec<(f64,f64)> stroke (ordered points).
//! Output: JSON-serializable descriptor with normalized length, curvature histogram,
//! and first N Fourier magnitude coefficients.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Descriptor {
    pub norm_length: f64,
    pub curvature_hist: Vec<f64>,
    pub fourier_mag: Vec<f64>,
}

fn euclid(a: (f64, f64), b: (f64, f64)) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    (dx * dx + dy * dy).sqrt()
}

fn resample_to_n(points: &[(f64, f64)], n: usize) -> Vec<(f64, f64)> {
    if points.len() <= 1 || n < 2 {
        return points.to_vec();
    }
    // compute cumulative arc length
    let mut dists = vec![0.0f64];
    for i in 1..points.len() {
        dists.push(dists[i - 1] + euclid(points[i], points[i - 1]));
    }
    let total = *dists.last().unwrap();
    if total == 0.0 {
        return vec![points[0]; n];
    }
    let step = total / ((n - 1) as f64);
    let mut res = Vec::with_capacity(n);
    let mut target = 0.0;
    let mut j = 0usize;
    for _ in 0..n {
        while j + 1 < dists.len() && dists[j + 1] < target {
            j += 1;
        }
        if j + 1 == dists.len() {
            res.push(points.last().cloned().unwrap());
        } else {
            let t0 = dists[j];
            let t1 = dists[j + 1];
            let p0 = points[j];
            let p1 = points[j + 1];
            let alpha = if (t1 - t0).abs() < 1e-12 { 0.0 } else { (target - t0) / (t1 - t0) };
            let x = p0.0 + alpha * (p1.0 - p0.0);
            let y = p0.1 + alpha * (p1.1 - p0.1);
            res.push((x, y));
        }
        target += step;
    }
    res
}

fn compute_curvatures(points: &[(f64, f64)]) -> Vec<f64> {
    let n = points.len();
    if n < 3 {
        return vec![0.0; n];
    }
    let mut curv = Vec::with_capacity(n);
    for i in 0..n {
        let prev = points[(i + n - 1) % n];
        let cur = points[i];
        let next = points[(i + 1) % n];
        let v1 = (cur.0 - prev.0, cur.1 - prev.1);
        let v2 = (next.0 - cur.0, next.1 - cur.1);
        let a = (v1.0 * v1.0 + v1.1 * v1.1).sqrt();
        let b = (v2.0 * v2.0 + v2.1 * v2.1).sqrt();
        if a == 0.0 || b == 0.0 {
            curv.push(0.0);
            continue;
        }
        let dot = v1.0 * v2.0 + v1.1 * v2.1;
        let cos_theta = (dot / (a * b)).clamp(-1.0, 1.0);
        let theta = cos_theta.acos();
        curv.push(theta);
    }
    curv
}

fn histogram(values: &[f64], bins: usize, max_angle: f64) -> Vec<f64> {
    let mut hist = vec![0.0f64; bins];
    let bin_w = max_angle / (bins as f64);
    if bin_w == 0.0 {
        return hist;
    }
    for &v in values {
        let idx = ((v / bin_w).floor() as isize).clamp(0, (bins - 1) as isize) as usize;
        hist[idx] += 1.0;
    }
    let sum: f64 = hist.iter().sum();
    if sum > 0.0 {
        for h in hist.iter_mut() {
            *h /= sum;
        }
    }
    hist
}

fn dft_magnitudes(points: &[(f64, f64)], k: usize) -> Vec<f64> {
    let n = points.len();
    let mut mags = Vec::with_capacity(k);
    for freq in 0..k {
        let mut re_x = 0.0;
        let mut im_x = 0.0;
        let mut re_y = 0.0;
        let mut im_y = 0.0;
        for (t, p) in points.iter().enumerate() {
            let angle = -2.0 * std::f64::consts::PI * (freq as f64) * (t as f64) / (n as f64);
            let c = angle.cos();
            let s = angle.sin();
            re_x += p.0 * c;
            im_x += p.0 * s;
            re_y += p.1 * c;
            im_y += p.1 * s;
        }
        let mag = ((re_x * re_x + im_x * im_x) + (re_y * re_y + im_y * im_y)).sqrt();
        mags.push(mag);
    }
    // normalize magnitudes
    let norm: f64 = mags.iter().sum();
    if norm > 0.0 {
        for m in mags.iter_mut() {
            *m /= norm;
        }
    }
    mags
}

/// Public function: compute descriptor from raw stroke points.
/// - resample_n: number of points to resample to (e.g., 64)
/// - curvature_bins: number of histogram bins (e.g., 16)
/// - fourier_k: number of Fourier magnitudes to keep (e.g., 8)
pub fn descriptor_from_stroke(
    stroke: &[(f64, f64)],
    resample_n: usize,
    curvature_bins: usize,
    fourier_k: usize,
) -> Option<Descriptor> {
    if stroke.len() < 2 || resample_n < 2 || curvature_bins == 0 || fourier_k == 0 {
        return None;
    }
    let resampled = resample_to_n(stroke, resample_n);
    // normalized length
    let mut length = 0.0;
    for i in 1..resampled.len() {
        length += euclid(resampled[i], resampled[i - 1]);
    }
    let norm_length = if length.is_finite() { length } else { 0.0 };
    // curvature histogram
    let curv = compute_curvatures(&resampled);
    let curvature_hist = histogram(&curv, curvature_bins, std::f64::consts::PI);
    // fourier magnitudes
    let fourier_mag = dft_magnitudes(&resampled, fourier_k);
    Some(Descriptor {
        norm_length,
        curvature_hist,
        fourier_mag,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_runs_on_circle() {
        // simple circle stroke
        let mut pts = Vec::new();
        for i in 0..32 {
            let t = 2.0 * std::f64::consts::PI * (i as f64) / 32.0;
            pts.push((t.cos(), t.sin()));
        }
        let desc = descriptor_from_stroke(&pts, 64, 16, 8).unwrap();
        assert_eq!(desc.curvature_hist.len(), 16);
        assert_eq!(desc.fourier_mag.len(), 8);
        assert!(desc.norm_length > 0.0);
    }
}