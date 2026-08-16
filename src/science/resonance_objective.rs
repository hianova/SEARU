use crate::science::ScienceObjective;
use rand::Rng;

#[derive(Clone)]
pub struct ResonanceObjective;

impl ResonanceObjective {
    // Target Spacing Density Function (Vibration Damping Distribution)
    // target_prob(x) = 3.290 * x^2 * exp(-0.785 * x^2)
    fn expected_density(x: f64) -> f64 {
        3.290 * x.powi(2) * (-0.785 * x.powi(2)).exp()
    }
}

// Optimizing 10 truss segment spacings (T = [f64; 10])
impl ScienceObjective<[f64; 10]> for ResonanceObjective {
    fn evaluate_fitness(&self, candidate: &[f64; 10]) -> (u32, u32) {
        let sorted = *candidate;

        let mut total_prob = 0.0;
        for &spacing in sorted.iter() {
            let prob = Self::expected_density(spacing);
            total_prob += prob;
        }

        // Add repulsion penalty (prevent identical spacing cluster)
        let mut repulsion_penalty = 0.0;
        for i in 0..9 {
            for j in (i+1)..10 {
                let diff = (sorted[i] - sorted[j]).abs();
                if diff < 0.1 {
                    repulsion_penalty += (0.1 - diff) * 10.0;
                }
            }
        }

        let base_score = 1000.0 - (total_prob * 100.0);
        let final_score = (base_score + repulsion_penalty * 100.0).max(0.0) as u32;

        (final_score, 0)
    }

    fn generate_seed(&self, _seed: usize, _parent: Option<&[f64; 10]>) -> [f64; 10] {
        let mut rng = rand::rng();
        let mut genes = [0.0; 10];
        for gene in &mut genes {
            *gene = rng.random_range(0.1..2.5);
        }
        genes
    }

    fn perturb(&self, candidate: &[f64; 10], scale: f32, _seed: usize) -> [f64; 10] {
        let mut rng = rand::rng();
        let mut new_candidate = *candidate;
        let idx = rng.random_range(0..10);
        let mutation = (rng.random_range(-1.0..1.0) * scale as f64) * 0.5;
        new_candidate[idx] = (new_candidate[idx] + mutation).clamp(0.0, 3.0);
        new_candidate
    }

    fn is_valid(&self, candidate: &[f64; 10]) -> bool {
        candidate.iter().all(|&x| (0.0..=3.0).contains(&x))
    }

    fn check_archival(&self, _candidate: &[f64; 10], fitness: (u32, u32)) -> bool {
        fitness.0 < 300 // Archive threshold
    }
}

