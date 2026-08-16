use crate::science::ScienceObjective;
use rand::Rng;

#[derive(Clone)]
pub struct AerodynamicObjective;

impl AerodynamicObjective {
    // Target Aerodynamic Flow Constants (Enstrophy & Dissipation Limits)
    const TARGET_ENSTROPHY: f64 = 5.074092;
    const TARGET_DISSIPATION: f64 = 2.951551;
}

// We optimize 8 aerodynamic deflection angles (T = [f64; 8])
impl ScienceObjective<[f64; 8]> for AerodynamicObjective {
    fn evaluate_fitness(&self, candidate: &[f64; 8]) -> (u32, u32) {
        let mut simulated_enstrophy = 0.0;
        let mut simulated_dissipation = 0.0;
        
        for (i, &angle) in candidate.iter().enumerate() {
            let flow_disruption = angle.sin().powi(2) + (angle * 1.5).cos().abs();
            simulated_enstrophy += flow_disruption * (1.1 + (i as f64 * 0.05));
            simulated_dissipation += (angle * 2.0).cos().powi(2) * 0.5;
        }

        let enstrophy_error = (simulated_enstrophy - Self::TARGET_ENSTROPHY).abs();
        let dissipation_error = (simulated_dissipation - Self::TARGET_DISSIPATION).abs();

        let total_mse = enstrophy_error.powi(2) + dissipation_error.powi(2);
        
        let fitness = (total_mse * 1000.0).max(0.0) as u32;

        (fitness, 0)
    }

    fn generate_seed(&self, _seed: usize, _parent: Option<&[f64; 8]>) -> [f64; 8] {
        let mut rng = rand::rng();
        let mut genes = [0.0; 8];
        for gene in &mut genes {
            *gene = rng.random_range(0.0..std::f64::consts::PI);
        }
        genes
    }

    fn perturb(&self, candidate: &[f64; 8], scale: f32, _seed: usize) -> [f64; 8] {
        let mut rng = rand::rng();
        let mut new_candidate = *candidate;
        let idx = rng.random_range(0..8);
        let mutation = rng.random_range(-0.5..0.5) * scale as f64;
        new_candidate[idx] = (new_candidate[idx] + mutation).clamp(0.0, std::f64::consts::PI);
        new_candidate
    }

    fn is_valid(&self, candidate: &[f64; 8]) -> bool {
        candidate.iter().all(|&x| (0.0..=std::f64::consts::PI).contains(&x))
    }

    fn check_archival(&self, _candidate: &[f64; 8], fitness: (u32, u32)) -> bool {
        fitness.0 < 50
    }
}

