use crate::science::crucible::{Gene, TheCrucible};
use crate::music::dsp::evo_synth::TimbreProfile;

pub enum InstrumentType {
    Bass,
    Pad,
    Arp,
    Lead,
}

pub struct TimbreEvolver;

impl TimbreEvolver {
    /// Evolves a `TimbreProfile` optimized for sensory consonance and spectral roll-off.
    pub fn evolve_instrument(inst_type: InstrumentType) -> TimbreProfile {
        let (attack_bounds, decay_bounds, sustain_bounds, release_bounds) = match inst_type {
            InstrumentType::Bass => ((0.01, 0.1), (0.1, 0.5), (0.3, 0.8), (0.1, 0.5)),
            InstrumentType::Pad  => ((0.5, 2.0), (1.0, 3.0), (0.6, 1.0), (1.0, 3.0)),
            InstrumentType::Arp  => ((0.005, 0.05), (0.1, 0.3), (0.0, 0.2), (0.1, 0.4)),
            InstrumentType::Lead => ((0.01, 0.05), (0.1, 0.5), (0.6, 1.0), (0.2, 0.8)), // Singing, sustaining, expressively responsive
        };

        let mut genes = Vec::new();
        // 16 Harmonics
        for i in 0..16 {
            genes.push(Gene {
                name: format!("h_{}", i),
                bounds: (0.0, 1.0),
                current_value: if i == 0 { 1.0 } else { 0.1 },
            });
        }
        // Inharmonicity (Stiff strings)
        genes.push(Gene {
            name: "inharmonicity".to_string(),
            bounds: (0.0, 0.001), // Greatly reduced to prevent the "Church Bell" effect
            current_value: 0.0001,
        });
        // ADSR
        genes.push(Gene { name: "attack".to_string(), bounds: attack_bounds, current_value: attack_bounds.0 });
        genes.push(Gene { name: "decay".to_string(), bounds: decay_bounds, current_value: decay_bounds.0 });
        genes.push(Gene { name: "sustain".to_string(), bounds: sustain_bounds, current_value: sustain_bounds.0 });
        genes.push(Gene { name: "release".to_string(), bounds: release_bounds, current_value: release_bounds.0 });

        let (_best_fitness, best_genes) = TheCrucible::anneal(
            genes,
            |current_genes| {
                let mut penalty = 0.0;
                
                // 1. Spectral Attractor (The Mathematical Spiral)
                // We define the perfect mathematical curve (1/f Pink Noise)
                // The Crucible (chaos) will explore around it, but the fitness function will heavily attract it back.
                
                // The Fundamental Frequency (h_0) MUST be loud, otherwise the note disappears!
                let h0_val = current_genes[0].current_value;
                penalty += (h0_val - 1.0).abs() * 100.0;
                
                for i in 1..16 {
                    let h_val = current_genes[i].current_value;
                    let n = (i + 1) as f64;
                    
                    // The mathematical target: 1/f^1.5 (Pink/Brown noise threshold)
                    let ideal_target = 1.0 / n.powf(1.5);
                    
                    // Instead of a ceiling, this is an ATTRACTOR.
                    // Any deviation (too loud OR too quiet) is penalized, forcing the chaos to wrap around the mathematical center.
                    penalty += (h_val - ideal_target).abs() * 50.0;
                }
                
                // 2. Sensory Dissonance (Inharmonicity Clashing)
                let inharm = current_genes[16].current_value;
                match inst_type {
                    InstrumentType::Bass | InstrumentType::Pad | InstrumentType::Lead => {
                        // Bass, Pad, and Lead should be pure and harmonic (vocal-like)
                        penalty += inharm * 10000.0; 
                    },
                    InstrumentType::Arp => {
                        // Arps can have a tiny bit of metallic pluck, but heavily penalized if too high
                        penalty += inharm * 2000.0; 
                    }
                }

                penalty.max(0.0)
            },
            5000 // 5000 iterations of simulated annealing per instrument
        );

        let mut harmonics = [0.0; 16];
        for i in 0..16 {
            harmonics[i] = best_genes[i].current_value;
        }

        TimbreProfile {
            harmonics,
            inharmonicity: best_genes[16].current_value,
            attack: best_genes[17].current_value,
            decay: best_genes[18].current_value,
            sustain: best_genes[19].current_value,
            release: best_genes[20].current_value,
        }
    }
}
