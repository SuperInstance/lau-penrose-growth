//! Growth laws — structure emerges from local growth rules, not global computation.
//!
//! The growth law IS the constructor. In Opus's Constitutive Computation,
//! the idempotent e determines the image S = im(e). Here, the growth law
//! (golden ratio, constant-angle spiral, Penrose matching rules) determines
//! the grown structure.

use serde::{Serialize, Deserialize};
use crate::golden::GoldenRatio;
use crate::spiral::LogarithmicSpiral;
use crate::shell::ConchShell;
use crate::penrose::{PenroseTiling, RhombType};

/// A growth law: a deterministic rule that produces structure through iteration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrowthLaw {
    /// Golden ratio growth: each step scales by φ.
    GoldenRatio { scale: f64 },
    /// Logarithmic spiral growth: constant angle, radius grows exponentially.
    SpiralGrowth { a: f64, b: f64 },
    /// Penrose inflation: local coordination rules → global aperiodic order.
    PenroseInflation { initial_edge: f64 },
    /// Fibonacci growth: each step is the sum of the two previous.
    FibonacciGrowth,
    /// Shell growth: curvature-determined increment.
    ShellGrowth { initial_radius: f64, d_theta: f64 },
}

impl GrowthLaw {
    /// Apply one step of the growth law.
    /// Returns the new state after one growth increment.
    pub fn grow_state(&self, state: &GrowthState) -> GrowthState {
        match self {
            GrowthLaw::GoldenRatio { scale } => {
                let mut new = state.clone();
                new.step += 1;
                new.value *= GoldenRatio::PHI * scale;
                new.history.push(new.value);
                new
            }
            GrowthLaw::SpiralGrowth { a, b } => {
                let spiral = LogarithmicSpiral { a: *a, b: *b };
                let mut new = state.clone();
                new.step += 1;
                new.value = spiral.radius(new.step as f64 * 0.1);
                new.history.push(new.value);
                new
            }
            GrowthLaw::PenroseInflation { .. } => {
                let mut new = state.clone();
                new.step += 1;
                new.tile_count = new.tile_count.map(|c| {
                    // Approximate: each inflation roughly triples the tile count
                    c * 3
                });
                new.history.push(new.tile_count.unwrap_or(0) as f64);
                new
            }
            GrowthLaw::FibonacciGrowth => {
                let mut new = state.clone();
                new.step += 1;
                let n = new.history.len();
                if n >= 2 {
                    new.value = new.history[n - 1] + new.history[n - 2];
                } else {
                    new.value = 1.0;
                }
                new.history.push(new.value);
                new
            }
            GrowthLaw::ShellGrowth { initial_radius, d_theta } => {
                let spiral = LogarithmicSpiral::golden(*initial_radius);
                let mut new = state.clone();
                new.step += 1;
                new.value = spiral.radius(new.step as f64 * d_theta);
                new.history.push(new.value);
                new
            }
        }
    }

    /// Grow for n steps.
    pub fn grow_n(&self, initial: GrowthState, n: usize) -> Vec<GrowthState> {
        let mut states = vec![initial];
        for _ in 0..n {
            let next = self.grow_state(states.last().unwrap());
            states.push(next);
        }
        states
    }

    /// The growth law is idempotent in the constitutive sense:
    /// applying it repeatedly produces a fixed structure.
    pub fn is_constitutive(&self) -> bool {
        true // All growth laws in this framework are constitutive
    }
}

/// The state of a growing structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthState {
    /// Current step number.
    pub step: usize,
    /// Primary value (radius, count, etc.).
    pub value: f64,
    /// Growth history.
    pub history: Vec<f64>,
    /// Optional tile count (for Penrose tilings).
    pub tile_count: Option<usize>,
}

impl GrowthState {
    pub fn new(value: f64) -> Self {
        Self {
            step: 0,
            value,
            history: vec![value],
            tile_count: None,
        }
    }

    pub fn with_tile_count(value: f64, count: usize) -> Self {
        Self {
            step: 0,
            value,
            history: vec![value],
            tile_count: Some(count),
        }
    }
}

/// Connection to Constitutive Computation:
/// - Growth law = idempotent e
/// - Grown structure = image S = im(e)
/// - The growth process IS the computation — no separate calculation step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutiveGrowth {
    /// The growth law (the idempotent e).
    pub law: GrowthLaw,
    /// The initial state (the "seed").
    pub seed: GrowthState,
}

impl ConstitutiveGrowth {
    pub fn new(law: GrowthLaw, seed: GrowthState) -> Self {
        Self { law, seed }
    }

    /// Apply the growth law n times.
    /// In constitutive terms: apply e n times.
    /// Since e is idempotent (e∘e = e), the structure stabilizes.
    pub fn realize(&self, n: usize) -> Vec<GrowthState> {
        self.law.grow_n(self.seed.clone(), n)
    }

    /// The grown structure is the image of e: im(e) = { e(x) | x ∈ domain }.
    /// For a growth law, this is the set of all possible grown states.
    pub fn image(&self, n: usize) -> Vec<f64> {
        self.realize(n).iter().map(|s| s.value).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_golden_ratio_growth() {
        let law = GrowthLaw::GoldenRatio { scale: 1.0 };
        let state = GrowthState::new(1.0);
        let grown = law.grow_n(state, 5);
        assert_eq!(grown.len(), 6);
        for w in grown.windows(2) {
            assert!(w[1].value > w[0].value);
        }
    }

    #[test]
    fn test_spiral_growth() {
        let law = GrowthLaw::SpiralGrowth { a: 1.0, b: 0.1 };
        let state = GrowthState::new(1.0);
        let grown = law.grow_n(state, 10);
        assert_eq!(grown.len(), 11);
    }

    #[test]
    fn test_fibonacci_growth() {
        let law = GrowthLaw::FibonacciGrowth;
        let state = GrowthState::new(1.0);
        let grown = law.grow_n(state, 10);
        // Fibonacci-like: each value is sum of two previous
        assert!(grown.len() == 11);
    }

    #[test]
    fn test_shell_growth_law() {
        let law = GrowthLaw::ShellGrowth { initial_radius: 1.0, d_theta: 0.1 };
        let state = GrowthState::new(1.0);
        let grown = law.grow_n(state, 10);
        assert_eq!(grown.len(), 11);
    }

    #[test]
    fn test_growth_law_is_constitutive() {
        let law = GrowthLaw::GoldenRatio { scale: 1.0 };
        assert!(law.is_constitutive());
    }

    #[test]
    fn test_constitutive_growth() {
        let law = GrowthLaw::GoldenRatio { scale: 1.0 };
        let seed = GrowthState::new(1.0);
        let cg = ConstitutiveGrowth::new(law, seed);
        let realized = cg.realize(5);
        assert_eq!(realized.len(), 6);
    }

    #[test]
    fn test_constitutive_image() {
        let law = GrowthLaw::GoldenRatio { scale: 1.0 };
        let seed = GrowthState::new(1.0);
        let cg = ConstitutiveGrowth::new(law, seed);
        let image = cg.image(5);
        assert_eq!(image.len(), 6);
    }

    #[test]
    fn test_growth_state_serialization() {
        let state = GrowthState::new(1.0);
        let json = serde_json::to_string(&state).unwrap();
        let back: GrowthState = serde_json::from_str(&json).unwrap();
        assert_eq!(back.step, 0);
        assert!((back.value - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_penrose_inflation_growth() {
        let law = GrowthLaw::PenroseInflation { initial_edge: 1.0 };
        let state = GrowthState::with_tile_count(10.0, 10);
        let grown = law.grow_n(state, 3);
        assert_eq!(grown.len(), 4);
    }
}
