//! The generic Crucible for optimizing any parameter space via Monte Carlo Annealing.

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Gene {
    pub name: String,
    pub bounds: (f64, f64),
    pub current_value: f64,
}

pub struct TheCrucible;

impl TheCrucible {
    /// Takes a set of genes, an evaluation function, and optimizes them
    /// using Simulated Annealing.
    pub fn anneal<F>(mut genes: Vec<Gene>, mut evaluate: F, iterations: usize) -> (f64, Vec<Gene>)
    where
        F: FnMut(&[Gene]) -> f64,
    {
        println!("🔥 The Crucible: Igniting Simulated Annealing for {} iterations...", iterations);
        
        let initial_temp = 100.0_f64;
        let final_temp = 0.001_f64;
        let cooling_rate = (final_temp / initial_temp).powf(1.0 / (iterations as f64));

        let mut current_fitness = evaluate(&genes);
        let mut best_fitness = current_fitness;
        let mut best_genes = genes.clone();
        
        let mut current_temp = initial_temp;

        for _ in 0..iterations {
            let mut candidate_genes = genes.clone();
            
            // Perturb genes
            for gene in &mut candidate_genes {
                let range = gene.bounds.1 - gene.bounds.0;
                // Step size scales with temperature, but has a minimum bound
                let max_step = range * (current_temp / initial_temp).max(0.05); 
                let step = (rand::random::<f64>() - 0.5) * max_step;
                gene.current_value = (gene.current_value + step).clamp(gene.bounds.0, gene.bounds.1);
            }

            let candidate_fitness = evaluate(&candidate_genes);
            
            // Acceptance probability
            if candidate_fitness < current_fitness {
                genes = candidate_genes.clone();
                current_fitness = candidate_fitness;
                
                if current_fitness < best_fitness {
                    best_fitness = current_fitness;
                    best_genes = candidate_genes;
                }
            } else {
                let diff = candidate_fitness - current_fitness;
                // To avoid NaN/Infinity if diff is too large and temp is tiny
                let acceptance_prob = (-diff / current_temp).exp();
                if rand::random::<f64>() < acceptance_prob {
                    genes = candidate_genes;
                    current_fitness = candidate_fitness;
                }
            }

            current_temp *= cooling_rate;
        }
        
        (best_fitness, best_genes)
    }
}
