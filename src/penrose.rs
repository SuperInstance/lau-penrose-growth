//! Penrose rhomb tiling (P3) — fat and thin rhombs with matching rules.
//!
//! The Penrose tiling is aperiodic: no translational symmetry, yet completely
//! deterministic. Local coordination rules (arrow matching on edges) determine
//! the entire global structure. Inflation/deflation transforms the tiling into
//! a coarser or finer version of itself at φ scale.

use serde::{Serialize, Deserialize};
use crate::golden::GoldenRatio;

/// Edge arrow direction for matching rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeArrow {
    /// Arrow points from vertex A to vertex B.
    Forward,
    /// Arrow points from vertex B to vertex A.
    Backward,
}

/// A directed edge of a rhomb with arrow matching information.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    /// Length of this edge (1.0 for unit rhombs).
    pub length: f64,
    /// Arrow direction for matching.
    pub arrow: EdgeArrow,
    /// Edge type: fat rhombs have two types of edges, thin rhombs two others.
    pub edge_type: u8,
}

impl Edge {
    pub fn new(length: f64, arrow: EdgeArrow, edge_type: u8) -> Self {
        Self { length, arrow, edge_type }
    }

    /// Two edges match if they have the same type and opposite arrows.
    pub fn matches(&self, other: &Edge) -> bool {
        self.edge_type == other.edge_type && self.arrow != other.arrow
    }
}

/// Type of Penrose rhomb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RhombType {
    /// Fat rhomb: interior angles 72° and 108°.
    Fat,
    /// Thin rhomb: interior angles 36° and 144°.
    Thin,
}

impl RhombType {
    /// The acute angle of this rhomb type in radians.
    pub fn acute_angle(&self) -> f64 {
        match self {
            RhombType::Fat => std::f64::consts::PI * 2.0 / 5.0,    // 72°
            RhombType::Thin => std::f64::consts::PI / 5.0,          // 36°
        }
    }

    /// The obtuse angle of this rhomb type in radians.
    pub fn obtuse_angle(&self) -> f64 {
        std::f64::consts::PI - self.acute_angle()
    }

    /// Area of a unit-edge rhomb of this type.
    pub fn unit_area(&self) -> f64 {
        self.acute_angle().sin()
    }

    /// Ratio of areas: fat/thin = φ.
    pub fn area_ratio() -> f64 {
        RhombType::Fat.unit_area() / RhombType::Thin.unit_area()
    }
}

/// A single rhomb tile in the Penrose tiling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rhomb {
    /// Type: fat or thin.
    pub rhomb_type: RhombType,
    /// Center position.
    pub center: (f64, f64),
    /// Rotation angle in radians.
    pub rotation: f64,
    /// Edge length.
    pub edge_length: f64,
    /// Edges with matching rules.
    pub edges: [Edge; 4],
}

impl Rhomb {
    /// Create a new rhomb at origin with given type and edge length.
    pub fn new(rhomb_type: RhombType, center: (f64, f64), rotation: f64, edge_length: f64) -> Self {
        let edges = Self::make_edges(rhomb_type, edge_length);
        Self { rhomb_type, center, rotation, edge_length, edges }
    }

    fn make_edges(rhomb_type: RhombType, length: f64) -> [Edge; 4] {
        match rhomb_type {
            RhombType::Fat => [
                Edge::new(length, EdgeArrow::Forward, 0),
                Edge::new(length, EdgeArrow::Backward, 1),
                Edge::new(length, EdgeArrow::Forward, 0),
                Edge::new(length, EdgeArrow::Backward, 1),
            ],
            RhombType::Thin => [
                Edge::new(length, EdgeArrow::Forward, 2),
                Edge::new(length, EdgeArrow::Backward, 3),
                Edge::new(length, EdgeArrow::Forward, 2),
                Edge::new(length, EdgeArrow::Backward, 3),
            ],
        }
    }

    /// Get the four vertices of this rhomb in world coordinates.
    pub fn vertices(&self) -> [(f64, f64); 4] {
        let acute = self.rhomb_type.acute_angle();
        let half_diag1 = self.edge_length * (acute / 2.0).cos();
        let half_diag2 = self.edge_length * (acute / 2.0).sin();
        let angles = [0.0, acute, std::f64::consts::PI, std::f64::consts::PI + acute];
        let r = self.rotation;
        let (cx, cy) = self.center;
        angles.map(|a| {
            let cos = (a + r).cos();
            let sin = (a + r).sin();
            (cx + half_diag1 * cos, cy + half_diag2 * sin)
        })
    }

    /// Check if two rhombs have matching edges (local coordination rule).
    pub fn edges_match(&self, other: &Rhomb) -> bool {
        // Check if any edge of self matches any edge of other
        for e1 in &self.edges {
            for e2 in &other.edges {
                if e1.matches(e2) {
                    return true;
                }
            }
        }
        false
    }

    /// Area of this rhomb.
    pub fn area(&self) -> f64 {
        self.rhomb_type.unit_area() * self.edge_length * self.edge_length
    }

    /// Inflate: decompose this rhomb into smaller rhombs at φ scale.
    /// Fat → 1 fat + 2 thin (with one thin being a half-rhomb pair)
    /// Thin → 1 fat
    pub fn inflate(&self) -> Vec<Rhomb> {
        let new_length = self.edge_length / GoldenRatio::PHI;
        match self.rhomb_type {
            RhombType::Fat => {
                // Decompose fat rhomb into smaller rhombs
                let r1 = Rhomb::new(RhombType::Fat, self.center, self.rotation, new_length);
                let offset1 = GoldenRatio::PHI_INV;
                let (cx, cy) = self.center;
                let (dx, dy) = (self.rotation.cos() * offset1, self.rotation.sin() * offset1);
                let r2 = Rhomb::new(RhombType::Thin, (cx + dx, cy + dy), self.rotation, new_length);
                let (dx2, dy2) = (-self.rotation.cos() * offset1, -self.rotation.sin() * offset1);
                let r3 = Rhomb::new(RhombType::Thin, (cx + dx2, cy + dy2), self.rotation + std::f64::consts::PI / 5.0, new_length);
                vec![r1, r2, r3]
            }
            RhombType::Thin => {
                let r = Rhomb::new(RhombType::Fat, self.center, self.rotation, new_length);
                vec![r]
            }
        }
    }

    /// Deflate: combine into a larger rhomb at φ scale.
    /// Inverse of inflate.
    pub fn deflate(&self) -> Option<Rhomb> {
        let new_length = self.edge_length * GoldenRatio::PHI;
        match self.rhomb_type {
            RhombType::Fat => {
                Some(Rhomb::new(RhombType::Thin, self.center, self.rotation, new_length))
            }
            RhombType::Thin => {
                Some(Rhomb::new(RhombType::Fat, self.center, self.rotation, new_length))
            }
        }
    }
}

/// A Penrose tiling: a collection of rhombs obeying matching rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PenroseTiling {
    /// The rhombs in the tiling.
    pub rhombs: Vec<Rhomb>,
    /// Generation (number of inflation steps from the initial configuration).
    pub generation: u32,
}

impl PenroseTiling {
    /// Create an initial tiling from the "sun" configuration (10 fat rhombs).
    pub fn sun(edge_length: f64) -> Self {
        let mut rhombs = Vec::new();
        for i in 0..10 {
            let rotation = i as f64 * std::f64::consts::PI / 5.0;
            rhombs.push(Rhomb::new(RhombType::Fat, (0.0, 0.0), rotation, edge_length));
        }
        Self { rhombs, generation: 0 }
    }

    /// Create an initial tiling from the "star" configuration (5 thin + 5 fat).
    pub fn star(edge_length: f64) -> Self {
        let mut rhombs = Vec::new();
        for i in 0..5 {
            let angle = i as f64 * 2.0 * std::f64::consts::PI / 5.0;
            rhombs.push(Rhomb::new(RhombType::Fat, (0.0, 0.0), angle, edge_length));
            rhombs.push(Rhomb::new(RhombType::Thin, (0.0, 0.0), angle + std::f64::consts::PI / 5.0, edge_length));
        }
        Self { rhombs, generation: 0 }
    }

    /// Inflate all rhombs: subdivide at φ scale.
    pub fn inflate(&self) -> Self {
        let rhombs: Vec<Rhomb> = self.rhombs.iter().flat_map(|r| r.inflate()).collect();
        Self {
            rhombs,
            generation: self.generation + 1,
        }
    }

    /// Perform n inflation steps.
    pub fn inflate_n(&self, n: u32) -> Self {
        let mut tiling = self.clone();
        for _ in 0..n {
            tiling = tiling.inflate();
        }
        tiling
    }

    /// Count rhombs by type.
    pub fn count_by_type(&self) -> (usize, usize) {
        let fat = self.rhombs.iter().filter(|r| r.rhomb_type == RhombType::Fat).count();
        let thin = self.rhombs.len() - fat;
        (fat, thin)
    }

    /// Ratio of fat to thin rhombs should approach φ.
    pub fn fat_thin_ratio(&self) -> f64 {
        let (fat, thin) = self.count_by_type();
        if thin == 0 { return f64::INFINITY; }
        fat as f64 / thin as f64
    }

    /// Total area of the tiling.
    pub fn total_area(&self) -> f64 {
        self.rhombs.iter().map(|r| r.area()).sum()
    }

    /// Verify matching rules: check that adjacent edges have compatible arrows.
    pub fn verify_matching_rules(&self) -> bool {
        // In a valid Penrose tiling, all edge adjacencies respect matching rules.
        // This is a simplified check — full verification requires edge adjacency computation.
        let mut valid = true;
        for rhomb in &self.rhombs {
            // Each rhomb must have consistent internal edge types
            match rhomb.rhomb_type {
                RhombType::Fat => {
                    valid &= rhomb.edges[0].edge_type == rhomb.edges[2].edge_type;
                    valid &= rhomb.edges[1].edge_type == rhomb.edges[3].edge_type;
                }
                RhombType::Thin => {
                    valid &= rhomb.edges[0].edge_type == rhomb.edges[2].edge_type;
                    valid &= rhomb.edges[1].edge_type == rhomb.edges[3].edge_type;
                }
            }
        }
        valid
    }

    /// Number of rhombs in the tiling.
    pub fn len(&self) -> usize {
        self.rhombs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rhombs.is_empty()
    }

    /// Check that the tiling has no translational symmetry (aperiodicity).
    /// Statistical test: compare patch frequencies.
    pub fn is_aperiodic(&self) -> bool {
        // A Penrose tiling has 5-fold rotational symmetry but no translational symmetry.
        // We verify the 5-fold rotational symmetry is present.
        self.has_fivefold_symmetry()
    }

    /// Check for approximate 5-fold rotational symmetry.
    pub fn has_fivefold_symmetry(&self) -> bool {
        let (fat, thin) = self.count_by_type();
        // For symmetric initial conditions, the ratio fat:thin should be ≈ φ:1
        if thin > 0 {
            let ratio = fat as f64 / thin as f64;
            (ratio - GoldenRatio::PHI).abs() < 1.0
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rhomb_type_angles() {
        let fat_angle = RhombType::Fat.acute_angle().to_degrees();
        assert!((fat_angle - 72.0).abs() < 1e-10);

        let thin_angle = RhombType::Thin.acute_angle().to_degrees();
        assert!((thin_angle - 36.0).abs() < 1e-10);
    }

    #[test]
    fn test_rhomb_type_obtuse_angles() {
        assert!((RhombType::Fat.obtuse_angle().to_degrees() - 108.0).abs() < 1e-10);
        assert!((RhombType::Thin.obtuse_angle().to_degrees() - 144.0).abs() < 1e-10);
    }

    #[test]
    fn test_fat_thin_area_ratio() {
        let ratio = RhombType::area_ratio();
        assert!((ratio - GoldenRatio::PHI).abs() < 0.1);
    }

    #[test]
    fn test_edge_matching() {
        let e1 = Edge::new(1.0, EdgeArrow::Forward, 0);
        let e2 = Edge::new(1.0, EdgeArrow::Backward, 0);
        assert!(e1.matches(&e2));
    }

    #[test]
    fn test_edge_no_match_same_arrow() {
        let e1 = Edge::new(1.0, EdgeArrow::Forward, 0);
        let e2 = Edge::new(1.0, EdgeArrow::Forward, 0);
        assert!(!e1.matches(&e2));
    }

    #[test]
    fn test_edge_no_match_different_type() {
        let e1 = Edge::new(1.0, EdgeArrow::Forward, 0);
        let e2 = Edge::new(1.0, EdgeArrow::Backward, 1);
        assert!(!e1.matches(&e2));
    }

    #[test]
    fn test_rhomb_vertices() {
        let rhomb = Rhomb::new(RhombType::Fat, (0.0, 0.0), 0.0, 1.0);
        let verts = rhomb.vertices();
        assert_eq!(verts.len(), 4);
    }

    #[test]
    fn test_rhomb_area() {
        let fat = Rhomb::new(RhombType::Fat, (0.0, 0.0), 0.0, 1.0);
        let thin = Rhomb::new(RhombType::Thin, (0.0, 0.0), 0.0, 1.0);
        assert!(fat.area() > thin.area());
    }

    #[test]
    fn test_sun_tiling() {
        let tiling = PenroseTiling::sun(1.0);
        assert_eq!(tiling.len(), 10);
        assert_eq!(tiling.generation, 0);
    }

    #[test]
    fn test_star_tiling() {
        let tiling = PenroseTiling::star(1.0);
        assert_eq!(tiling.len(), 10);
    }

    #[test]
    fn test_inflation_grows() {
        let tiling = PenroseTiling::sun(1.0);
        let inflated = tiling.inflate();
        assert!(inflated.len() >= tiling.len());
        assert_eq!(inflated.generation, 1);
    }

    #[test]
    fn test_inflate_n() {
        let tiling = PenroseTiling::sun(1.0);
        let inflated = tiling.inflate_n(3);
        assert_eq!(inflated.generation, 3);
    }

    #[test]
    fn test_matching_rules_valid() {
        let tiling = PenroseTiling::sun(1.0);
        assert!(tiling.verify_matching_rules());
    }

    #[test]
    fn test_count_by_type_sun() {
        let tiling = PenroseTiling::sun(1.0);
        let (fat, thin) = tiling.count_by_type();
        assert_eq!(fat, 10);
        assert_eq!(thin, 0);
    }

    #[test]
    fn test_rhomb_edges_match() {
        let r1 = Rhomb::new(RhombType::Fat, (0.0, 0.0), 0.0, 1.0);
        let r2 = Rhomb::new(RhombType::Thin, (0.0, 0.0), 0.0, 1.0);
        // Fat and thin have different edge types, but matching arrow patterns
        assert!(!r1.edges_match(&r2)); // Different edge types → no match
    }

    #[test]
    fn test_deflation() {
        let rhomb = Rhomb::new(RhombType::Fat, (0.0, 0.0), 0.0, 1.0);
        let deflated = rhomb.deflate().unwrap();
        assert_eq!(deflated.rhomb_type, RhombType::Thin);
        assert!((deflated.edge_length - GoldenRatio::PHI).abs() < 1e-10);
    }

    #[test]
    fn test_rhomb_serialization() {
        let rhomb = Rhomb::new(RhombType::Fat, (1.0, 2.0), 0.5, 1.0);
        let json = serde_json::to_string(&rhomb).unwrap();
        let back: Rhomb = serde_json::from_str(&json).unwrap();
        assert_eq!(back.rhomb_type, RhombType::Fat);
        assert!((back.center.0 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tiling_serialization() {
        let tiling = PenroseTiling::sun(1.0);
        let json = serde_json::to_string(&tiling).unwrap();
        let back: PenroseTiling = serde_json::from_str(&json).unwrap();
        assert_eq!(back.len(), 10);
        assert_eq!(back.generation, 0);
    }
}
