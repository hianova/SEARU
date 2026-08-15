use crate::science::ScienceObjective;

pub struct NeuroMatrix241Objective;

impl ScienceObjective<[f64; 3]> for NeuroMatrix241Objective {
    fn evaluate_fitness(&self, candidate: &[f64; 3]) -> (u32, u32) {
        // Ackley-like non-linear fitness landscape
        let x = candidate[0];
        let y = candidate[1];
        let z = candidate[2];
        let val = x * x + y * y + z * z
            - 10.0 * (x * 3.14).cos()
            - 10.0 * (y * 3.14).cos()
            - 10.0 * (z * 3.14).cos();
        let penalty = (val * 1000.0).abs() as u32;
        (penalty, 0)
    }

    fn generate_seed(&self, seed: usize, _parent: Option<&[f64; 3]>) -> [f64; 3] {
        let s = seed as f64 * 0.01;
        [s.sin(), s.cos(), (s * 2.0).sin()]
    }

    fn perturb(&self, candidate: &[f64; 3], scale: f32, seed: usize) -> [f64; 3] {
        let s = scale as f64 * (seed as f64 * 0.01).sin();
        [candidate[0] + s, candidate[1] - s, candidate[2] + s * 0.5]
    }

    fn is_valid(&self, candidate: &[f64; 3]) -> bool {
        candidate[0].abs() < 100.0 && candidate[1].abs() < 100.0 && candidate[2].abs() < 100.0
    }

    fn check_archival(&self, _candidate: &[f64; 3], fitness: (u32, u32)) -> bool {
        fitness.0 < 100
    }
}
