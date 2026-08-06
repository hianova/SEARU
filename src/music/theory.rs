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
    let (f_min, f_max) = if freq1 < freq2 {
        (freq1, freq2)
    } else {
        (freq2, freq1)
    };
    let s = 0.24 / (0.0207 * f_min + 18.96);
    let diff = f_max - f_min;

    let a = 3.51;
    let b = 5.75;

    // Plomp & Levelt parameterization (Sethares, 1993)
    let d = (amp1 * amp2)
        * ((std::f64::consts::E).powf(-a * s * diff) - (std::f64::consts::E).powf(-b * s * diff));
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

pub struct Counterpoint;

impl Counterpoint {
    /// 檢查是否有平行五度或平行八度 (回傳非常高的懲罰值)
    /// 假設傳入的 chord_a 和 chord_b 都是按音高排序好的陣列
    pub fn parallel_interval_penalty(chord_a: &[f64], chord_b: &[f64]) -> f64 {
        let mut penalty = 0.0;
        let num_voices = chord_a.len();
        if num_voices != chord_b.len() || num_voices < 2 {
            return 0.0;
        }

        for i in 0..num_voices {
            for j in (i + 1)..num_voices {
                let interval_a = (chord_a[j] - chord_a[i]).round() as i32;
                let interval_b = (chord_b[j] - chord_b[i]).round() as i32;

                // 檢查移動方向是否一致 (平行或反向)
                let move_i = (chord_b[i] - chord_a[i]).round() as i32;
                let move_j = (chord_b[j] - chord_a[j]).round() as i32;

                let is_parallel_motion =
                    move_i != 0 && move_j != 0 && (move_i.signum() == move_j.signum());

                if is_parallel_motion {
                    // 完全五度 (7 個半音) 或 完全八度 (12 個半音)
                    // 這裡用 % 12 可以同時抓到純十二度 (平行五度的八度延伸)
                    if (interval_a % 12 == 7 && interval_b % 12 == 7)
                        || (interval_a % 12 == 0 && interval_b % 12 == 0 && interval_a != 0)
                    {
                        penalty += 10000.0; // 極度嚴厲的懲罰
                    }
                }
            }
        }
        penalty
    }

    /// 聲部進行的距離懲罰 (鼓勵平順移動，避免大跳)
    pub fn voice_leading_distance(chord_a: &[f64], chord_b: &[f64]) -> f64 {
        let mut distance = 0.0;
        for i in 0..chord_a.len() {
            distance += (chord_a[i] - chord_b[i]).abs();
        }
        distance // 每一半音加 1 分懲罰
    }
}
