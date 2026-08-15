use std::f32::consts::PI;
use crate::music::theory::Note;

#[derive(Clone, Debug)]
pub struct TimbreProfile {
    pub harmonics: [f64; 16],
    pub inharmonicity: f64,
    pub attack: f64,  // seconds
    pub decay: f64,   // seconds
    pub sustain: f64, // amplitude ratio
    pub release: f64, // seconds
}

pub struct EvolutionarySynth;

impl EvolutionarySynth {
    /// Renders an Additive Synthesized note using an evolved TimbreProfile.
    /// `brightness` (0.0 to 1.0) can be used to dynamically lowpass/dampen the harmonics.
    pub fn render_note(
        midi_note: f64,
        seconds: f32,
        sample_rate: u32,
        profile: &TimbreProfile,
        brightness: f32, // Energy-driven parameter
    ) -> Vec<f32> {
        let total_samples = (seconds * sample_rate as f32) as usize;
        let mut buffer = vec![0.0; total_samples];
        
        let fundamental_freq = Note::new(midi_note).to_freq() as f64;
        
        let attack_samples = (profile.attack * sample_rate as f64) as usize;
        let decay_samples = (profile.decay * sample_rate as f64) as usize;
        let release_samples = (profile.release * sample_rate as f64) as usize;
        let sustain_level = profile.sustain as f32;

        for i in 0..total_samples {
            let t = i as f64 / sample_rate as f64;
            
            // 1. Additive Harmonics with Inharmonicity
            let mut wave_val = 0.0;
            for h in 0..16 {
                let h_f64 = (h + 1) as f64;
                // Inharmonicity: f_n = n * f_0 * sqrt(1 + B * n^2)
                let stiff_factor = (1.0 + profile.inharmonicity * h_f64 * h_f64).sqrt();
                let freq = h_f64 * fundamental_freq * stiff_factor;
                
                // Nyquist limit
                if freq > (sample_rate as f64 / 2.0) {
                    break; 
                }
                
                // Energy/Brightness damping (High frequencies get damped more when brightness is low)
                let dampening = if h == 0 { 
                    1.0 
                } else { 
                    let cutoff = (brightness as f64).max(0.1);
                    // A gentler exponent decay instead of a hard linear clip.
                    // This preserves the mathematical 1/f structure but softens it at low energy,
                    // rather than annihilating all harmonics and turning it into a whale sonar.
                    cutoff.powf(h_f64 * 0.25)
                };

                let amplitude = profile.harmonics[h] * dampening;
                wave_val += (2.0 * PI as f64 * freq * t).sin() * amplitude;
            }
            
            // Normalize sum (rough approximation to avoid clipping)
            let max_possible_amp: f64 = profile.harmonics.iter().sum();
            if max_possible_amp > 0.0 {
                wave_val /= max_possible_amp.sqrt().max(1.0); // Soft normalization
            }
            
            // 2. ADSR Envelope
            let mut envelope = 0.0;
            if i < attack_samples {
                // Attack phase (linear)
                envelope = i as f32 / attack_samples.max(1) as f32;
            } else if i < attack_samples + decay_samples {
                // Decay phase (exponential decay to sustain)
                let decay_progress = (i - attack_samples) as f32 / decay_samples.max(1) as f32;
                let decay_curve = (1.0 - decay_progress).powi(2); 
                envelope = sustain_level + (1.0 - sustain_level) * decay_curve;
            } else {
                // Sustain phase
                envelope = sustain_level;
            }
            
            // Safe Release phase: always apply release curve at the end of the buffer
            let release_start = total_samples.saturating_sub(release_samples);
            if i >= release_start {
                let release_progress = (i - release_start) as f32 / (total_samples - release_start).max(1) as f32;
                envelope *= (1.0 - release_progress).max(0.0);
            }
            
            buffer[i] = (wave_val as f32) * envelope * 0.5; // Master volume padding
        }
        
        buffer
    }
}
