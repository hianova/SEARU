use crate::science::crucible::{Gene, TheCrucible};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct LayoutNode {
    pub id: String,
    pub margin: f64,
    pub padding: f64,
    pub flex_grow: f64,
}

pub struct UiOptimizer;

impl UiOptimizer {
    pub fn optimize() -> Vec<LayoutNode> {
        let mut genes = Vec::new();
        for i in 0..3 {
            genes.push(Gene { name: format!("M{}", i), bounds: (0.0, 50.0), current_value: 10.0 });
            genes.push(Gene { name: format!("P{}", i), bounds: (0.0, 50.0), current_value: 10.0 });
            genes.push(Gene { name: format!("F{}", i), bounds: (1.0, 5.0), current_value: 1.0 });
        }
        
        let (_, best_genes) = TheCrucible::anneal(
            genes,
            |g| {
                let mut cost = 0.0;
                let target_ratio = [1.0, 2.0, 1.0]; 
                for i in 0..3 {
                    cost += (g[i*3+2].current_value - target_ratio[i]).powi(2) * 100.0;
                    cost += (g[i*3].current_value - 16.0).abs(); // Prefer 16px margin
                }
                cost
            },
            5000
        );
        
        let mut result = Vec::new();
        for i in 0..3 {
            result.push(LayoutNode {
                id: format!("Component_{}", i),
                margin: best_genes[i*3].current_value.round(),
                padding: best_genes[i*3+1].current_value.round(),
                flex_grow: (best_genes[i*3+2].current_value * 10.0).round() / 10.0,
            });
        }
        result
    }
}
