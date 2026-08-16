use crate::science::ScienceObjective;
use rand::Rng;

#[derive(Clone)]
pub struct MetamaterialObjective;

impl MetamaterialObjective {
    // 32-Dimensional Acoustic/Shock Void Target Matrix
    const TARGET_VOID_SPECTRUM: [f64; 32] = [
        3.84, -4.5509996, 4.7208, -4.4534006, -0.68, 0.022600003, 2.78, 3.0142, 
        -0.24, -1.37, -3.7832, -3.43, -4.9512, -13.29, 1.8088, -2.5544, 
        2.56, -4.1606, 0.532, -4.1848, 3.7256, 0.21679999, 1.42, 4.37, 
        2.8636, 4.9129996, -2.78, -1.91, 2.6391997, 1.173, -4.4136, -1.6322
    ];
}

// Optimizing 32 geometric parameters defining internal acoustic void micro-structures
impl ScienceObjective<[f64; 32]> for MetamaterialObjective {
    fn evaluate_fitness(&self, candidate: &[f64; 32]) -> (u32, u32) {
        let mut total_mse = 0.0;
        
        for (i, &void_dim) in candidate.iter().enumerate() {
            let structural_reflection = (void_dim * 1.5).sin() + (void_dim * std::f64::consts::PI).cos();
            let target_response = Self::TARGET_VOID_SPECTRUM[i];
            
            total_mse += (structural_reflection - target_response).powi(2);
        }

        let fitness = (total_mse * 100.0).max(0.0) as u32;

        (fitness, 0)
    }

    fn generate_seed(&self, _seed: usize, _parent: Option<&[f64; 32]>) -> [f64; 32] {
        let mut rng = rand::rng();
        let mut genes = [0.0; 32];
        for gene in &mut genes {
            *gene = rng.random_range(-15.0..15.0);
        }
        genes
    }

    fn perturb(&self, candidate: &[f64; 32], scale: f32, _seed: usize) -> [f64; 32] {
        let mut rng = rand::rng();
        let mut new_candidate = *candidate;
        for _ in 0..4 {
            let idx = rng.random_range(0..32);
            let mutation = (rng.random_range(-1.0..1.0) * scale as f64) * 2.0;
            new_candidate[idx] = (new_candidate[idx] + mutation).clamp(-15.0, 15.0);
        }
        new_candidate
    }

    fn is_valid(&self, candidate: &[f64; 32]) -> bool {
        candidate.iter().all(|&x| (-15.0..=15.0).contains(&x))
    }

    fn check_archival(&self, _candidate: &[f64; 32], fitness: (u32, u32)) -> bool {
        fitness.0 < 500
    }
}

