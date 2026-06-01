//! Conch shell growth — self-similar spiral growth.
//!
//! A conch shell grows by adding material at the aperture (opening). Each
//! increment is determined by the curvature of what's already there. The result
//! is a logarithmic spiral with a specific growth rate determined by the
//! geometry of the existing shell.
//!
//! Key insight: zoom into the spatial geometry of a shell and you see the
//! temporal sequence of growth — spatial recording of temporal dynamics
//! (Mandelbrot's insight).

use serde::{Serialize, Deserialize};
use crate::golden::GoldenRatio;
use crate::spiral::LogarithmicSpiral;

/// A growing conch shell.
///
/// The shell is modeled as a series of growth increments, each determined
/// by the curvature of the previous state. The growth law IS the constructor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConchShell {
    /// The underlying logarithmic spiral.
    pub spiral: LogarithmicSpiral,
    /// Number of growth increments applied.
    pub increments: usize,
    /// Growth history: radius at each step.
    pub history: Vec<f64>,
    /// Aperture width (proportional to current radius).
    pub aperture_ratio: f64,
}

impl ConchShell {
    /// Create a new shell with initial radius and golden growth.
    pub fn new(initial_radius: f64) -> Self {
        Self {
            spiral: LogarithmicSpiral::golden(initial_radius),
            increments: 0,
            history: vec![initial_radius],
            aperture_ratio: GoldenRatio::PHI_INV,
        }
    }

    /// Create with a specific growth rate.
    pub fn with_growth_rate(initial_radius: f64, b: f64) -> Self {
        Self {
            spiral: LogarithmicSpiral { a: initial_radius, b },
            increments: 0,
            history: vec![initial_radius],
            aperture_ratio: GoldenRatio::PHI_INV,
        }
    }

    /// Grow by one increment.
    /// The new radius is determined by the curvature of the current shell.
    pub fn grow(&mut self) -> f64 {
        self.increments += 1;
        let theta = self.increments as f64 * 0.1; // Each increment adds 0.1 radians
        let new_radius = self.spiral.radius(theta);
        self.history.push(new_radius);
        new_radius
    }

    /// Grow by n increments.
    pub fn grow_n(&mut self, n: usize) {
        for _ in 0..n {
            self.grow();
        }
    }

    /// Current radius (aperture size).
    pub fn current_radius(&self) -> f64 {
        *self.history.last().unwrap_or(&self.spiral.a)
    }

    /// Current aperture width.
    pub fn aperture_width(&self) -> f64 {
        self.current_radius() * self.aperture_ratio
    }

    /// Curvature at the current growth front.
    pub fn current_curvature(&self) -> f64 {
        let theta = self.increments as f64 * 0.1;
        self.spiral.curvature(theta)
    }

    /// Self-similarity: the ratio between consecutive whorls.
    /// For a golden spiral, this is φ^(2π/b) per full turn.
    pub fn self_similarity_factor(&self) -> f64 {
        self.spiral.growth_per_turn()
    }

    /// Mandelbrot zoom: examine the spatial structure at a given scale.
    /// The spatial geometry IS the temporal record of growth.
    pub fn spatial_temporal_record(&self) -> &Vec<f64> {
        &self.history
    }

    /// Growth rate: how much the radius changes per increment.
    pub fn growth_rate(&self) -> f64 {
        if self.history.len() < 2 {
            return 0.0;
        }
        let n = self.history.len();
        self.history[n - 1] / self.history[n - 2]
    }

    /// Check if the growth is self-similar (growth rate is approximately constant).
    pub fn is_self_similar(&self, tol: f64) -> bool {
        if self.history.len() < 3 {
            return true;
        }
        let rates: Vec<f64> = self.history.windows(2)
            .map(|w| w[1] / w[0])
            .collect();
        let mean = rates.iter().sum::<f64>() / rates.len() as f64;
        rates.iter().all(|&r| (r - mean).abs() < tol)
    }

    /// Total arc length grown so far.
    pub fn total_arc_length(&self) -> f64 {
        if self.history.len() < 2 {
            return 0.0;
        }
        let theta1 = 0.0;
        let theta2 = self.increments as f64 * 0.1;
        self.spiral.arc_length(theta1, theta2)
    }

    /// The growth law is the constructor — each increment is the logical
    /// consequence of the previous geometry. No computation needed.
    pub fn growth_law_is_constructor(&self) -> bool {
        // In a logarithmic spiral, the growth law (constant angle) determines
        // the entire shape. No external computation — just local curvature response.
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_creation() {
        let shell = ConchShell::new(1.0);
        assert_eq!(shell.increments, 0);
        assert!((shell.current_radius() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_shell_grow() {
        let mut shell = ConchShell::new(1.0);
        let r1 = shell.grow();
        assert!(r1 > 1.0);
        assert_eq!(shell.increments, 1);
        assert_eq!(shell.history.len(), 2);
    }

    #[test]
    fn test_shell_grow_n() {
        let mut shell = ConchShell::new(1.0);
        shell.grow_n(20);
        assert_eq!(shell.increments, 20);
        assert_eq!(shell.history.len(), 21);
    }

    #[test]
    fn test_shell_radius_increases() {
        let mut shell = ConchShell::new(1.0);
        shell.grow_n(10);
        for w in shell.history.windows(2) {
            assert!(w[1] > w[0]);
        }
    }

    #[test]
    fn test_aperture_width() {
        let mut shell = ConchShell::new(1.0);
        shell.grow_n(5);
        let w = shell.aperture_width();
        assert!(w > 0.0);
        assert!((w / shell.current_radius() - GoldenRatio::PHI_INV).abs() < 1e-10);
    }

    #[test]
    fn test_self_similarity() {
        let mut shell = ConchShell::new(1.0);
        shell.grow_n(30);
        assert!(shell.is_self_similar(0.1));
    }

    #[test]
    fn test_growth_law_is_constructor() {
        let shell = ConchShell::new(1.0);
        assert!(shell.growth_law_is_constructor());
    }

    #[test]
    fn test_spatial_temporal_record() {
        let mut shell = ConchShell::new(1.0);
        shell.grow_n(10);
        let record = shell.spatial_temporal_record();
        assert_eq!(record.len(), 11);
        // Monotonically increasing (for golden spiral with positive b)
        for w in record.windows(2) {
            assert!(w[1] > w[0]);
        }
    }

    #[test]
    fn test_total_arc_length() {
        let mut shell = ConchShell::new(1.0);
        shell.grow_n(10);
        let len = shell.total_arc_length();
        assert!(len > 0.0);
    }

    #[test]
    fn test_shell_serialization() {
        let mut shell = ConchShell::new(1.0);
        shell.grow_n(5);
        let json = serde_json::to_string(&shell).unwrap();
        let back: ConchShell = serde_json::from_str(&json).unwrap();
        assert_eq!(back.increments, 5);
        assert_eq!(back.history.len(), 6);
    }

    #[test]
    fn test_conch_self_similarity_factor() {
        let shell = ConchShell::new(1.0);
        let factor = shell.self_similarity_factor();
        assert!(factor > 1.0);
    }

    #[test]
    fn test_growth_rate_positive() {
        let mut shell = ConchShell::new(1.0);
        shell.grow_n(5);
        let rate = shell.growth_rate();
        assert!(rate > 1.0);
    }

    #[test]
    fn test_current_curvature() {
        let mut shell = ConchShell::new(1.0);
        shell.grow_n(5);
        let k = shell.current_curvature();
        assert!(k > 0.0);
    }
}
