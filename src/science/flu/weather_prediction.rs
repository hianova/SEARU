use crate::science::ScienceObjective;
use crate::science::chaos_runner::ChaosRunner;

/// Macro Objective: Optimize Climate Parameters (sigma, rho, beta)
/// to be RESILIENT to local butterfly perturbations.
#[derive(Clone)]
pub struct WeatherMacroObjective;

impl WeatherMacroObjective {
    pub fn new() -> Self {
        Self {}
    }
}

impl ScienceObjective<[f64; 3]> for WeatherMacroObjective {
    fn evaluate_fitness(&self, macro_params: &[f64; 3]) -> (u32, u32) {
        let sigma = 5.0 + macro_params[0] * 10.0;
        let rho = 10.0 + macro_params[1] * 30.0;
        let beta = 1.0 + macro_params[2] * 4.0;

        let inner_objective = WeatherMicroObjective { sigma, rho, beta };

        let best_inner = ChaosRunner::evaluate_nested(
            inner_objective,
            &format!(
                "Inner Adv. Chaos (σ={:.2}, ρ={:.2}, β={:.2})",
                sigma, rho, beta
            ),
            20,       // population
            5,        // hard limit gen
            u32::MAX, // hard limit score
        );

        let max_divergence = 10000.0 - best_inner as f64;

        let p_sigma = (sigma - 10.0).abs();
        let p_rho = (rho - 28.0).abs();
        let p_beta = (beta - 2.66).abs();

        let loss = max_divergence + p_sigma * 10.0 + p_rho * 10.0 + p_beta * 10.0;

        (loss as u32, 0)
    }

    fn perturb(&self, candidate: &[f64; 3], scale: f32, mut seed: usize) -> [f64; 3] {
        let mut child = *candidate;
        let rand = |s: &mut usize| -> f64 {
            *s = s.wrapping_mul(1664525).wrapping_add(1013904223);
            (*s % 1000) as f64 / 500.0 - 1.0
        };
        for i in 0..3 {
            child[i] = (child[i] + rand(&mut seed) * 0.1 * scale as f64).clamp(0.0, 1.0);
        }
        child
    }

    fn generate_seed(&self, mut seed: usize, parent: Option<&[f64; 3]>) -> [f64; 3] {
        if let Some(p) = parent {
            return *p;
        }
        let rand = |s: &mut usize| -> f64 {
            *s = s.wrapping_mul(1664525).wrapping_add(1013904223);
            (*s % 1000) as f64 / 1000.0
        };
        [rand(&mut seed), rand(&mut seed), rand(&mut seed)]
    }

    fn is_valid(&self, _candidate: &[f64; 3]) -> bool {
        true
    }

    fn check_archival(&self, _candidate: &[f64; 3], _fitness: (u32, u32)) -> bool {
        // Only stop if manual stop or enough generations.
        false
    }
}

/// Micro Objective: Maximize divergence
#[derive(Clone)]
pub struct WeatherMicroObjective {
    sigma: f64,
    rho: f64,
    beta: f64,
}

impl WeatherMicroObjective {
    pub fn new() -> Self {
        Self {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        }
    }

    fn step(&self, state: &mut [f64; 3], dt: f64) {
        let x = state[0];
        let y = state[1];
        let z = state[2];
        let dx = self.sigma * (y - x);
        let dy = x * (self.rho - z) - y;
        let dz = x * y - self.beta * z;
        state[0] += dx * dt;
        state[1] += dy * dt;
        state[2] += dz * dt;
    }
}

impl ScienceObjective<[f64; 3]> for WeatherMicroObjective {
    fn evaluate_fitness(&self, micro_params: &[f64; 3]) -> (u32, u32) {
        let mut base_state = [1.0, 1.0, 1.0];
        let dx = (micro_params[0] - 0.5) * 0.02;
        let dy = (micro_params[1] - 0.5) * 0.02;
        let dz = (micro_params[2] - 0.5) * 0.02;
        let mut pert_state = [1.0 + dx, 1.0 + dy, 1.0 + dz];

        let dt = 0.01;
        for _ in 0..500 {
            self.step(&mut base_state, dt);
            self.step(&mut pert_state, dt);
        }

        let dist = ((base_state[0] - pert_state[0]).powi(2)
            + (base_state[1] - pert_state[1]).powi(2)
            + (base_state[2] - pert_state[2]).powi(2))
        .sqrt();

        let mut score = 10000.0 - (dist * 10.0);
        if score < 0.0 {
            score = 0.0;
        }
        (score as u32, 0)
    }

    fn perturb(&self, candidate: &[f64; 3], scale: f32, mut seed: usize) -> [f64; 3] {
        let mut child = *candidate;
        let rand = |s: &mut usize| -> f64 {
            *s = s.wrapping_mul(1664525).wrapping_add(1013904223);
            (*s % 1000) as f64 / 500.0 - 1.0
        };
        for i in 0..3 {
            child[i] = (child[i] + rand(&mut seed) * 0.1 * scale as f64).clamp(0.0, 1.0);
        }
        child
    }

    fn generate_seed(&self, mut seed: usize, parent: Option<&[f64; 3]>) -> [f64; 3] {
        if let Some(p) = parent {
            return *p;
        }
        let rand = |s: &mut usize| -> f64 {
            *s = s.wrapping_mul(1664525).wrapping_add(1013904223);
            (*s % 1000) as f64 / 1000.0
        };
        [rand(&mut seed), rand(&mut seed), rand(&mut seed)]
    }

    fn is_valid(&self, _candidate: &[f64; 3]) -> bool {
        true
    }

    fn check_archival(&self, _candidate: &[f64; 3], _fitness: (u32, u32)) -> bool {
        false
    }
}
