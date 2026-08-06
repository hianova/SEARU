use crate::science::crucible::{Gene, TheCrucible};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct AnimationCurve {
    pub keyframes: Vec<f64>,
}

pub struct AnimationOptimizer;

impl AnimationOptimizer {
    pub fn optimize_transition() -> AnimationCurve {
        let mut genes = Vec::new();
        for i in 1..=5 {
            genes.push(Gene {
                name: format!("K{}", i),
                bounds: (0.0, 1.0),
                current_value: (i as f64) / 6.0,
            });
        }

        let (_, best_genes) = TheCrucible::anneal(
            genes,
            |g| {
                let mut vals = vec![0.0];
                for i in 0..5 {
                    vals.push(g[i].current_value);
                }
                vals.push(1.0);

                let mut cost = 0.0;
                // Penalize if not monotonically increasing
                for i in 0..6 {
                    if vals[i + 1] < vals[i] {
                        cost += (vals[i] - vals[i + 1]) * 1000.0;
                    }
                }

                // Target an ease-in-out curve (S-curve)
                let s_curve = [0.0, 0.1, 0.3, 0.7, 0.9, 1.0];
                for i in 1..=5 {
                    cost += (vals[i] - s_curve[i]).powi(2) * 100.0;
                }

                cost
            },
            5000,
        );

        let mut kfs = vec![0.0];
        for g in best_genes {
            kfs.push(g.current_value);
        }
        kfs.push(1.0);

        AnimationCurve { keyframes: kfs }
    }
}
