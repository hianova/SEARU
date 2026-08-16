use crate::science::crucible::{Gene, TheCrucible};
use std::f64::consts::PI;

pub struct FmPatch {
    pub carrier_ratio: f64,
    pub modulator_ratio: f64,
    pub mod_index: f64,
    pub attack: f64,
    pub decay: f64,
    pub sustain: f64,
    pub release: f64,
}

pub struct FmSynthesizer;

impl FmSynthesizer {
    /// Renders the FM Patch into a raw float audio buffer
    pub fn render(patch: &FmPatch, base_freq: f64, duration_sec: f64, sample_rate: u32) -> Vec<f32> {
        let num_samples = (duration_sec * sample_rate as f64) as usize;
        let mut buffer = Vec::with_capacity(num_samples);

        let c_freq = base_freq * patch.carrier_ratio;
        let m_freq = base_freq * patch.modulator_ratio;

        for i in 0..num_samples {
            let t = i as f64 / sample_rate as f64;
            
            // Envelope (ADSR)
            let mut env = 0.0;
            if t < patch.attack {
                env = t / patch.attack;
            } else if t < patch.attack + patch.decay {
                let dec_t = (t - patch.attack) / patch.decay;
                env = 1.0 - dec_t * (1.0 - patch.sustain);
            } else if t < duration_sec - patch.release {
                env = patch.sustain;
            } else {
                let rel_t = (t - (duration_sec - patch.release)) / patch.release;
                env = patch.sustain * (1.0 - rel_t);
                if env < 0.0 { env = 0.0; }
            }

            // FM Equation: y(t) = A(t) * sin(2*PI*fc*t + I * sin(2*PI*fm*t))
            let mod_sig = (2.0 * PI * m_freq * t).sin();
            let car_sig = (2.0 * PI * c_freq * t + patch.mod_index * mod_sig).sin();
            
            buffer.push((car_sig * env) as f32);
        }

        buffer
    }
}

pub struct FmOptimizer;

impl FmOptimizer {
    /// Anneals an FM Patch towards a specific dissonance target (0.0 = Pure Harmonic, 1.0 = Pure Inharmonic/Noise)
    pub fn optimize_patch(target_dissonance: f64) -> FmPatch {
        println!("🎹 FM Engine: Optimizing Timbre (Target Dissonance: {:.2})", target_dissonance);

        let genes = vec![
            Gene { name: "C_Ratio".to_string(), bounds: (0.5, 10.0), current_value: 1.0 },
            Gene { name: "M_Ratio".to_string(), bounds: (0.1, 15.0), current_value: 1.0 },
            Gene { name: "Mod_Idx".to_string(), bounds: (0.0, 20.0), current_value: 2.0 },
            Gene { name: "Attack".to_string(), bounds: (0.01, 0.5), current_value: 0.1 },
            Gene { name: "Decay".to_string(), bounds: (0.01, 0.5), current_value: 0.2 },
            Gene { name: "Sustain".to_string(), bounds: (0.0, 1.0), current_value: 0.5 },
            Gene { name: "Release".to_string(), bounds: (0.1, 1.0), current_value: 0.5 },
        ];

        let (_, best) = TheCrucible::anneal(
            genes,
            |g| {
                let c_ratio = g[0].current_value;
                let m_ratio = g[1].current_value;
                let mod_idx = g[2].current_value;

                let ratio = c_ratio / m_ratio;
                
                let simple_fractions = [0.5, 0.666, 1.0, 1.5, 2.0, 3.0, 4.0];
                let mut min_dist_to_harmonic = 100.0;
                for &f in &simple_fractions {
                    let d = (ratio - f).abs();
                    if d < min_dist_to_harmonic { min_dist_to_harmonic = d; }
                }

                let mut current_diss = 0.0;
                current_diss += (mod_idx / 20.0) * 0.4;
                
                let inharmonicity = (min_dist_to_harmonic / 0.5).min(1.0);
                current_diss += inharmonicity * 0.6;

                (current_diss - target_dissonance).powi(2) * 10000.0 
            },
            2000,
        );

        FmPatch {
            carrier_ratio: best[0].current_value,
            modulator_ratio: best[1].current_value,
            mod_index: best[2].current_value,
            attack: best[3].current_value,
            decay: best[4].current_value,
            sustain: best[5].current_value,
            release: best[6].current_value,
        }
    }
}
