use crate::music::dsp::evo_synth::TimbreProfile;
use crate::science::crucible::{Gene, TheCrucible};

#[derive(Clone, Debug)]
pub struct MixProfile {
    pub vol_bass: f32,
    pub vol_pad: f32,
    pub vol_arp: f32,
    pub vol_lead: f32,
    pub vol_kick: f32,
    pub vol_hat: f32,
}

pub struct MixEvolver;

impl MixEvolver {
    /// Simplified A-Weighting (Fletcher-Munson) curve to estimate perceived loudness
    fn a_weight(freq: f64) -> f64 {
        if freq < 100.0 {
            0.2 // Ear is very insensitive to sub-bass
        } else if freq < 300.0 {
            0.5
        } else if freq < 1500.0 {
            1.0
        } else if freq < 5000.0 {
            3.5 // Peak sensitivity! (Hi-Hat region, baby cries)
        } else if freq < 10000.0 {
            1.5
        } else {
            0.8
        }
    }

    /// Evaluates the approximate perceived frequency distribution of a TimbreProfile
    fn evaluate_timbre_energy(
        profile: &TimbreProfile,
        root_freq_multiplier: f64,
    ) -> (f64, f64, f64) {
        let mut low = 0.0;
        let mut mid = 0.0;
        let mut high = 0.0;

        for h in 0..16 {
            let freq = (h + 1) as f64 * root_freq_multiplier;
            let amp = profile.harmonics[h];
            let perceived_amp = amp * Self::a_weight(freq);

            if freq < 250.0 {
                low += perceived_amp;
            } else if freq < 2000.0 {
                mid += perceived_amp;
            } else {
                high += perceived_amp;
            }
        }
        (low, mid, high)
    }

    pub fn evolve_mix(
        bass_profile: &TimbreProfile,
        pad_profile: &TimbreProfile,
        arp_profile: &TimbreProfile,
        lead_profile: &TimbreProfile,
        ideal_low: f64,
        ideal_mid: f64,
        ideal_high: f64,
    ) -> MixProfile {
        // Approximate base frequencies for the instruments
        let bass_base_freq = 65.0; // ~C2
        let pad_base_freq = 261.0; // ~C4
        let arp_base_freq = 523.0; // ~C5
        let lead_base_freq = 523.0; // ~C5/C6

        let (b_low, b_mid, b_high) = Self::evaluate_timbre_energy(bass_profile, bass_base_freq);
        let (p_low, p_mid, p_high) = Self::evaluate_timbre_energy(pad_profile, pad_base_freq);
        let (a_low, a_mid, a_high) = Self::evaluate_timbre_energy(arp_profile, arp_base_freq);
        let (l_low, l_mid, l_high) = Self::evaluate_timbre_energy(lead_profile, lead_base_freq);

        // Fixed proxy perceived energies for synthetic drums
        let kick_low = 1.0 * Self::a_weight(80.0); // Kick has very low physical frequency
        let kick_mid = 0.1 * Self::a_weight(500.0);
        let kick_high = 0.0;

        let hat_low = 0.0;
        let hat_mid = 0.1 * Self::a_weight(1000.0);
        let hat_high = 1.0 * Self::a_weight(4000.0); // MASSIVELY penalized by human ear

        // REVERTED to completely free bounds (0.001 to 1.5). Let the physics handle the limitation!
        let genes = vec![
            Gene {
                name: "vol_bass".to_string(),
                bounds: (0.001, 1.5),
                current_value: 0.5,
            },
            Gene {
                name: "vol_pad".to_string(),
                bounds: (0.001, 1.5),
                current_value: 0.4,
            },
            Gene {
                name: "vol_arp".to_string(),
                bounds: (0.001, 1.5),
                current_value: 0.4,
            },
            Gene {
                name: "vol_lead".to_string(),
                bounds: (0.001, 1.5),
                current_value: 0.6,
            },
            Gene {
                name: "vol_kick".to_string(),
                bounds: (0.001, 1.5),
                current_value: 0.8,
            },
            Gene {
                name: "vol_hat".to_string(),
                bounds: (0.001, 1.5),
                current_value: 0.4,
            },
        ];

        let (_best_fitness, best_genes) = TheCrucible::anneal(
            genes.clone(),
            |current_genes| {
                let v_bass = current_genes[0].current_value;
                let v_pad = current_genes[1].current_value;
                let v_arp = current_genes[2].current_value;
                let v_lead = current_genes[3].current_value;
                let v_kick = current_genes[4].current_value;
                let v_hat = current_genes[5].current_value;

                let total_low = (b_low * v_bass)
                    + (p_low * v_pad)
                    + (a_low * v_arp)
                    + (l_low * v_lead)
                    + (kick_low * v_kick)
                    + (hat_low * v_hat);
                let total_mid = (b_mid * v_bass)
                    + (p_mid * v_pad)
                    + (a_mid * v_arp)
                    + (l_mid * v_lead)
                    + (kick_mid * v_kick)
                    + (hat_mid * v_hat);
                let total_high = (b_high * v_bass)
                    + (p_high * v_pad)
                    + (a_high * v_arp)
                    + (l_high * v_lead)
                    + (kick_high * v_kick)
                    + (hat_high * v_hat);

                let total_energy = total_low + total_mid + total_high;

                let mut penalty = 0.0;

                // 1. Headroom limit (prevent overall clipping)
                if total_energy > 4.0 {
                    penalty += (total_energy - 4.0) * 100.0;
                } else if total_energy < 2.0 {
                    penalty += (2.0 - total_energy) * 50.0; // Prevent it from being too quiet
                }

                // 2. Pink Noise target: Low > Mid > High
                // Ideal proportions roughly: Low 60%, Mid 30%, High 10%
                let low_ratio = total_low / total_energy.max(0.001);
                let mid_ratio = total_mid / total_energy.max(0.001);
                let high_ratio = total_high / total_energy.max(0.001);

                penalty += (low_ratio - ideal_low).abs() * 50.0;
                penalty += (mid_ratio - ideal_mid).abs() * 50.0;
                penalty += (high_ratio - ideal_high).abs() * 50.0;

                // 3. Masking penalty (Kick and Bass should not overpower each other excessively)
                let kick_energy = kick_low * v_kick;
                let bass_energy = b_low * v_bass;
                let ratio = kick_energy / bass_energy.max(0.001);
                // In cinematic music, kick should be subdued compared to bass, not equal.
                if ratio > 0.4 {
                    penalty += (ratio - 0.4) * 20.0; // Kick is too loud!
                }

                penalty
            },
            2000, // Fast mixing anneal
        );

        MixProfile {
            vol_bass: best_genes[0].current_value as f32,
            vol_pad: best_genes[1].current_value as f32,
            vol_arp: best_genes[2].current_value as f32,
            vol_lead: best_genes[3].current_value as f32,
            vol_kick: best_genes[4].current_value as f32,
            vol_hat: best_genes[5].current_value as f32,
        }
    }
}
