//! Binary entry point for lau-penrose-growth.
//!
//! Demonstrates the principle: grow, don't calculate.

use lau_penrose_growth::{GoldenRatio, PenroseTiling, LogarithmicSpiral, ConchShell};

fn main() {
    println!("=== Grow, Don't Calculate ===\n");

    // Golden ratio
    println!("φ = {:.10}", GoldenRatio::PHI);
    println!("1/φ = {:.10}", GoldenRatio::PHI_INV);
    println!("Golden angle = {:.3}°\n", GoldenRatio::GOLDEN_ANGLE_DEG);

    // Penrose tiling
    let sun = PenroseTiling::sun(1.0);
    println!("Sun tiling: {} rhombs (generation 0)", sun.len());

    let inflated = sun.inflate_n(3);
    let (fat, thin) = inflated.count_by_type();
    println!("After 3 inflations: {} rhombs ({} fat, {} thin)", inflated.len(), fat, thin);

    if thin > 0 {
        println!("Fat/thin ratio: {:.4} (φ = {:.4})", inflated.fat_thin_ratio(), GoldenRatio::PHI);
    }
    println!();

    // Golden spiral
    let spiral = LogarithmicSpiral::golden(1.0);
    println!("Golden spiral: angle = {:.4}°", spiral.constant_angle().to_degrees());
    println!("Growth per turn: {:.4}×\n", spiral.growth_per_turn());

    // Shell growth
    let mut shell = ConchShell::new(0.1);
    shell.grow_n(50);
    println!("Shell after 50 increments:");
    println!("  Current radius: {:.4}", shell.current_radius());
    println!("  Growth rate: {:.4}", shell.growth_rate());
    println!("  Self-similar: {}", shell.is_self_similar(0.1));

    println!("\n=== The growth law IS the constructor ===");
}
