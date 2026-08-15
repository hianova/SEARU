use crate::science::ScienceObjective;

#[derive(Clone)]
pub struct KinematicsMacro;
impl KinematicsMacro {
    pub fn new() -> Self {
        Self
    }
}

impl ScienceObjective<[f64; 3]> for KinematicsMacro {
    fn generate_seed(&self, seed: usize, p: Option<&[f64; 3]>) -> [f64; 3] {
        if let Some(parent) = p {
            return self.perturb(parent, 0.5, seed);
        }
        let f = seed as f64;
        [
            100.0 + (f % 50.0),
            10.0 + (f % 5.0),
            1.0 + ((f % 10.0) / 10.0),
        ]
    }
    fn check_archival(&self, _: &[f64; 3], _: (u32, u32)) -> bool {
        false
    }
    fn is_valid(&self, c: &[f64; 3]) -> bool {
        c[0] >= 10.0 && c[0] <= 500.0
    }
    fn perturb(&self, c: &[f64; 3], s: f32, seed: usize) -> [f64; 3] {
        let mut n = *c;
        let s_val = s as f64;
        n[0] += (((seed % 100) as f64 - 50.0) / 50.0) * 10.0 * s_val;
        n[1] += ((((seed + 1) % 100) as f64 - 50.0) / 50.0) * 2.0 * s_val;
        n[2] += ((((seed + 2) % 100) as f64 - 50.0) / 50.0) * 0.1 * s_val;
        n[0] = n[0].clamp(10.0, 500.0);
        n[1] = n[1].clamp(1.0, 50.0);
        n[2] = n[2].clamp(0.1, 5.0);
        n
    }
    fn evaluate_fitness(&self, c: &[f64; 3]) -> (u32, u32) {
        let stiffness = c[0];
        let damping = c[1];
        let freq = c[2];
        let resonance = (stiffness / 10.0).sqrt(); // mass=10
        let mut score = 0.0;
        // want freq close to resonance for efficiency
        score += (freq - resonance).abs() * 50.0;
        // want damping to balance stability without wasting energy
        score += damping * 2.0 + (100.0 / damping);
        (score as u32, 0)
    }
}
