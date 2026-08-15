use crate::science::ScienceObjective;

pub const GRID_SIZE: usize = 100;
pub const MACH_NUMBER: f32 = 100.0;

#[derive(Clone, Debug, PartialEq)]
pub struct QuantumFluidState {
    pub wavefunction_amplitude: [f32; GRID_SIZE * GRID_SIZE],
    pub wavefunction_phase: [f32; GRID_SIZE * GRID_SIZE],
}

pub struct Mach100Objective;

impl Default for Mach100Objective {
    fn default() -> Self {
        Self::new()
    }
}

impl Mach100Objective {
    pub fn new() -> Self {
        Self
    }
}

impl ScienceObjective<QuantumFluidState> for Mach100Objective {
    fn evaluate_fitness(&self, candidate: &QuantumFluidState) -> (u32, u32) {
        // P/NP engine evaluates the stability of the Gross-Pitaevskii Equation
        // under Mach 100 extreme shockwave conditions.

        let mut shockwave_intensity = 0.0;
        let mut quantum_vortices = 0;

        for y in 1..GRID_SIZE - 1 {
            for x in 1..GRID_SIZE - 1 {
                let idx = y * GRID_SIZE + x;
                let _phase = candidate.wavefunction_phase[idx];
                let phase_dx =
                    candidate.wavefunction_phase[idx + 1] - candidate.wavefunction_phase[idx - 1];
                let phase_dy = candidate.wavefunction_phase[idx + GRID_SIZE]
                    - candidate.wavefunction_phase[idx - GRID_SIZE];

                let gradient_magnitude = (phase_dx * phase_dx + phase_dy * phase_dy).sqrt();
                shockwave_intensity += gradient_magnitude;

                // Detect phase singularities (quantum vortices)
                if gradient_magnitude > 3.0 {
                    quantum_vortices += 1;
                }
            }
        }

        // We want to maximize the generation of stable quantum vortices while minimizing chaotic shockwave destruction
        let score = (shockwave_intensity - (quantum_vortices as f32 * 100.0)).max(0.0) as u32;
        (score, score)
    }

    fn generate_seed(&self, _seed: usize, parent: Option<&QuantumFluidState>) -> QuantumFluidState {
        if let Some(p) = parent {
            return p.clone();
        }

        let mut amp = [1.0; GRID_SIZE * GRID_SIZE];
        let mut phase = [0.0; GRID_SIZE * GRID_SIZE];

        // Initial state: Extreme Mach 100 uniform flow (High phase gradient in X direction)
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let idx = y * GRID_SIZE + x;
                phase[idx] = (x as f32) * MACH_NUMBER * 0.1;

                // Add a solid obstacle in the center (Warp Bubble)
                if (x as f32 - 50.0).powi(2) + (y as f32 - 50.0).powi(2) < 100.0 {
                    amp[idx] = 0.0;
                }
            }
        }

        QuantumFluidState {
            wavefunction_amplitude: amp,
            wavefunction_phase: phase,
        }
    }

    fn perturb(
        &self,
        candidate: &QuantumFluidState,
        scale: f32,
        mut seed: usize,
    ) -> QuantumFluidState {
        let mut child = candidate.clone();
        let num_mutations = (scale * 50.0).max(1.0).ceil() as usize;

        for _ in 0..num_mutations {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let idx = seed % (GRID_SIZE * GRID_SIZE);

            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let noise = (((seed % 2000) as f32 / 1000.0) - 1.0) * scale * std::f32::consts::PI; // Phase mutation

            // Cannot mutate inside the obstacle
            if child.wavefunction_amplitude[idx] > 0.0 {
                child.wavefunction_phase[idx] += noise;
            }
        }
        child
    }

    fn is_valid(&self, _candidate: &QuantumFluidState) -> bool {
        true
    }

    fn check_archival(&self, _candidate: &QuantumFluidState, fitness: (u32, u32)) -> bool {
        if fitness.0 < 5000 {
            return true;
        }
        false
    }

    fn periodic_validate_and_visualize(&self, _candidate: &QuantumFluidState) {
        // Output for python plotter handled by main
    }
}
