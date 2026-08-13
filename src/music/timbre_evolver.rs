use crate::science::crucible::{Gene, TheCrucible};
use crate::music::dsp::evo_synth::TimbreProfile;

pub enum InstrumentType {
    Bass,
    Pad,
    Arp,
}

pub struct TimbreEvolver;

impl TimbreEvolver {
    /// Evolves a `TimbreProfile` optimized for sensory consonance and spectral roll-off.
    pub fn evolve_instrument(inst_type: InstrumentType) -> TimbreProfile {
        let (attack_bounds, decay_bounds, sustain_bounds, release_bounds) = match inst_type {
            InstrumentType::Bass => ((0.01, 0.1), (0.1, 0.5), (0.3, 0.8), (0.1, 0.5)),
            InstrumentType::Pad  => ((0.5, 2.0), (1.0, 3.0), (0.6, 1.0), (1.0, 3.0)),
            InstrumentType::Arp  => ((0.005, 0.05), (0.1, 0.3), (0.0, 0.2), (0.1, 0.4)),
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
        // Inharmonicity
        genes.push(Gene {
            name: "inharmonicity".to_string(),
            bounds: (0.0, 0.01), // Stiff strings
            current_value: 0.001,
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
                
                // 1. Spectral Roll-off (1/f Pink Noise limit)
                // Acoustic physics dictate that higher harmonics must contain less energy.
                for i in 1..16 {
                    let h_val = current_genes[i].current_value;
                    let n = (i + 1) as f64;
                    
                    // Ideal acoustic roll-off is roughly 1/n or 1/n^2
                    let ideal_max = 1.0 / n.powf(1.5);
                    
                    if h_val > ideal_max {
                        // Harsh penalty for high frequencies exceeding the pink noise threshold
                        penalty += (h_val - ideal_max) * 50.0;
                    }
                    
                    // We also don't want a pure sine wave (too boring), so reward *some* harmonics
                    if i < 5 && h_val < (ideal_max * 0.1) {
                        penalty += (ideal_max * 0.1 - h_val) * 10.0;
                    }
                }
                
                // 2. Sensory Dissonance (Inharmonicity Clashing)
                let inharm = current_genes[16].current_value;
                if inharm > 0.005 {
                    // Extreme inharmonicity causes partials to clash like a metallic bell,
                    // which is okay for Arps but bad for Pads/Bass.
                    match inst_type {
                        InstrumentType::Bass | InstrumentType::Pad => {
                            penalty += inharm * 1000.0; 
                        },
                        InstrumentType::Arp => {
                            // Arps can be slightly metallic (like a glockenspiel)
                            penalty += inharm * 100.0; 
                        }
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
