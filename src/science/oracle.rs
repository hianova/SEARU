use crate::science::enlighten_engine::EnlightenEngineFast;
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Copy, Debug)]
pub enum DomainContext {
    Music { tension: f64, density: f64 },
    Architecture { height: f64, stress: f64 },
    Mechanics { degrees_of_freedom: f64 },
    Language { prosody_complexity: f64, emotional_tension: f64 },
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
            DomainContext::Language { prosody_complexity, emotional_tension } => {
                vec[0] = 4; // Domain ID
                vec[6] = (*prosody_complexity * 10.0) as i8;
                vec[7] = (*emotional_tension * 10.0) as i8;
            }
        }
        vec[7] = vec[7].saturating_add(1); // Global bias token (using saturating add to avoid overflow)
        vec
    }
}

pub struct ExperienceOracle {
    pub engine: EnlightenEngineFast,
    pub stagnation_counter: usize,
    pub genome_dimension: usize,
}

static ORACLE_INSTANCE: OnceLock<Mutex<ExperienceOracle>> = OnceLock::new();

pub fn get_oracle() -> &'static Mutex<ExperienceOracle> {
    ORACLE_INSTANCE.get_or_init(|| {
        let mut engine = EnlightenEngineFast::new(&[8, 128, 2]);
        if let Ok(_) = engine.load("searu.engram") {
            println!("👁 [Experience Oracle] Woke up. Loaded memory from 'searu.engram'");
        } else {
            println!("👁 [Experience Oracle] First incarnation. No engram file found.");
        }
        Mutex::new(ExperienceOracle {
            engine,
            stagnation_counter: 0,
            genome_dimension: 10,
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
            self.stagnation_counter = 0;
            println!("🧬 [Experience Oracle] EPIPHANY ACHIEVED! Preserving current ENLIGHTEN topology.");
            if let Err(e) = self.engine.save("searu.engram") {
                eprintln!("🧬 [Experience Oracle] Failed to persist engram: {}", e);
            } else {
                println!("🧬 [Experience Oracle] Engram persisted to 'searu.engram'");
            }
        } else {
            self.stagnation_counter += 1;
            
            if self.stagnation_counter >= 10 {
                self.trigger_paradigm_shift();
                return;
            }

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

    pub fn trigger_paradigm_shift(&mut self) {
        self.stagnation_counter = 0;
        self.genome_dimension += 1;
        println!("\n=======================================================");
        println!("🌌 [Paradigm Shift] Pareto Collapse Detected!");
        println!("🌌 The system cannot resolve the conflict between Order and Chaos in {}D.", self.genome_dimension - 1);
        println!("🌌 Ripping the Latent Space... Dimension Expanded to {}D!", self.genome_dimension);
        println!("=======================================================\n");
        
        // Huge mutation to reset the topology in the new dimension
        self.engine.mutate(0.5);
    }
}
