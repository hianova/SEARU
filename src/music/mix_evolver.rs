use crate::science::crucible::{Gene, TheCrucible};
use crate::music::dsp::evo_synth::TimbreProfile;

#[derive(Clone, Debug)]
pub struct MixProfile {
    pub vol_bass: f32,
    pub vol_pad: f32,
    pub vol_arp: f32,
    pub vol_kick: f32,
    pub vol_hat: f32,
}

pub struct MixEvolver;

impl MixEvolver {
    /// Evaluates the approximate frequency distribution of a TimbreProfile
    fn evaluate_timbre_energy(profile: &TimbreProfile, root_freq_multiplier: f64) -> (f64, f64, f64) {
        let mut low = 0.0;
        let mut mid = 0.0;
        let mut high = 0.0;

        for h in 0..16 {
            let freq = (h + 1) as f64 * root_freq_multiplier;
            let amp = profile.harmonics[h];

            if freq < 250.0 {
                low += amp;
            } else if freq < 2000.0 {
                mid += amp;
            } else {
                high += amp;
            }
        }
        (low, mid, high)
    }

    pub fn evolve_mix(
        bass_profile: &TimbreProfile,
        pad_profile: &TimbreProfile,
        arp_profile: &TimbreProfile,
    ) -> MixProfile {
        // Approximate base frequencies for the instruments
        let bass_base_freq = 65.0; // ~C2
        let pad_base_freq = 261.0; // ~C4
        let arp_base_freq = 523.0; // ~C5

        let (b_low, b_mid, b_high) = Self::evaluate_timbre_energy(bass_profile, bass_base_freq);
        let (p_low, p_mid, p_high) = Self::evaluate_timbre_energy(pad_profile, pad_base_freq);
        let (a_low, a_mid, a_high) = Self::evaluate_timbre_energy(arp_profile, arp_base_freq);

        // Fixed proxy energies for synthetic drums
        let kick_low = 1.0; let kick_mid = 0.1; let kick_high = 0.0;
        let hat_low = 0.0; let hat_mid = 0.2; let hat_high = 1.0;

        let mut genes = vec![
            Gene { name: "vol_bass".to_string(), bounds: (0.1, 1.0), current_value: 0.5 },
            Gene { name: "vol_pad".to_string(), bounds: (0.1, 0.8), current_value: 0.4 },
            Gene { name: "vol_arp".to_string(), bounds: (0.1, 0.8), current_value: 0.4 },
            Gene { name: "vol_kick".to_string(), bounds: (0.2, 1.2), current_value: 0.8 },
            Gene { name: "vol_hat".to_string(), bounds: (0.05, 0.5), current_value: 0.2 },
        ];

        let (_best_fitness, best_genes) = TheCrucible::anneal(
            genes.clone(),
            |current_genes| {
                let v_bass = current_genes[0].current_value;
                let v_pad = current_genes[1].current_value;
                let v_arp = current_genes[2].current_value;
                let v_kick = current_genes[3].current_value;
                let v_hat = current_genes[4].current_value;

                let total_low = (b_low * v_bass) + (p_low * v_pad) + (a_low * v_arp) + (kick_low * v_kick) + (hat_low * v_hat);
                let total_mid = (b_mid * v_bass) + (p_mid * v_pad) + (a_mid * v_arp) + (kick_mid * v_kick) + (hat_mid * v_hat);
                let total_high = (b_high * v_bass) + (p_high * v_pad) + (a_high * v_arp) + (kick_high * v_kick) + (hat_high * v_hat);

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

                let ideal_low = 0.6;
                let ideal_mid = 0.3;
                let ideal_high = 0.1;

                penalty += (low_ratio - ideal_low).abs() * 50.0;
                penalty += (mid_ratio - ideal_mid).abs() * 50.0;
                penalty += (high_ratio - ideal_high).abs() * 50.0;
                
                // 3. Masking penalty (Kick and Bass should not overpower each other excessively)
                let kick_energy = kick_low * v_kick;
                let bass_energy = b_low * v_bass;
                let ratio = kick_energy / bass_energy.max(0.001);
                if ratio < 0.5 {
                    penalty += (0.5 - ratio) * 20.0; // Kick lost
                } else if ratio > 2.0 {
                    penalty += (ratio - 2.0) * 20.0; // Bass lost
                }

                penalty
            },
            2000 // Fast mixing anneal
        );

        MixProfile {
            vol_bass: best_genes[0].current_value as f32,
            vol_pad: best_genes[1].current_value as f32,
            vol_arp: best_genes[2].current_value as f32,
            vol_kick: best_genes[3].current_value as f32,
            vol_hat: best_genes[4].current_value as f32,
        }
    }
}
