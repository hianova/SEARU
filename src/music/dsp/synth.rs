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
            } else if i > total_samples - release_samples {
                envelope = (total_samples - i) as f32 / release_samples as f32;
            }

            // Global volume control (0.8)
            *sample = wave_val * envelope * 0.8;
        }

        buffer
    }
}
