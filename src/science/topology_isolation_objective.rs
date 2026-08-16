use crate::science::ScienceObjective;
use rand::Rng;

#[derive(Clone)]
pub struct TopologyIsolationObjective;

impl TopologyIsolationObjective {
    // Target Joint Load Isolation and Stress Limits
    const TARGET_LOAD_GAP: f64 = 0.104561;
    const TARGET_STRESS_LIMIT: f64 = 0.293704;
}

// We optimize 15 structural connection weights/tensions (T = [f64; 15])
impl ScienceObjective<[f64; 15]> for TopologyIsolationObjective {
    fn evaluate_fitness(&self, candidate: &[f64; 15]) -> (u32, u32) {
        let mut simulated_load_gap = 0.0;
        let mut simulated_stress = 0.0;
        
        for (i, &weight) in candidate.iter().enumerate() {
            let decay = (-weight * (i as f64 * 0.1)).exp();
            simulated_load_gap += decay * 0.01;
            simulated_stress += (weight * 1.2).sin().abs() * 0.05;
        }

        let load_error = (simulated_load_gap - Self::TARGET_LOAD_GAP).abs();
        let stress_error = (simulated_stress - Self::TARGET_STRESS_LIMIT).abs();

        let total_mse = load_error.powi(2) + stress_error.powi(2);
        
        let fitness = (total_mse * 2000.0).max(0.0) as u32;

        (fitness, 0)
    }

    fn generate_seed(&self, _seed: usize, _parent: Option<&[f64; 15]>) -> [f64; 15] {
        let mut rng = rand::rng();
        let mut genes = [0.0; 15];
        for gene in &mut genes {
            *gene = rng.random_range(0.0..5.0);
        }
        genes
    }

    fn perturb(&self, candidate: &[f64; 15], scale: f32, _seed: usize) -> [f64; 15] {
        let mut rng = rand::rng();
        let mut new_candidate = *candidate;
        let idx = rng.random_range(0..15);
        let mutation = rng.random_range(-0.5..0.5) * scale as f64;
        new_candidate[idx] = (new_candidate[idx] + mutation).clamp(0.0, 5.0);
        new_candidate
    }

    fn is_valid(&self, candidate: &[f64; 15]) -> bool {
        candidate.iter().all(|&x| (0.0..=5.0).contains(&x))
    }

    fn check_archival(&self, _candidate: &[f64; 15], fitness: (u32, u32)) -> bool {
        fitness.0 < 40 
    }
}

