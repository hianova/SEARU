use crate::science::ScienceObjective;

#[derive(Clone)]
pub struct OpticalMacro;
impl OpticalMacro {
    pub fn new() -> Self {
        Self
    }
}

impl ScienceObjective<[f64; 2]> for OpticalMacro {
    fn generate_seed(&self, seed: usize, p: Option<&[f64; 2]>) -> [f64; 2] {
        if let Some(parent) = p {
            return self.perturb(parent, 0.5, seed);
        }
        let f = seed as f64;
        [16.0 + (f % 16.0), 0.5 + ((f % 10.0) / 10.0)]
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
        n[0] += (((seed % 100) as f64 - 50.0) / 50.0) * 5.0 * s_val;
        n[1] += ((((seed + 1) % 100) as f64 - 50.0) / 50.0) * 0.2 * s_val;
        n[0] = n[0].clamp(4.0, 128.0);
        n[1] = n[1].clamp(0.1, 2.0);
        n
    }
    fn evaluate_fitness(&self, c: &[f64; 2]) -> (u32, u32) {
        let taps = c[0];
        let alpha = c[1];
        let ber = 100.0 / taps + (alpha - 1.0).abs() * 50.0;
        let dsp_power = taps * taps * 0.1;
        let score = (ber * 10.0 + dsp_power).max(0.0);
        (score as u32, 0)
    }
}
