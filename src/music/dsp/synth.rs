use crate::music::theory::Note;
use std::f32::consts::PI;

pub struct SineSynth;

impl SineSynth {
    /// Renders a chord (a list of MIDI notes) for a given duration in seconds.
    pub fn render_chord(notes: &[f64], seconds: f32, sample_rate: u32) -> Vec<f32> {
        let total_samples = (seconds * sample_rate as f32) as usize;
        let mut buffer = vec![0.0; total_samples];

        let frequencies: Vec<f32> = notes
            .iter()
            .map(|&m| Note::new(m).to_freq() as f32)
            .collect();

        // Simple Envelope parameters (Attack and Release to prevent clicking)
        let attack_samples = (0.05 * sample_rate as f32) as usize; // 50ms attack
        let release_samples = (0.1 * sample_rate as f32) as usize; // 100ms release

        for (i, sample) in buffer.iter_mut().enumerate() {
            let t = i as f32 / sample_rate as f32;

            // Generate combined sine wave
            let mut wave_val = 0.0;
            for &freq in &frequencies {
                wave_val += (2.0 * PI * freq * t).sin();
            }

            // Normalize amplitude to prevent clipping based on number of voices
            if !frequencies.is_empty() {
                wave_val /= frequencies.len() as f32;
            }

            // Apply Envelope
            let mut envelope = 1.0;
            if i < attack_samples {
                envelope = i as f32 / attack_samples as f32;
            } else if i > total_samples.saturating_sub(release_samples) {
                let diff = total_samples - i;
                envelope = diff as f32 / release_samples as f32;
            }

            // Global volume control (0.8)
            *sample = wave_val * envelope * 0.8;
        }

        buffer
    }
}

pub struct FMSynth;

impl FMSynth {
    /// Renders an FM synthesized note.
    /// Modulator acts on the carrier frequency.
    pub fn render_note(midi_note: f64, seconds: f32, sample_rate: u32, mod_index: f32, mod_ratio: f32) -> Vec<f32> {
        let total_samples = (seconds * sample_rate as f32) as usize;
        let mut buffer = vec![0.0; total_samples];
        let freq = Note::new(midi_note).to_freq() as f32;

        let attack_samples = (0.01 * sample_rate as f32) as usize;
        let release_samples = (0.5 * sample_rate as f32) as usize;

        for i in 0..total_samples {
            let t = i as f32 / sample_rate as f32;

            // Modulator
            let mod_freq = freq * mod_ratio;
            let modulator = (2.0 * PI * mod_freq * t).sin() * mod_index;

            // Carrier
            let wave_val = (2.0 * PI * freq * t + modulator).sin();

            // ADSR Envelope
            let mut envelope = 1.0;
            if i < attack_samples {
                envelope = i as f32 / attack_samples as f32;
            } else if i > total_samples.saturating_sub(release_samples) {
                let diff = total_samples - i;
                envelope = diff as f32 / release_samples as f32;
            }

            buffer[i] = wave_val * envelope * 0.6;
        }
        buffer
    }
}

pub struct DrumMachine;

impl DrumMachine {
    pub fn kick(seconds: f32, sample_rate: u32) -> Vec<f32> {
        let total_samples = (seconds * sample_rate as f32) as usize;
        let mut buffer = vec![0.0; total_samples];
        
        let start_freq = 150.0;
        let end_freq = 40.0;
        let decay_samples = (0.2 * sample_rate as f32) as usize; // short decay for kick

        let mut phase = 0.0;
        for i in 0..total_samples {
            let mut envelope = 1.0;
            if i > decay_samples {
                envelope = 0.0;
            } else {
                envelope = 1.0 - (i as f32 / decay_samples as f32);
            }
            
            // Exponential pitch sweep
            let ratio: f32 = end_freq / start_freq;
            let current_freq = start_freq * ratio.powf(i as f32 / decay_samples as f32);
            phase += 2.0 * PI * current_freq / sample_rate as f32;
            
            buffer[i] = phase.sin() * envelope * 0.9;
        }
        buffer
    }

    pub fn hihat(seconds: f32, sample_rate: u32) -> Vec<f32> {
        let total_samples = (seconds * sample_rate as f32) as usize;
        let mut buffer = vec![0.0; total_samples];
        let decay_samples = (0.05 * sample_rate as f32) as usize; // very short decay

        for i in 0..total_samples {
            let mut envelope = 0.0;
            if i < decay_samples {
                envelope = 1.0 - (i as f32 / decay_samples as f32);
            }
            // Pseudo-random noise for hihat
            let noise: f32 = (rand::random::<f32>() * 2.0) - 1.0;
            buffer[i] = noise * envelope * 0.4;
        }
        buffer
    }
}
