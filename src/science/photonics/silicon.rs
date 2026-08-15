use crate::science::ScienceObjective;

#[derive(Clone)]
pub struct SiliconMacro;
impl SiliconMacro {
    pub fn new() -> Self {
        Self
    }
}

impl ScienceObjective<[f64; 2]> for SiliconMacro {
    fn generate_seed(&self, seed: usize, p: Option<&[f64; 2]>) -> [f64; 2] {
        if let Some(parent) = p {
            return self.perturb(parent, 0.5, seed);
        }
        let f = seed as f64;
        [5.0 + (f % 5.0), 10.0 + (f % 10.0)]
    }
    fn check_archival(&self, _: &[f64; 2], _: (u32, u32)) -> bool {
        false
    }
    fn is_valid(&self, _: &[f64; 2]) -> bool {
        true
    }
    fn perturb(&self, c: &[f64; 2], s: f32, seed: usize) -> [f64; 2] {
        let mut n = *c;
        let s_val = s as f64;
        n[0] += (((seed % 100) as f64 - 50.0) / 50.0) * 1.0 * s_val;
        n[1] += ((((seed + 1) % 100) as f64 - 50.0) / 50.0) * 2.0 * s_val;
        n[0] = n[0].clamp(1.0, 20.0);
        n[1] = n[1].clamp(1.0, 50.0);
        n
    }
    fn evaluate_fitness(&self, c: &[f64; 2]) -> (u32, u32) {
        let radius = c[0];
        let power = c[1];
        let loss = 10.0 / radius + radius * 0.1; // tradeoff bend loss vs scattering
        let heat = power * 2.0 + (100.0 / power); // need enough power for phase shift
        let score = (loss * 10.0 + heat * 5.0).max(0.0);
        (score as u32, 0)
    }
}
