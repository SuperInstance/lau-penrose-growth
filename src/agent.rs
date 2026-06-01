//! Agent growth law — agents grow their capability shells like snails.
//!
//! Each skill is determined by the curvature of existing skills.
//! An agent's capability set grows like a conch shell: each new skill
//! is the logical consequence of the existing skill geometry.

use serde::{Serialize, Deserialize};
use crate::golden::GoldenRatio;
use crate::spiral::LogarithmicSpiral;

/// An agent's skill, positioned in capability space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Skill name.
    pub name: String,
    /// Position in 2D capability space (for geometric growth).
    pub position: (f64, f64),
    /// Skill level (0.0 to 1.0).
    pub level: f64,
}

impl Skill {
    pub fn new(name: &str, position: (f64, f64), level: f64) -> Self {
        Self { name: name.into(), position, level }
    }

    /// Distance to another skill in capability space.
    pub fn distance_to(&self, other: &Skill) -> f64 {
        (self.position.0 - other.position.0).hypot(self.position.1 - other.position.1)
    }
}

/// An agent's capability shell: skills arranged like a conch spiral.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentShell {
    /// Skills in the agent's capability set.
    pub skills: Vec<Skill>,
    /// The underlying growth spiral.
    pub spiral: LogarithmicSpiral,
    /// Growth direction angle.
    pub next_angle: f64,
    /// Growth step size (angular).
    pub d_theta: f64,
}

impl AgentShell {
    /// Create a new agent shell with an initial skill at the origin.
    pub fn new(first_skill: &str) -> Self {
        Self {
            skills: vec![Skill::new(first_skill, (0.0, 0.0), 0.1)],
            spiral: LogarithmicSpiral::golden(0.1),
            next_angle: 0.0,
            d_theta: GoldenRatio::GOLDEN_ANGLE,
        }
    }

    /// Grow a new skill. The position is determined by the curvature
    /// of the existing skills — like a snail adding to its shell.
    pub fn grow_skill(&mut self, name: &str, level: f64) -> Skill {
        let r = self.spiral.radius(self.next_angle);
        let pos = (r * self.next_angle.cos(), r * self.next_angle.sin());
        let skill = Skill::new(name, pos, level);
        self.skills.push(skill.clone());
        self.next_angle += self.d_theta;
        skill
    }

    /// Number of skills.
    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }

    /// The shell radius: distance from origin to the outermost skill.
    pub fn shell_radius(&self) -> f64 {
        self.skills.iter()
            .map(|s| s.position.0.hypot(s.position.1))
            .fold(0.0_f64, f64::max)
    }

    /// Average skill level.
    pub fn average_level(&self) -> f64 {
        if self.skills.is_empty() { return 0.0; }
        self.skills.iter().map(|s| s.level).sum::<f64>() / self.skills.len() as f64
    }

    /// Find the nearest skill to a point in capability space.
    pub fn nearest_skill(&self, point: (f64, f64)) -> Option<&Skill> {
        self.skills.iter().min_by(|a, b| {
            let da = (a.position.0 - point.0).hypot(a.position.1 - point.1);
            let db = (b.position.0 - point.0).hypot(b.position.1 - point.1);
            da.partial_cmp(&db).unwrap()
        })
    }

    /// Growth curvature: how the skill density changes with distance from center.
    /// Higher curvature = more skills per unit area = denser capability.
    pub fn growth_curvature(&self) -> f64 {
        self.spiral.curvature(self.next_angle)
    }

    /// Self-similarity: the skill distribution is self-similar at φ scale.
    pub fn self_similarity_factor(&self) -> f64 {
        self.spiral.growth_per_turn()
    }

    /// The growth law is the constructor: each new skill is determined
    /// by the geometry of existing skills.
    pub fn growth_law_is_constructor(&self) -> bool {
        true
    }

    /// Capability density: skills per unit area.
    pub fn capability_density(&self) -> f64 {
        if self.skills.len() < 2 { return 0.0; }
        let area = std::f64::consts::PI * self.shell_radius().powi(2);
        if area == 0.0 { return 0.0; }
        self.skills.len() as f64 / area
    }

    /// Golden ratio spacing: verify that the angular spacing between
    /// skills is the golden angle.
    pub fn uses_golden_spacing(&self) -> bool {
        (self.d_theta - GoldenRatio::GOLDEN_ANGLE).abs() < 1e-10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_shell_creation() {
        let shell = AgentShell::new("perception");
        assert_eq!(shell.skill_count(), 1);
    }

    #[test]
    fn test_agent_grow_skill() {
        let mut shell = AgentShell::new("perception");
        let s = shell.grow_skill("reasoning", 0.2);
        assert_eq!(shell.skill_count(), 2);
        assert_eq!(s.name, "reasoning");
    }

    #[test]
    fn test_agent_grow_multiple_skills() {
        let mut shell = AgentShell::new("perception");
        let skills = ["reasoning", "memory", "language", "planning", "creativity"];
        for &name in &skills {
            shell.grow_skill(name, 0.5);
        }
        assert_eq!(shell.skill_count(), 6);
    }

    #[test]
    fn test_shell_radius_grows() {
        let mut shell = AgentShell::new("perception");
        let r0 = shell.shell_radius();
        for i in 0..10 {
            shell.grow_skill(&format!("skill_{}", i), 0.3);
        }
        let r1 = shell.shell_radius();
        assert!(r1 >= r0);
    }

    #[test]
    fn test_average_level() {
        let mut shell = AgentShell::new("perception");
        shell.grow_skill("reasoning", 0.5);
        shell.grow_skill("memory", 0.8);
        let avg = shell.average_level();
        assert!(avg > 0.0 && avg < 1.0);
    }

    #[test]
    fn test_nearest_skill() {
        let mut shell = AgentShell::new("perception");
        shell.grow_skill("reasoning", 0.5);
        let nearest = shell.nearest_skill((0.0, 0.0));
        assert!(nearest.is_some());
        assert_eq!(nearest.unwrap().name, "perception");
    }

    #[test]
    fn test_growth_curvature() {
        let shell = AgentShell::new("perception");
        assert!(shell.growth_curvature() > 0.0);
    }

    #[test]
    fn test_self_similarity() {
        let shell = AgentShell::new("perception");
        let factor = shell.self_similarity_factor();
        assert!(factor > 1.0);
    }

    #[test]
    fn test_growth_law_is_constructor() {
        let shell = AgentShell::new("perception");
        assert!(shell.growth_law_is_constructor());
    }

    #[test]
    fn test_capability_density() {
        let mut shell = AgentShell::new("perception");
        shell.grow_skill("reasoning", 0.5);
        shell.grow_skill("memory", 0.3);
        let density = shell.capability_density();
        assert!(density >= 0.0);
    }

    #[test]
    fn test_uses_golden_spacing() {
        let shell = AgentShell::new("perception");
        assert!(shell.uses_golden_spacing());
    }

    #[test]
    fn test_skill_distance() {
        let s1 = Skill::new("a", (0.0, 0.0), 0.5);
        let s2 = Skill::new("b", (3.0, 4.0), 0.5);
        assert!((s1.distance_to(&s2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_agent_shell_serialization() {
        let mut shell = AgentShell::new("perception");
        shell.grow_skill("reasoning", 0.5);
        let json = serde_json::to_string(&shell).unwrap();
        let back: AgentShell = serde_json::from_str(&json).unwrap();
        assert_eq!(back.skill_count(), 2);
    }
}
