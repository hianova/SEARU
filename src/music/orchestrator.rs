use crate::music::composer::EvolutionaryComposer;

#[derive(Clone, Debug)]
pub struct StepEvent {
    pub velocity: f32, // 0.0 = Rest, 1.0 = Max Velocity
    pub length: f32,   // 1.0 = Full 16th note, 4.0 = Quarter note (sustained)
    pub pitch: f64,    // Absolute MIDI note (used for Lead melodies)
}

#[derive(Clone, Debug)]
pub struct BarScore {
    pub kick: [StepEvent; 16],
    pub hat: [StepEvent; 16],
    pub arp: [StepEvent; 16],
    pub bass: [StepEvent; 16],
    pub pad: [StepEvent; 16],
    pub lead: [StepEvent; 16],
    
    // Macro parameters for the entire bar
    pub energy: f32,
    pub density: f32,
    pub tension_target: f64,
    pub is_anti_drop: bool,
    pub chord: Vec<f64>,
    pub scale: Vec<f64>,
    pub cost: f64,
}

pub struct Orchestrator;

impl Orchestrator {
    /// Smooth step interpolation function
    pub fn smooth_step(edge0: f32, edge1: f32, x: f32) -> f32 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    /// Evaluates the macro curves for the entire track
    pub fn evaluate_macro_curves(bars: usize, energy_curve: &[f64]) -> (Vec<f32>, Vec<f32>, Vec<f64>, Vec<bool>) {
        let mut energy = vec![0.0; bars];
        let mut density = vec![0.0; bars];
        let mut tension = vec![0.0; bars];
        let mut anti_drop = vec![false; bars];

        for bar in 0..bars {
            let e_current = *energy_curve.get(bar).unwrap_or(&0.5) as f32;
            let e_next = *energy_curve.get(bar + 1).unwrap_or(&(e_current as f64)) as f32;
            
            energy[bar] = e_current;
            
            // Density correlates with energy but adds some high-frequency jitter
            let lfo = (bar as f32 * 0.5).sin() * 0.2;
            density[bar] = (e_current * 0.8 + lfo).clamp(0.0, 1.0);
            
            // Tension targets rise sharply during energy build-ups
            let delta = e_next - e_current;
            tension[bar] = if delta > 0.05 {
                (delta * 10.0 + e_current * 2.0) as f64
            } else {
                0.0
            };
            
            // Anti-Drop vacuum
            anti_drop[bar] = e_next > 0.8 && e_current < e_next - 0.1;
        }

        (energy, density, tension, anti_drop)
    }

    /// Orchestrates the entire track structure (Chords + Marco Parameters) before any rendering
    pub fn orchestrate_track(seed_chord: &[f64; 3], bars: usize, energy_curve: &[f64], profile: &crate::profile::ArtistProfile) -> Vec<BarScore> {
        let (energy, density, tension, anti_drop) = Self::evaluate_macro_curves(bars, energy_curve);
        
        let mut chord_history = vec![seed_chord.to_vec()];
        let mut cost_scores = vec![0.0];
        let mut track_score = Vec::with_capacity(bars);

        let empty_step = StepEvent { velocity: 0.0, length: 0.0, pitch: 0.0 };

        // Generate a structured harmonic progression (Loop) based on Phrase Length
        let mut chord_loop = vec![seed_chord.to_vec()];
        let phrase_len = profile.culture.phrase_length_bars;
        for i in 1..phrase_len {
            // Apply Dissonance Tolerance from Profile
            let target_diss = tension[i.min(bars - 1)] * profile.physics.dissonance_tolerance;
            let (next_chord, _) = EvolutionaryComposer::discover_bach_progression(&chord_loop, target_diss);
            chord_loop.push(next_chord);
        }
        
        // Discover an alien scale that perfectly aligns with the seed chord
        let scale = EvolutionaryComposer::discover_scale(seed_chord, profile);

        for bar in 0..bars {
            let current_chord = chord_loop[bar % phrase_len].clone();

            track_score.push(BarScore {
                kick: core::array::from_fn(|_| empty_step.clone()),
                hat: core::array::from_fn(|_| empty_step.clone()),
                arp: core::array::from_fn(|_| empty_step.clone()),
                bass: core::array::from_fn(|_| empty_step.clone()),
                pad: core::array::from_fn(|_| empty_step.clone()),
                lead: core::array::from_fn(|_| empty_step.clone()),
                
                energy: energy[bar],
                density: density[bar],
                tension_target: tension[bar],
                is_anti_drop: anti_drop[bar],
                chord: current_chord,
                scale: scale.clone(),
                cost: 1.5, // Fixed cost since we are looping
            });
        }
        
        track_score
    }
}
