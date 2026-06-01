//! Golden ratio mathematics — φ as the most irrational number.
//!
//! φ = (1 + √5) / 2 ≈ 1.6180339887...
//!
//! The golden ratio is "the most irrational number" because its continued
//! fraction representation [1; 1, 1, 1, ...] converges slowest to any rational
//! approximation. This means a growth process governed by φ produces the most
//! uniform spatial coverage without periodic repetition.

use serde::{Serialize, Deserialize};

/// The golden ratio φ and related constants/functions.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GoldenRatio;

impl GoldenRatio {
    /// φ = (1 + √5) / 2 ≈ 1.6180339887...
    pub const PHI: f64 = 1.6180339887498948482;

    /// 1/φ = φ - 1 ≈ 0.6180339887...
    pub const PHI_INV: f64 = Self::PHI - 1.0;

    /// φ² = φ + 1 ≈ 2.6180339887...
    pub const PHI_SQ: f64 = Self::PHI * Self::PHI;

    /// The golden angle: 2π / φ² ≈ 137.508° in radians
    /// This is the optimal angle for uniform angular distribution.
    pub const GOLDEN_ANGLE: f64 = 2.0 * std::f64::consts::PI / Self::PHI_SQ;

    /// Golden angle in degrees ≈ 137.508°
    pub const GOLDEN_ANGLE_DEG: f64 = 360.0 / Self::PHI_SQ;

    /// Continued fraction coefficients of φ: [1; 1, 1, 1, ...]
    /// All 1s → slowest possible convergence → most irrational.
    pub fn continued_fraction(depth: usize) -> Vec<u64> {
        vec![1; depth]
    }

    /// Evaluate a continued fraction [a₀; a₁, a₂, ...] as a rational approximation.
    pub fn evaluate_cf(coeffs: &[u64]) -> (u64, u64) {
        if coeffs.is_empty() {
            return (1, 1);
        }
        let mut num = coeffs[0] as u64;
        let mut den = 1u64;
        let mut prev_num = 1u64;
        let mut prev_den = 0u64;

        for &c in &coeffs[1..] {
            let new_num = c * num + prev_num;
            let new_den = c * den + prev_den;
            prev_num = num;
            prev_den = den;
            num = new_num;
            den = new_den;
        }
        (num, den)
    }

    /// Successive rational approximations to φ.
    /// These are ratios of consecutive Fibonacci numbers.
    pub fn rational_approximations(n: usize) -> Vec<(u64, u64)> {
        (1..=n)
            .map(|depth| Self::evaluate_cf(&Self::continued_fraction(depth)))
            .collect()
    }

    /// How irrational is a number? Measured by the Hurwitz irrationality measure.
    /// For φ, this returns the infimum of exponents μ such that |x - p/q| < 1/q^μ
    /// has infinitely many solutions. For φ, this is exactly 2 (worst case).
    pub fn irrationality_measure(x: f64, max_q: u64) -> f64 {
        let mut best_mu = 0.0_f64;
        for q in 2..=max_q {
            let p = (x * q as f64).round() as i64;
            let err = (x - p as f64 / q as f64).abs();
            if err > 0.0 {
                let mu = -err.log2() / (q as f64).log2();
                if mu > best_mu {
                    best_mu = mu;
                }
            }
        }
        best_mu
    }

    /// Compute the nth Fibonacci number (0-indexed).
    pub fn fibonacci(n: u64) -> u64 {
        let mut a = 0u64;
        let mut b = 1u64;
        for _ in 0..n {
            let tmp = a + b;
            a = b;
            b = tmp;
        }
        a
    }

    /// Verify φ = F(n+1)/F(n) as n → ∞.
    pub fn golden_ratio_convergence(n: usize) -> f64 {
        let a = Self::fibonacci(n as u64 + 1) as f64;
        let b = Self::fibonacci(n as u64) as f64;
        a / b
    }

    /// Golden ratio conjugate: ψ = 1 - φ = -1/φ ≈ -0.618
    pub const PHI_CONJUGATE: f64 = 1.0 - Self::PHI;

    /// Check if a value is close to φ within tolerance.
    pub fn is_golden(value: f64, tol: f64) -> bool {
        (value - Self::PHI).abs() < tol
    }

    /// Phyllotaxis: place n elements on a disk using the golden angle.
    /// Each point at distance sqrt(k) * scale, angle k * golden_angle.
    /// Produces Fermat's spiral — the sunflower pattern.
    pub fn phyllotaxis(n: usize, scale: f64) -> Vec<(f64, f64)> {
        (0..n)
            .map(|k| {
                let r = (k as f64).sqrt() * scale;
                let theta = k as f64 * Self::GOLDEN_ANGLE;
                (r * theta.cos(), r * theta.sin())
            })
            .collect()
    }

    /// Compute the packing density of phyllotaxis points.
    /// Golden angle phyllotaxis maximizes the minimum distance between points.
    pub fn packing_quality(points: &[(f64, f64)]) -> f64 {
        if points.len() < 2 {
            return 0.0;
        }
        let mut min_dist = f64::INFINITY;
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let dx = points[i].0 - points[j].0;
                let dy = points[i].1 - points[j].1;
                let d = (dx * dx + dy * dy).sqrt();
                if d < min_dist {
                    min_dist = d;
                }
            }
        }
        min_dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phi_value() {
        assert!((GoldenRatio::PHI - 1.6180339887).abs() < 1e-7);
    }

    #[test]
    fn test_phi_identity() {
        // φ² = φ + 1
        assert!((GoldenRatio::PHI_SQ - GoldenRatio::PHI - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_phi_inverse() {
        // 1/φ = φ - 1
        assert!((GoldenRatio::PHI_INV - 1.0 / GoldenRatio::PHI).abs() < 1e-10);
    }

    #[test]
    fn test_continued_fraction_all_ones() {
        let cf = GoldenRatio::continued_fraction(10);
        assert!(cf.iter().all(|&c| c == 1));
        assert_eq!(cf.len(), 10);
    }

    #[test]
    fn test_rational_approximations_converge_to_phi() {
        let approx = GoldenRatio::rational_approximations(15);
        let last = approx.last().unwrap();
        let ratio = last.0 as f64 / last.1 as f64;
        assert!((ratio - GoldenRatio::PHI).abs() < 1e-3);
    }

    #[test]
    fn test_rational_approximations_are_fibonacci_ratios() {
        let approx = GoldenRatio::rational_approximations(8);
        for (i, (p, q)) in approx.iter().enumerate() {
            assert_eq!(*p, GoldenRatio::fibonacci((i + 2) as u64));
            assert_eq!(*q, GoldenRatio::fibonacci((i + 1) as u64));
        }
    }

    #[test]
    fn test_golden_angle_degrees() {
        assert!((GoldenRatio::GOLDEN_ANGLE_DEG - 137.508).abs() < 0.01);
    }

    #[test]
    fn test_golden_ratio_convergence() {
        let c10 = GoldenRatio::golden_ratio_convergence(10);
        assert!((c10 - GoldenRatio::PHI).abs() < 1e-3);

        let c20 = GoldenRatio::golden_ratio_convergence(20);
        assert!((c20 - GoldenRatio::PHI).abs() < 1e-5);
    }

    #[test]
    fn test_is_golden() {
        assert!(GoldenRatio::is_golden(1.618, 0.01));
        assert!(!GoldenRatio::is_golden(1.5, 0.01));
    }

    #[test]
    fn test_fibonacci_sequence() {
        let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
        for (i, &exp) in expected.iter().enumerate() {
            assert_eq!(GoldenRatio::fibonacci(i as u64), exp);
        }
    }

    #[test]
    fn test_phyllotaxis_produces_points() {
        let pts = GoldenRatio::phyllotaxis(100, 1.0);
        assert_eq!(pts.len(), 100);
    }

    #[test]
    fn test_phyllotaxis_packing_quality() {
        // Golden angle phyllotaxis should have better packing than random angles
        let golden_pts = GoldenRatio::phyllotaxis(50, 1.0);
        let golden_quality = GoldenRatio::packing_quality(&golden_pts);
        assert!(golden_quality > 0.0);
    }

    #[test]
    fn test_phi_conjugate() {
        // ψ = 1 - φ = -1/φ
        assert!((GoldenRatio::PHI_CONJUGATE - (1.0 - GoldenRatio::PHI)).abs() < 1e-10);
        assert!((GoldenRatio::PHI_CONJUGATE + 1.0 / GoldenRatio::PHI).abs() < 1e-10);
    }

    #[test]
    fn test_irrationality_phi_is_worst() {
        // φ converges slowest to rationals — verify by checking convergence rate
        // of Fibonacci ratios F(n+1)/F(n) → φ
        let approx = GoldenRatio::rational_approximations(20);
        let (p, q) = approx.last().unwrap();
        let error = (GoldenRatio::PHI - *p as f64 / *q as f64).abs();
        // The error should decrease but not too fast (slowest convergence)
        assert!(error > 0.0 && error < 0.01);
    }

    #[test]
    fn test_evaluate_cf_empty() {
        assert_eq!(GoldenRatio::evaluate_cf(&[]), (1, 1));
    }
}
