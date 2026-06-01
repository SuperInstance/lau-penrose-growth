# lau-penrose-growth

**Grow, don't calculate** — Penrose tilings, golden spirals, and shell growth as deterministic aperiodic structure without computation.

---

## What This Does

This crate implements the mathematical thesis that **structure can emerge from local growth laws rather than global computation**. It provides:

- **Golden ratio** mathematics — φ as the "most irrational number" and its role in optimal packing
- **Penrose rhomb tilings** (P3) — aperiodic tilings with inflation/deflation at φ scale
- **Logarithmic spirals** — growth by curvature, where each increment follows from what's already there
- **Conch shell growth** — self-similar spiral growth as a spatial recording of temporal dynamics (Mandelbrot's insight)
- **Aperiodic sequences** — Fibonacci words and 1D Penrose projections
- **Diffraction patterns** — quasicrystalline Bragg peaks with forbidden 5-fold symmetry
- **Agent capability shells** — skills positioned along a golden spiral, growing like a snail

The crate contains **114 tests** verifying all mathematical properties.

---

## Key Idea

> *The growth law IS the constructor.*

A snail doesn't calculate where to grow next. The curvature of its existing shell determines the next increment — growth by geometric consequence. The golden ratio φ governs this because φ is the most irrational number (continued fraction `[1; 1, 1, ...]` converges slowest), producing the most uniform spatial coverage without periodic repetition.

In the **Constitutive Computation** framework, the growth law acts as an idempotent `e` (where `e ∘ e = e`). The grown structure is the image `S = im(e)`. The growth process itself is the computation — no separate calculation step is needed.

---

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
lau-penrose-growth = { git = "https://github.com/SuperInstance/lau-penrose-growth" }
```

Or clone directly:

```bash
git clone https://github.com/SuperInstance/lau-penrose-growth.git
cd lau-penrose-growth
cargo build
```

### Dependencies

- `serde` 1.x (with `derive`) — serialization for all types
- `nalgebra` 0.33 — linear algebra support

### Dev Dependencies

- `serde_json` 1.x — JSON round-trip tests

---

## Quick Start

```rust
use lau_penrose_growth::{GoldenRatio, PenroseTiling, LogarithmicSpiral, ConchShell};

// Golden ratio
println!("φ = {:.10}", GoldenRatio::PHI);         // 1.6180339887...
println!("Golden angle = {:.3}°", GoldenRatio::GOLDEN_ANGLE_DEG);  // ~137.508°

// Sunflower phyllotaxis — golden angle packing
let points = GoldenRatio::phyllotaxis(100, 1.0);
let quality = GoldenRatio::packing_quality(&points);

// Penrose tiling from "sun" seed
let tiling = PenroseTiling::sun(1.0);    // 10 fat rhombs
let inflated = tiling.inflate_n(3);       // inflate 3 generations
let (fat, thin) = inflated.count_by_type();

// Golden spiral
let spiral = LogarithmicSpiral::golden(1.0);
let r = spiral.radius(std::f64::consts::PI / 2.0);  // radius at quarter turn ≈ φ

// Shell growth
let mut shell = ConchShell::new(0.1);
shell.grow_n(50);
println!("Self-similar: {}", shell.is_self_similar(0.1));  // true
```

Run the demo binary:

```bash
cargo run
```

Run all 114 tests:

```bash
cargo test
```

---

## API Reference

### `GoldenRatio`

Constants and functions for the golden ratio φ = (1+√5)/2.

| Item | Description |
|------|-------------|
| `PHI` | φ ≈ 1.6180339887 |
| `PHI_INV` | 1/φ ≈ 0.6180339887 |
| `PHI_SQ` | φ² ≈ 2.6180339887 |
| `GOLDEN_ANGLE` | 2π/φ² in radians (optimal angular spacing) |
| `GOLDEN_ANGLE_DEG` | ≈ 137.508° |
| `continued_fraction(depth)` | Returns `[1; 1, 1, ...]` — all 1s = slowest convergence |
| `evaluate_cf(coeffs)` | Evaluate a continued fraction to a rational (p, q) |
| `rational_approximations(n)` | Successive convergents F(n+1)/F(n) → φ |
| `fibonacci(n)` | nth Fibonacci number (0-indexed) |
| `phyllotaxis(n, scale)` | n points on a disk using golden angle (Fermat's spiral) |
| `packing_quality(points)` | Minimum pairwise distance (higher = more uniform) |
| `irrationality_measure(x, max_q)` | Hurwitz-type irrationality measure |

### `PenroseTiling` and `Rhomb`

Penrose P3 rhomb tiling with fat (72°/108°) and thin (36°/144°) rhombs.

| Item | Description |
|------|-------------|
| `RhombType::Fat` / `::Thin` | Fat (72° acute) or thin (36° acute) rhomb |
| `RhombType::area_ratio()` | fat/thin area ratio = φ |
| `Rhomb::new(type, center, rotation, edge_length)` | Create a rhomb with matching-rule edges |
| `Rhomb::inflate()` | Subdivide into smaller rhombs at 1/φ scale |
| `Rhomb::deflate()` | Combine into a larger rhomb at φ scale |
| `PenroseTiling::sun(edge)` | Initial tiling: 10 fat rhombs (sun config) |
| `PenroseTiling::star(edge)` | Initial tiling: 5 fat + 5 thin (star config) |
| `.inflate()` / `.inflate_n(n)` | Inflate the entire tiling |
| `.count_by_type()` | Returns `(fat_count, thin_count)` |
| `.fat_thin_ratio()` | Approaches φ as generation increases |
| `.verify_matching_rules()` | Check internal edge consistency |
| `.is_aperiodic()` | Statistical aperiodicity check |

### `LogarithmicSpiral`

Equiangular spiral r(θ) = a·e^(bθ).

| Item | Description |
|------|-------------|
| `LogarithmicSpiral::golden(a)` | Golden spiral (growth per quarter turn = φ) |
| `::with_angle(a, alpha)` | Spiral with constant angle α |
| `.radius(θ)` / `.point(θ)` | Radius and (x, y) at angle θ |
| `.constant_angle()` | Angle between radius and tangent (constant) |
| `.arc_length(θ₁, θ₂)` | Arc length between two angles |
| `.curvature(θ)` | Curvature κ = sin(α) / r(θ) |
| `.growth_per_turn()` | How much the radius increases per full rotation |
| `.is_golden(tol)` | Check if growth per quarter turn ≈ φ |

### `ConchShell`

Self-similar shell growth along a golden spiral.

| Item | Description |
|------|-------------|
| `ConchShell::new(radius)` | Golden-growth shell with initial radius |
| `.grow()` / `.grow_n(n)` | Grow by one or n increments |
| `.current_radius()` / `.aperture_width()` | Current size measurements |
| `.current_curvature()` | Curvature at the growth front |
| `.is_self_similar(tol)` | Verify approximately constant growth rate |
| `.spatial_temporal_record()` | Full growth history (spatial = temporal) |
| `.growth_law_is_constructor()` | Always `true` — the thesis of this crate |

### `FibonacciSequence` and `PenroseSequence`

Aperiodic deterministic sequences.

| Item | Description |
|------|-------------|
| `FibonacciSequence::new()` | Start from "0" |
| `.substitute()` / `.substitute_n(n)` | Apply substitution rules (0→01, 1→0) |
| `.counts()` | (zeros, ones) — ratio approaches φ |
| `.ratio()` | 0s/1s → φ |
| `.is_aperiodic()` | Verifies no exact translational period |
| `.contains(pattern)` | Pattern matching (note: "11" and "000" never appear) |
| `PenroseSequence::new(n)` | 1D projection of Penrose tiling via L→LS, S→L |
| `.total_length()` / `.counts()` / `.ratio()` | Length statistics |

### `DiffractionPattern`

Structure factor analysis of Penrose tilings.

| Item | Description |
|------|-------------|
| `DiffractionPattern::from_tiling(tiling)` | Compute diffraction from tiling vertices |
| `.has_sharp_peaks` | Whether Bragg peaks exceed background |
| `.has_fivefold_symmetry()` | Forbidden rotational symmetry check |
| `.is_quasicrystalline()` | Sharp peaks + 5-fold symmetry |
| `.golden_indexing()` | Peak indices in Z[ζ₅] |

### `AgentShell`

Agent capabilities modeled as a growing conch spiral.

| Item | Description |
|------|-------------|
| `AgentShell::new(first_skill)` | Create with an initial skill at origin |
| `.grow_skill(name, level)` | Add a skill at golden-angle position |
| `.shell_radius()` | Distance to outermost skill |
| `.nearest_skill(point)` | Find closest skill in capability space |
| `.capability_density()` | Skills per unit area |
| `.uses_golden_spacing()` | Verify golden-angle angular spacing |

---

## How It Works

### Growth Laws as Constructors

Each growth law in this crate is **constitutive**: the structure emerges from the growth process itself, not from a separate computation. The key relationships:

1. **Golden ratio → optimal packing**: φ has continued fraction `[1; 1, 1, ...]`, meaning it is worst-case approximable by rationals. This produces the most uniform angular spacing (golden angle ≈ 137.508°) without ever repeating.

2. **Penrose matching rules → aperiodic order**: Local edge-arrow matching rules (edges must have same type and opposite arrows) determine the global tiling. Inflation by φ scales the tiling while preserving the same structure.

3. **Logarithmic spirals → self-similar growth**: The constant angle between radius and tangent means each increment is geometrically determined by the previous shape. The golden spiral grows by factor φ per quarter turn.

4. **Substitution sequences → quasicrystals**: The Fibonacci word (0→01, 1→0) and Penrose sequence (L→LS, S→L) produce aperiodic sequences where the ratio of symbol frequencies converges to φ.

### Constitutive Computation Connection

In the Constitutive Computation framework:

| Growth Concept | Constitutive Concept |
|---|---|
| Growth law | Idempotent `e` (e∘e = e) |
| Grown structure | Image `S = im(e)` |
| Fixed point of growth | Element of the image |
| Growth process | The computation itself |

The golden ratio is the **fixed point** of x → 1 + 1/x. Iterating this from any positive starting value converges to φ. At the fixed point, applying the function again gives the same value — idempotency.

---

## The Math

### The Golden Ratio as Most Irrational

A number's irrationality is measured by how well rational numbers approximate it. The Hurwitz theorem says: for any irrational x, there exist infinitely many p/q with |x - p/q| < 1/(√5 · q²). The constant √5 is optimal and achieved only by φ and its relatives.

The continued fraction [a₀; a₁, a₂, ...] converges faster when the aᵢ are larger. φ = [1; 1, 1, 1, ...] has all coefficients equal to 1 — the slowest possible convergence. This means φ "avoids" rational approximations most effectively.

### Penrose Tiling Properties

The P3 Penrose tiling uses two rhomb types:
- **Fat rhomb**: angles 72° and 108°, area = sin(72°)
- **Thin rhomb**: angles 36° and 144°, area = sin(36°)
- Area ratio: sin(72°)/sin(36°) = 2cos(36°) ≈ 1.618... = φ

Inflation decomposes a fat rhomb into 1 fat + 2 thin, and a thin rhomb into 1 fat. After many inflations, the ratio fat:thin approaches φ:1.

### Quasicrystal Diffraction

Penrose tilings produce sharp Bragg peaks in diffraction despite being aperiodic. This was experimentally observed by Dan Shechtman in 1982 (Nobel Prize 2011). The peaks exhibit 5-fold rotational symmetry, which is **forbidden** for periodic crystals by the crystallographic restriction theorem.

The diffraction peaks are indexed by Z[ζ₅] — the ring of integers in the 5th cyclotomic field Q(e^{2πi/5}), which is deeply connected to φ through ζ₅ + ζ₅⁻¹ = 2cos(72°) = 1/φ.

### Fibonacci Word Properties

The Fibonacci word is a Sturmian word — it has exactly n+1 distinct substrings of length n. Key forbidden patterns:
- "11" never appears
- "000" never appears

The ratio |word₀|/|word₁| → φ as generation → ∞.

---

## License

MIT
