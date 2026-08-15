pub mod conlang;



/// The type of phoneme in the generative language.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PhonemeType {
    /// Plosives (P, T, K, B, D, G) - High transient energy, acts as a percussive attack.
    Plosive,
    /// Fricatives (S, Z, F, V) - White noise, adds tension.
    Fricative,
    /// Nasals (M, N) - Low frequency resonance.
    Nasal,
    /// Vowels (A, E, I, O, U) - Sustained formant frequencies.
    Vowel,
}

/// A SyllableGene encapsulates both musical pitch/duration and linguistic phonetic data.
/// It operates in the same search space as standard genes but carries acoustic semantic weight.
#[derive(Clone, Debug, PartialEq)]
pub struct SyllableGene {
    pub pitch: f32,          // The fundamental frequency or MIDI note
    pub duration: f32,       // The time value (e.g., 0.25 for 16th note)
    pub formant_1: f32,      // Primary vocal tract resonance (Hz)
    pub formant_2: f32,      // Secondary vocal tract resonance (Hz)
    pub phoneme: PhonemeType,// Categorical phoneme class
}


