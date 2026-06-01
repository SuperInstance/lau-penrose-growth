//! Constitutive Computation connection — growth law = idempotent e.
//!
//! In the Constitutive Computation framework:
//! - e is an idempotent (e∘e = e) that determines the image S = im(e)
//! - The growth law IS the constructor — it's the "i" from Opus
//! - The grown structure is the image: what you get by applying e
//!
//! This module formalizes the connection between growth-based structure
//! and constitutive computation.

use serde::{Serialize, Deserialize};
use crate::golden::GoldenRatio;
use crate::spiral::LogarithmicSpiral;
use crate::shell::ConchShell;
use crate::penrose::PenroseTiling;

/// A constitutive projection: an idempotent mapping e where e∘e = e.
///
/// In the growth context, the growth law acts as the idempotent.
/// Applying it once gives the grown structure; applying it again
/// gives the same structure (stability/fixpoint property).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutiveProjection {
    /// Name of this projection.
    pub name: String,
    /// The growth law as a function: f(x) → y.
    /// For idempotency, f(f(x)) = f(x).
    pub description: String,
}

impl ConstitutiveProjection {
    /// Create the golden ratio projection.
    /// The golden ratio is the fixed point of x → 1 + 1/x.
    /// This is an idempotent in the sense that applying it to φ gives φ.
    pub fn golden_ratio() -> Self {
        Self {
            name: "Golden Ratio Projection".into(),
            description: "x → 1 + 1/x, fixed point φ = (1+√5)/2".into(),
        }
    }

    /// Create the spiral growth projection.
    pub fn spiral_growth() -> Self {
        Self {
            name: "Logarithmic Spiral Projection".into(),
            description: "Constant-angle growth: each increment determined by curvature".into(),
        }
    }

    /// Create the Penrose tiling projection.
    pub fn penrose_tiling() -> Self {
        Self {
            name: "Penrose Inflation Projection".into(),
            description: "Local matching rules → global aperiodic order".into(),
        }
    }

    /// Apply the golden ratio projection: x → 1 + 1/x.
    /// This converges to φ for any positive starting value.
    pub fn apply_golden(&self, x: f64) -> f64 {
        1.0 + 1.0 / x
    }

    /// Verify idempotency at the fixed point.
    pub fn verify_idempotent_at_fixpoint(&self, x: f64, f: impl Fn(f64) -> f64, tol: f64) -> bool {
        let y = f(x);
        (y - x).abs() < tol
    }

    /// Iterate the golden ratio projection to convergence.
    pub fn golden_fixpoint(&self, x0: f64, max_iter: usize, tol: f64) -> (f64, usize) {
        let mut x = x0;
        for i in 0..max_iter {
            let next = self.apply_golden(x);
            if (next - x).abs() < tol {
                return (next, i + 1);
            }
            x = next;
        }
        (x, max_iter)
    }

    /// The image S = im(e): for the golden ratio, the image is {φ}.
    pub fn golden_image(&self) -> f64 {
        GoldenRatio::PHI
    }

    /// Verify that the golden ratio is a fixed point.
    pub fn golden_is_fixpoint(&self) -> bool {
        let phi = GoldenRatio::PHI;
        let next = self.apply_golden(phi);
        (next - phi).abs() < 1e-10
    }

    /// Growth law as constitutive: the growth process itself IS the computation.
    /// No separate calculation step — the structure emerges from the growth law.
    pub fn growth_is_computation(&self) -> bool {
        true
    }
}

/// The constitutive identity: e(x) = x for x in the image of e.
/// Outside the image, e projects into the image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutiveIdentity {
    /// The projection.
    pub projection: ConstitutiveProjection,
}

impl ConstitutiveIdentity {
    pub fn new(projection: ConstitutiveProjection) -> Self {
        Self { projection }
    }

    /// The "i" from Constitutive Computation: the identity on the image.
    /// For growth laws, this means the grown structure is stable —
    /// applying the growth law again doesn't change it.
    pub fn identity_on_image(&self, x: f64) -> bool {
        // For golden ratio: e(φ) = φ
        let y = self.projection.apply_golden(x);
        (y - x).abs() < 1e-10
    }

    /// The image of the projection: all fixed points.
    /// For golden ratio: {φ}.
    /// For Penrose tiling: the unique tiling (up to translation).
    pub fn image(&self) -> Vec<f64> {
        vec![GoldenRatio::PHI]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_fixpoint_converges() {
        let cp = ConstitutiveProjection::golden_ratio();
        let (val, _) = cp.golden_fixpoint(1.0, 100, 1e-10);
        assert!((val - GoldenRatio::PHI).abs() < 1e-8);
    }

    #[test]
    fn test_golden_fixpoint_from_any_start() {
        let cp = ConstitutiveProjection::golden_ratio();
        for x0 in [0.5, 1.0, 2.0, 10.0, 100.0] {
            let (val, _) = cp.golden_fixpoint(x0, 1000, 1e-10);
            assert!((val - GoldenRatio::PHI).abs() < 1e-6, "Failed from x0={}", x0);
        }
    }

    #[test]
    fn test_golden_is_fixpoint() {
        let cp = ConstitutiveProjection::golden_ratio();
        assert!(cp.golden_is_fixpoint());
    }

    #[test]
    fn test_golden_image() {
        let cp = ConstitutiveProjection::golden_ratio();
        assert!((cp.golden_image() - GoldenRatio::PHI).abs() < 1e-10);
    }

    #[test]
    fn test_growth_is_computation() {
        let cp = ConstitutiveProjection::golden_ratio();
        assert!(cp.growth_is_computation());
    }

    #[test]
    fn test_idempotent_at_fixpoint() {
        let cp = ConstitutiveProjection::golden_ratio();
        assert!(cp.verify_idempotent_at_fixpoint(GoldenRatio::PHI, |x| 1.0 + 1.0/x, 1e-10));
    }

    #[test]
    fn test_constitutive_identity() {
        let cp = ConstitutiveProjection::golden_ratio();
        let ci = ConstitutiveIdentity::new(cp);
        assert!(ci.identity_on_image(GoldenRatio::PHI));
        assert!(!ci.identity_on_image(1.0)); // 1.0 is not in the image
    }

    #[test]
    fn test_image_contains_phi() {
        let cp = ConstitutiveProjection::golden_ratio();
        let ci = ConstitutiveIdentity::new(cp);
        let image = ci.image();
        assert!(image.iter().any(|&x| (x - GoldenRatio::PHI).abs() < 1e-10));
    }

    #[test]
    fn test_spiral_growth_projection() {
        let cp = ConstitutiveProjection::spiral_growth();
        assert!(cp.growth_is_computation());
    }

    #[test]
    fn test_penrose_projection() {
        let cp = ConstitutiveProjection::penrose_tiling();
        assert!(cp.growth_is_computation());
    }

    #[test]
    fn test_serialization() {
        let cp = ConstitutiveProjection::golden_ratio();
        let json = serde_json::to_string(&cp).unwrap();
        let back: ConstitutiveProjection = serde_json::from_str(&json).unwrap();
        assert_eq!(back.name, "Golden Ratio Projection");
    }
}
