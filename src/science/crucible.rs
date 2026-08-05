//! The generic Crucible for optimizing any parameter space via Monte Carlo Annealing.

use std::collections::HashMap;

/// A generic gene that needs optimization.
pub struct Gene {
    pub name: String,
    pub bounds: (f64, f64),
    pub current_value: f64,
}

pub struct TheCrucible;

impl TheCrucible {
    /// Takes a set of genes, an evaluation function, and optimizes them.
    /// Returns the best fitness and the optimized genes.
    pub fn anneal<F>(mut genes: Vec<Gene>, mut evaluate: F, iterations: usize) -> (f64, Vec<Gene>)
    where
        F: FnMut(&[Gene]) -> f64,
    {
        println!("🔥 The Crucible: Igniting Monte Carlo Annealing for {} iterations...", iterations);
        let mut best_fitness = f64::MAX;
        let mut best_genes = Vec::new();
        
        // Fast mock annealing
        for _ in 0..iterations {
            let fitness = evaluate(&genes);
            if fitness < best_fitness {
                best_fitness = fitness;
                best_genes = genes.iter().map(|g| Gene {
                    name: g.name.clone(),
                    bounds: g.bounds,
                    current_value: g.current_value,
                }).collect();
            }
            // Perturb genes
            for gene in &mut genes {
                let range = gene.bounds.1 - gene.bounds.0;
                let step = (rand::random::<f64>() - 0.5) * range * 0.1;
                gene.current_value = (gene.current_value + step).clamp(gene.bounds.0, gene.bounds.1);
            }
        }
        (best_fitness, best_genes)
    }
}
