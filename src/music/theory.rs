//! Mathematical foundation of Music.
//! Uses psychoacoustic models (e.g. Plomp-Levelt curve) to evaluate dissonance.

pub struct Note {
    pub midi_note: f64, // E.g., 60 for Middle C
}

impl Note {
    pub fn new(midi_note: f64) -> Self {
        Self { midi_note }
    }

    /// Converts a MIDI note number to its physical frequency (Hz)
    pub fn to_freq(&self) -> f64 {
        440.0 * (2.0f64).powf((self.midi_note - 69.0) / 12.0)
    }

    /// Generates the harmonic series of this note up to `n` harmonics
    pub fn harmonics(&self, count: usize) -> Vec<(f64, f64)> {
        let base_freq = self.to_freq();
        let mut h = Vec::new();
        for i in 1..=count {
            // Amplitude falls off as 1/n
            let amplitude = 1.0 / (i as f64);
            h.push((base_freq * (i as f64), amplitude));
        }
        h
    }
}

/// Computes the psychoacoustic sensory dissonance between two frequencies
/// based on the Plomp-Levelt curve.
pub fn dissonance(freq1: f64, amp1: f64, freq2: f64, amp2: f64) -> f64 {
    let (f_min, f_max) = if freq1 < freq2 { (freq1, freq2) } else { (freq2, freq1) };
    let s = 0.24 / (0.0207 * f_min + 18.96);
    let diff = f_max - f_min;
    
    let a = 3.51;
    let b = 5.75;
    
    // Plomp & Levelt parameterization (Sethares, 1993)
    let d = (amp1 * amp2) * ((std::f64::consts::E).powf(-a * s * diff) - (std::f64::consts::E).powf(-b * s * diff));
    d.max(0.0) // Clamp negative values just in case
}

/// Evaluates the total roughness of a chord (a group of notes) by summing the
/// dissonance of all interacting harmonics.
pub fn chord_roughness(notes: &[Note], harmonics: usize) -> f64 {
    let mut all_harmonics = Vec::new();
    for note in notes {
        all_harmonics.extend(note.harmonics(harmonics));
    }

    let mut total_dissonance = 0.0;
    for i in 0..all_harmonics.len() {
        for j in (i + 1)..all_harmonics.len() {
            let (f1, a1) = all_harmonics[i];
            let (f2, a2) = all_harmonics[j];
            total_dissonance += dissonance(f1, a1, f2, a2);
        }
    }
    total_dissonance
}
