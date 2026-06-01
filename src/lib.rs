//! # lau-penrose-growth
//!
//! **Grow, don't calculate** — Penrose tilings, golden spirals, and shell growth
//! as deterministic aperiodic structure without computation.
//!
//! The Penrose tiling is aperiodic (no translational symmetry) but completely
//! deterministic (local coordination rules determine the whole). A snail shell
//! grows by the curvature of what's already there — each whorl is the logical
//! consequence of the previous geometry. The golden ratio φ governs growth
//! because φ is the most irrational number (slowest rational approximation =
//! most uniform packing).
//!
//! This crate formalizes "grow, don't calculate" — structure that emerges from
//! local growth laws, not global computation.

pub mod golden;
pub mod penrose;
pub mod spiral;
pub mod shell;
pub mod growth;
pub mod constitutive;
pub mod diffraction;
pub mod agent;
pub mod sequence;

pub use golden::GoldenRatio;
pub use penrose::{PenroseTiling, Rhomb, RhombType, Edge, EdgeArrow};
pub use spiral::LogarithmicSpiral;
pub use shell::ConchShell;
pub use growth::GrowthLaw;
pub use constitutive::ConstitutiveProjection;
pub use diffraction::DiffractionPattern;
pub use agent::AgentShell;
pub use sequence::{FibonacciSequence, PenroseSequence};
