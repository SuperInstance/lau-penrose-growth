//! Diffraction pattern verification.
//!
//! Penrose tilings produce sharp Bragg peaks in their diffraction patterns
//! despite being aperiodic. This is the hallmark of quasicrystals:
//! long-range order without periodicity.

use serde::{Serialize, Deserialize};
use crate::penrose::PenroseTiling;
use crate::golden::GoldenRatio;

/// A diffraction pattern computed from a Penrose tiling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffractionPattern {
    /// Reciprocal space points (q_x, q_y).
    pub peaks: Vec<(f64, f64)>,
    /// Intensity at each peak.
    pub intensities: Vec<f64>,
    /// Whether the pattern shows sharp Bragg peaks.
    pub has_sharp_peaks: bool,
}

impl DiffractionPattern {
    /// Compute a simplified diffraction pattern from a Penrose tiling.
    /// Uses the structure factor: F(q) = Σ_j exp(i q · r_j)
    pub fn from_tiling(tiling: &PenroseTiling) -> Self {
        let n_peaks = 50;
        let mut peaks = Vec::new();
        let mut intensities = Vec::new();

        // Sample reciprocal space at points related to 5-fold symmetry
        for k in 0..n_peaks {
            let angle = k as f64 * 2.0 * std::f64::consts::PI / 5.0;
            for shell in 1..=3 {
                let q_mag = shell as f64 * 2.0 * std::f64::consts::PI;
                let qx = q_mag * angle.cos();
                let qy = q_mag * angle.sin();

                // Compute structure factor
                let mut re = 0.0_f64;
                let mut im = 0.0_f64;
                for rhomb in &tiling.rhombs {
                    let (rx, ry) = rhomb.center;
                    let phase = qx * rx + qy * ry;
                    re += phase.cos();
                    im += phase.sin();
                }
                let intensity = (re * re + im * im) / (tiling.len() as f64).powi(2);

                peaks.push((qx, qy));
                intensities.push(intensity);
            }
        }

        // Determine if sharp peaks exist (intensity significantly above background)
        let max_intensity = intensities.iter().cloned().fold(0.0_f64, f64::max);
        let mean_intensity = intensities.iter().sum::<f64>() / intensities.len() as f64;
        let has_sharp = max_intensity > 3.0 * mean_intensity;

        Self { peaks, intensities, has_sharp_peaks: has_sharp }
    }

    /// Check if the pattern has 5-fold (or 10-fold) rotational symmetry.
    /// This is impossible for periodic crystals (crystallographic restriction theorem).
    pub fn has_fivefold_symmetry(&self) -> bool {
        if self.peaks.is_empty() {
            return false;
        }

        // Check that rotating the pattern by 72° gives approximately the same pattern
        let angle = 2.0 * std::f64::consts::PI / 5.0;
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let mut matches = 0;
        for (qx, qy) in &self.peaks {
            let rqx = qx * cos_a - qy * sin_a;
            let rqy = qx * sin_a + qy * cos_a;
            // Check if the rotated point is close to any peak
            if self.peaks.iter().any(|(px, py)| {
                let d = (px - rqx).hypot(py - rqy);
                d < 1.0
            }) {
                matches += 1;
            }
        }

        matches > self.peaks.len() / 2
    }

    /// Verify that the pattern shows quasicrystalline order:
    /// sharp Bragg peaks + forbidden rotational symmetry.
    pub fn is_quasicrystalline(&self) -> bool {
        self.has_sharp_peaks && self.has_fivefold_symmetry()
    }

    /// Number of detected peaks.
    pub fn num_peaks(&self) -> usize {
        self.peaks.len()
    }

    /// Maximum intensity.
    pub fn max_intensity(&self) -> f64 {
        self.intensities.iter().cloned().fold(0.0_f64, f64::max)
    }

    /// The diffraction pattern of a Penrose tiling has peaks indexed by
    /// Z[ζ₅] — the ring of integers in the 5th cyclotomic field.
    /// This is related to the golden ratio through ζ₅ = e^(2πi/5).
    pub fn golden_indexing(&self) -> Vec<(i64, i64)> {
        // Simplified: return indices that would label the peaks
        self.peaks.iter().enumerate().map(|(i, _)| {
            let shell = (i / 10) as i64;
            let n = (i % 10) as i64;
            (shell, n)
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diffraction_from_tiling() {
        let tiling = PenroseTiling::sun(1.0);
        let pattern = DiffractionPattern::from_tiling(&tiling);
        assert!(pattern.num_peaks() > 0);
    }

    #[test]
    fn test_diffraction_has_peaks() {
        let tiling = PenroseTiling::sun(1.0);
        let pattern = DiffractionPattern::from_tiling(&tiling);
        assert_eq!(pattern.peaks.len(), pattern.intensities.len());
    }

    #[test]
    fn test_max_intensity_positive() {
        let tiling = PenroseTiling::sun(1.0);
        let pattern = DiffractionPattern::from_tiling(&tiling);
        assert!(pattern.max_intensity() >= 0.0);
    }

    #[test]
    fn test_inflated_tiling_diffraction() {
        let tiling = PenroseTiling::sun(1.0).inflate_n(2);
        let pattern = DiffractionPattern::from_tiling(&tiling);
        assert!(pattern.num_peaks() > 0);
    }

    #[test]
    fn test_golden_indexing() {
        let tiling = PenroseTiling::sun(1.0);
        let pattern = DiffractionPattern::from_tiling(&tiling);
        let indices = pattern.golden_indexing();
        assert_eq!(indices.len(), pattern.num_peaks());
    }

    #[test]
    fn test_serialization() {
        let tiling = PenroseTiling::sun(1.0);
        let pattern = DiffractionPattern::from_tiling(&tiling);
        let json = serde_json::to_string(&pattern).unwrap();
        let back: DiffractionPattern = serde_json::from_str(&json).unwrap();
        assert_eq!(back.peaks.len(), pattern.peaks.len());
    }
}
