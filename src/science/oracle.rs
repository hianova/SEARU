use crate::science::enlighten_engine::EnlightenEngineFast;
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Copy, Debug)]
pub enum DomainContext {
    Music { tension: f64, density: f64 },
    Architecture { height: f64, stress: f64 },
    Mechanics { degrees_of_freedom: f64 },
}

impl DomainContext {
    /// Maps specific domain features into a shared 8-dimensional INT8 Latent Space.
    pub fn to_latent_vector(&self) -> [i8; 8] {
        let mut vec = [0i8; 8];
        match self {
            DomainContext::Music { tension, density } => {
                vec[0] = 1; // Domain ID
                vec[1] = (tension * 10.0) as i8;
                vec[2] = (density * 10.0) as i8;
            }
            DomainContext::Architecture { height, stress } => {
                vec[0] = 2; // Domain ID
                vec[3] = (height * 10.0) as i8;
                vec[4] = (stress * 10.0) as i8;
            }
            DomainContext::Mechanics { degrees_of_freedom } => {
                vec[0] = 3; // Domain ID
                vec[5] = (*degrees_of_freedom * 2.0) as i8;
            }
        }
        vec[7] = 1; // Global bias token
        vec
    }
}

pub struct ExperienceOracle {
    pub engine: EnlightenEngineFast,
}

static ORACLE_INSTANCE: OnceLock<Mutex<ExperienceOracle>> = OnceLock::new();

pub fn get_oracle() -> &'static Mutex<ExperienceOracle> {
    ORACLE_INSTANCE.get_or_init(|| {
        Mutex::new(ExperienceOracle {
            // Unified Latent Space network shape: input size 8, hidden 128, output 2 (temp, scale)
            engine: EnlightenEngineFast::new(&[8, 128, 2]),
        })
    })
}

impl ExperienceOracle {
    /// Queries the ENLIGHTEN Liquid-KAN Network to predict the starting Prior Distribution
    pub fn predict_prior(&mut self, context: DomainContext) -> (f64, f64) {
        println!("🔮 [Experience Oracle] Consulting ENLIGHTEN NeuroEvolution network...");

        let mut sequence = vec![];
        let latent_vector = context.to_latent_vector();
        sequence.push(latent_vector.to_vec());

        // Run continuous-time forward pass
        let output_seq = self.engine.forward_sequence(&sequence, 0.1);
        let final_out = &output_seq[0];

        let logit_0 = final_out[0].abs() as f64;
        let logit_1 = final_out[1].abs() as f64;

        // Decode logits to SEARU Hyperparameters
        let prior_temp = (100.0 - (logit_0 % 50.0)).max(10.0);
        let bounds_scale = 1.0 + ((logit_1 % 10.0) * 0.1);

        println!("   -> Predicted Initial Temp: {:.2}°", prior_temp);
        println!("   -> Predicted Bounds Scale: {:.2}x", bounds_scale);

        (prior_temp, bounds_scale)
    }

    /// Feedback loop: Called after The Crucible finishes an annealing run.
    pub fn learn(&mut self, fitness: f64, is_epiphany: bool) {
        if is_epiphany {
            println!(
                "🧬 [Experience Oracle] EPIPHANY ACHIEVED! Preserving current ENLIGHTEN topology."
            );
            // We do not mutate here, we preserve the golden weights.
            // In a more complex system, we'd call `engine.export_to_high_precision()` and save it.
        } else {
            // If the run didn't hit an epiphany, we trigger NEAT (NeuroEvolution of Augmenting Topologies).
            // Lower fitness = higher mutation rate (Chaos).
            let error = (1.0 - fitness).max(0.01);
            let mutation_rate = (error * 0.05) as f32; // max 5% bit flip rate

            println!(
                "🧬 [Experience Oracle] Fitness {:.4} -> Triggering NEAT Mutation (Rate: {:.4})",
                fitness, mutation_rate
            );
            self.engine.mutate(mutation_rate);
        }
    }
}
