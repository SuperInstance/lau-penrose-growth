//! Logarithmic spiral — growth by curvature.
//!
//! A logarithmic spiral r = a * e^(bθ) has the property that the angle between
//! the radius and the tangent is constant. Each increment is determined by the
//! curvature of what's already there — growth by geometric consequence, not
//! calculation.

use serde::{Serialize, Deserialize};
use crate::golden::GoldenRatio;

/// A logarithmic (equiangular) spiral.
///
/// r(θ) = a * e^(bθ) where:
/// - a is the initial radius
/// - b controls how tightly the spiral winds
/// - The angle between radius and tangent is constant: α = atan(1/b)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogarithmicSpiral {
    /// Initial radius (scale factor).
    pub a: f64,
    /// Growth rate. b = 1/tan(α) where α is the constant angle.
    pub b: f64,
}

impl LogarithmicSpiral {
    /// Create a golden spiral: a logarithmic spiral where the growth factor per
    /// quarter turn is φ.
    pub fn golden(a: f64) -> Self {
        // r increases by factor φ per quarter turn: b = ln(φ) / (π/2)
        let b = GoldenRatio::PHI.ln() / (std::f64::consts::PI / 2.0);
        Self { a, b }
    }

    /// Create a spiral with a specific constant angle α.
    pub fn with_angle(a: f64, alpha: f64) -> Self {
        let b = 1.0 / alpha.tan();
        Self { a, b }
    }

    /// Radius at angle θ.
    pub fn radius(&self, theta: f64) -> f64 {
        self.a * (self.b * theta).exp()
    }

    /// Point on the spiral at angle θ.
    pub fn point(&self, theta: f64) -> (f64, f64) {
        let r = self.radius(theta);
        (r * theta.cos(), r * theta.sin())
    }

    /// Generate n equally-spaced points (by angle) along the spiral.
    pub fn points(&self, n: usize, d_theta: f64) -> Vec<(f64, f64)> {
        (0..n).map(|i| self.point(i as f64 * d_theta)).collect()
    }

    /// The constant angle α between radius vector and tangent.
    pub fn constant_angle(&self) -> f64 {
        (1.0 / self.b).atan()
    }

    /// Arc length from θ₁ to θ₂.
    pub fn arc_length(&self, theta1: f64, theta2: f64) -> f64 {
        // L = (r(θ₂) - r(θ₁)) / cos(α)
        let r1 = self.radius(theta1);
        let r2 = self.radius(theta2);
        let cos_alpha = self.constant_angle().cos();
        (r2 - r1) / cos_alpha
    }

    /// Curvature at angle θ.
    pub fn curvature(&self, theta: f64) -> f64 {
        // κ = sin(α) / r(θ)
        self.constant_angle().sin() / self.radius(theta)
    }

    /// Growth factor: how much the radius increases per full turn.
    pub fn growth_per_turn(&self) -> f64 {
        (self.b * 2.0 * std::f64::consts::PI).exp()
    }

    /// Check if this is a golden spiral (growth per quarter turn = φ).
    pub fn is_golden(&self, tol: f64) -> bool {
        let quarter_growth = (self.b * std::f64::consts::PI / 2.0).exp();
        GoldenRatio::is_golden(quarter_growth, tol)
    }

    /// The self-similarity ratio: scaling by this factor is equivalent to
    /// rotating by the golden angle.
    pub fn self_similarity_ratio(&self) -> f64 {
        self.growth_per_turn()
    }

    /// Mandelbrot zoom: the spatial geometry encodes temporal dynamics.
    /// Zooming in by factor s reveals the same spiral shifted in time.
    pub fn mandelbrot_zoom(&self, zoom_factor: f64) -> Self {
        // Zooming by factor f is equivalent to shifting θ by ln(f)/b
        let a_new = self.a * zoom_factor;
        Self { a: a_new, b: self.b }
    }

    /// Conch self-similarity: the spatial geometry records temporal growth.
    /// Each whorl is a scaled copy of the previous one.
    pub fn whorl_ratio(&self) -> f64 {
        self.growth_per_turn()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_radius() {
        let s = LogarithmicSpiral { a: 1.0, b: 0.1 };
        assert!((s.radius(0.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_golden_spiral_growth() {
        let s = LogarithmicSpiral::golden(1.0);
        let r0 = s.radius(0.0);
        let r_quarter = s.radius(std::f64::consts::PI / 2.0);
        let ratio = r_quarter / r0;
        assert!(GoldenRatio::is_golden(ratio, 0.01));
    }

    #[test]
    fn test_golden_spiral_is_golden() {
        let s = LogarithmicSpiral::golden(1.0);
        assert!(s.is_golden(0.01));
    }

    #[test]
    fn test_constant_angle() {
        let s = LogarithmicSpiral::with_angle(1.0, std::f64::consts::PI / 4.0);
        let alpha = s.constant_angle();
        assert!((alpha - std::f64::consts::PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_points_generation() {
        let s = LogarithmicSpiral::golden(1.0);
        let pts = s.points(50, 0.1);
        assert_eq!(pts.len(), 50);
    }

    #[test]
    fn test_arc_length_positive() {
        let s = LogarithmicSpiral::golden(1.0);
        let len = s.arc_length(0.0, std::f64::consts::PI);
        assert!(len > 0.0);
    }

    #[test]
    fn test_curvature_decreases() {
        // In a logarithmic spiral, curvature decreases with radius
        let s = LogarithmicSpiral::golden(1.0);
        let k1 = s.curvature(0.0);
        let k2 = s.curvature(1.0);
        assert!(k2 < k1);
    }

    #[test]
    fn test_growth_per_turn() {
        let s = LogarithmicSpiral::golden(1.0);
        let growth = s.growth_per_turn();
        assert!(growth > 1.0);
    }

    #[test]
    fn test_self_similarity() {
        let s = LogarithmicSpiral::golden(1.0);
        let ratio = s.self_similarity_ratio();
        assert!(ratio > 1.0);
    }

    #[test]
    fn test_mandelbrot_zoom() {
        let s = LogarithmicSpiral::golden(1.0);
        let zoomed = s.mandelbrot_zoom(2.0);
        assert!((zoomed.a - 2.0).abs() < 1e-10);
        assert!((zoomed.b - s.b).abs() < 1e-10);
    }

    #[test]
    fn test_whorl_ratio() {
        let s = LogarithmicSpiral::golden(1.0);
        let ratio = s.whorl_ratio();
        assert!(ratio > 1.0);
    }

    #[test]
    fn test_spiral_serialization() {
        let s = LogarithmicSpiral::golden(1.5);
        let json = serde_json::to_string(&s).unwrap();
        let back: LogarithmicSpiral = serde_json::from_str(&json).unwrap();
        assert!((back.a - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_radius_monotonic() {
        let s = LogarithmicSpiral::golden(1.0);
        let mut prev = s.radius(0.0);
        for i in 1..20 {
            let theta = i as f64 * 0.5;
            let r = s.radius(theta);
            assert!(r > prev);
            prev = r;
        }
    }
}
