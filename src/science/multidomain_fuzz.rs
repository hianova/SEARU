use crate::science::ScienceObjective;

#[derive(Clone)]
pub struct MultiDomainFuzzObjective {
    pub env_rooms: f64,
    pub env_truss_bars: f64,
    pub env_metallic: f64,
}

impl MultiDomainFuzzObjective {
    pub fn new(env_rooms: f64, env_truss_bars: f64, env_metallic: f64) -> Self {
        Self { env_rooms, env_truss_bars, env_metallic }
    }
}

impl ScienceObjective<[f64; 9]> for MultiDomainFuzzObjective {
    fn evaluate_fitness(&self, candidate: &[f64; 9]) -> (u32, u32) {
        // --- 1. Decode Genes ---
        // Fluid (Navier-Stokes)
        let enstrophy = candidate[0];
        let pressure_gradient = candidate[1];
        let viscosity = candidate[2];
        let local_strain = candidate[3];
        // Robotics (Kinematics)
        let stiffness = candidate[4];
        let damping = candidate[5];
        let freq = candidate[6];
        // Photonics (Silicon)
        let radius = candidate[7];
        let power = candidate[8];

        let mut total_penalty = 0.0;

        // --- 2. Independent Objectives ---
        let fluid_growth = (enstrophy * pressure_gradient * local_strain) / (viscosity + 0.01);
        let fluid_bound = (enstrophy * enstrophy) / (viscosity * viscosity + 0.01);
        if fluid_growth > fluid_bound {
            total_penalty += (fluid_growth - fluid_bound) * 100.0;
        }

        let resonance = (stiffness / 10.0).sqrt();
        total_penalty += (freq - resonance).abs() * 50.0;
        total_penalty += damping * 2.0 + (100.0 / (damping + 0.1));

        let opt_loss = 10.0 / (radius + 0.1) + radius * 0.1;
        let opt_heat = power * 2.0 + (100.0 / (power + 0.1));
        total_penalty += opt_loss * 10.0 + opt_heat * 5.0;

        // --- 3. MUTUAL FUZZING (Cross-Domain Interference anchored by MegaCity) ---
        
        // Heat vs Viscosity: Heat increases with power, but MegaCity's metallic property helps dissipate it.
        // Good heat dissipation (metallic > 0.5) requires less viscosity drop.
        let effective_heat = 100.0 / (power + 0.1) * (1.0 - self.env_metallic * 0.5);
        total_penalty += (viscosity - effective_heat).powi(2) * 5.0;

        // Robotics vs Fluid: Stiffness vs Local Strain, influenced by room density.
        let expected_strain = stiffness / (10.0 + self.env_rooms);
        total_penalty += (local_strain - expected_strain).powi(2) * 2.0;

        // Robotics vs Photonics: High mechanical frequency causes vibrations. 
        // A rigid MegaCity (many truss bars) dampens it.
        let effective_vibration = freq * 2.0 / (1.0 + self.env_truss_bars * 0.01);
        if radius < effective_vibration {
            total_penalty += (effective_vibration - radius).powi(2) * 10.0;
        }

        (total_penalty as u32, 0)
    }

    fn generate_seed(&self, seed: usize, parent: Option<&[f64; 9]>) -> [f64; 9] {
        if let Some(p) = parent {
            return *p;
        }
        let f = seed as f64;
        [
            // fluid
            (f % 10.0) / 10.0,
            (f % 10.0) / 10.0,
            (f % 100.0) / 10.0 + 0.1,
            (f % 10.0) / 10.0,
            // kinematics
            100.0 + (f % 50.0),
            10.0 + (f % 5.0),
            1.0 + ((f % 10.0) / 10.0),
            // photonics
            5.0 + (f % 5.0),
            10.0 + (f % 10.0),
        ]
    }

    fn perturb(&self, candidate: &[f64; 9], scale: f32, seed: usize) -> [f64; 9] {
        let mut n = *candidate;
        let s = scale as f64;
        
        // Fluid perturbations
        n[0] += (((seed % 100) as f64 - 50.0) / 50.0) * s;
        n[1] += ((((seed + 1) % 100) as f64 - 50.0) / 50.0) * s;
        n[2] += ((((seed + 2) % 100) as f64 - 50.0) / 50.0) * 2.0 * s;
        n[3] += ((((seed + 3) % 100) as f64 - 50.0) / 50.0) * s;
        
        // Kinematics perturbations
        n[4] += ((((seed + 4) % 100) as f64 - 50.0) / 50.0) * 10.0 * s;
        n[5] += ((((seed + 5) % 100) as f64 - 50.0) / 50.0) * 2.0 * s;
        n[6] += ((((seed + 6) % 100) as f64 - 50.0) / 50.0) * 0.5 * s;

        // Photonics perturbations
        n[7] += ((((seed + 7) % 100) as f64 - 50.0) / 50.0) * 2.0 * s;
        n[8] += ((((seed + 8) % 100) as f64 - 50.0) / 50.0) * 2.0 * s;

        // Clamping bounds
        n[0] = n[0].clamp(0.0, 10.0);
        n[1] = n[1].clamp(0.0, 10.0);
        n[2] = n[2].clamp(0.1, 50.0);
        n[3] = n[3].clamp(0.0, 50.0);
        n[4] = n[4].clamp(10.0, 500.0);
        n[5] = n[5].clamp(1.0, 50.0);
        n[6] = n[6].clamp(0.1, 20.0);
        n[7] = n[7].clamp(1.0, 50.0);
        n[8] = n[8].clamp(1.0, 100.0);
        n
    }

    fn check_archival(&self, _: &[f64; 9], _: (u32, u32)) -> bool {
        false
    }

    fn is_valid(&self, _: &[f64; 9]) -> bool {
        true
    }
}
