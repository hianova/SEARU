use crate::science::crucible::{Gene, TheCrucible};

pub struct MacroArranger;

impl MacroArranger {
    /// Evolves a cinematic 15-minute energy curve (0.0 to 1.0) for `total_bars`.
    pub fn evolve_energy_curve(total_bars: usize) -> Vec<f64> {
        println!(
            "🌊 Macro-Arranger: Evolving 1/f Pink Noise Energy Curve for {} bars...",
            total_bars
        );

        let num_control_points = 10;
        let mut genes = Vec::new();
        for i in 0..num_control_points {
            genes.push(Gene {
                name: format!("cp_{}", i),
                bounds: (0.0, 1.0),
                current_value: 0.1, // Start with low energy everywhere
            });
        }

        let golden_ratio_idx = (num_control_points as f64 * 0.618).round() as usize;

        let (_best_fitness, best_genes) = TheCrucible::anneal(
            genes,
            |current_genes| {
                let mut penalty = 0.0;

                // 1. Intro and Outro should be calm but NOT completely silent (target 0.1)
                penalty += (current_genes[0].current_value - 0.1).abs() * 50.0;
                penalty += (current_genes[num_control_points - 1].current_value - 0.1).abs() * 50.0;

                // 2. Golden Ratio Climax Reward
                // We want the peak energy to be right around 61.8% of the track.
                for (i, gene) in current_genes.iter().enumerate() {
                    let dist_to_golden = (i as f64 - golden_ratio_idx as f64).abs();
                    // If it's near the golden ratio, we WANT high energy (penalty for low energy)
                    if dist_to_golden <= 1.0 {
                        penalty += (1.0 - gene.current_value) * 100.0;
                    } else {
                        // Far from golden ratio, penalize excessively high energy to create contrast
                        penalty += gene.current_value * 10.0;
                    }
                }

                // 3. Tension and Release (1/f Pink Noise proxy)
                // We want smooth build-ups and sudden drops, not pure randomness.
                let mut total_variation = 0.0;
                for i in 1..num_control_points {
                    let diff = current_genes[i].current_value - current_genes[i - 1].current_value;
                    total_variation += diff.abs();

                    if diff < -0.4 {
                        // Reward a massive drop (The Drop) after a climax
                        penalty -= 20.0;
                    }
                }

                // Penalize zig-zag white noise (too much variation)
                if total_variation > 4.0 {
                    penalty += (total_variation - 4.0) * 50.0;
                }

                // 4. Require at least one moment of calm / breakdown (target 0.05, not 0.0)
                let min_energy = current_genes
                    .iter()
                    .map(|g| g.current_value)
                    .fold(f64::INFINITY, |a, b| a.min(b));
                penalty += (min_energy - 0.05).abs() * 30.0;

                penalty.max(0.0)
            },
            10000,
        );

        // Extract control points
        let control_points: Vec<f64> = best_genes.into_iter().map(|g| g.current_value).collect();

        // Interpolate control points into total_bars
        let mut final_curve = Vec::with_capacity(total_bars);
        for bar in 0..total_bars {
            let progress = bar as f64 / (total_bars - 1).max(1) as f64;
            let float_idx = progress * (num_control_points - 1) as f64;
            let idx0 = float_idx.floor() as usize;
            let idx1 = (idx0 + 1).min(num_control_points - 1);
            let frac = float_idx - idx0 as f64;

            // Cubic easing smoothstep
            let smooth_frac = frac * frac * (3.0 - 2.0 * frac);
            let interpolated =
                control_points[idx0] * (1.0 - smooth_frac) + control_points[idx1] * smooth_frac;
            // Clamp to 0.05 to prevent pure digital silence
            final_curve.push(interpolated.clamp(0.05, 1.0));
        }

        println!(
            "✅ Macro-Arranger: Energy Curve mapped to {} bars.",
            total_bars
        );
        final_curve
    }
}
