use crate::music::composer::EvolutionaryComposer;
use crate::music::dsp::noise::{NoiseColor, NoiseGenerator};
use crate::music::dsp::synth::SineSynth;

use crate::visual::composer::VisualComposer;
use crate::visual::geometry::Shape;

use crate::mechanics::optimizer::MechanicsOptimizer;
use crate::mechanics::statics::Truss;

use crate::materials::matcher::MaterialMatcher;
use crate::materials::pbr::PbrMaterial;

/// SEARU Core API
/// A unified interface for Web UIs or external applications to interact with the SEARU engine.
pub struct SearuApi;

impl SearuApi {
    // ---------------------------------------------------------
    // Music Domain
    // ---------------------------------------------------------
    pub fn generate_noise(color: NoiseColor, seconds: f32, sample_rate: u32) -> Vec<f32> {
        NoiseGenerator::generate(color, seconds, sample_rate)
    }

    pub fn generate_bach_progression(
        start_chord: &[f64],
        num_chords: usize,
        seconds_per_chord: f32,
        sample_rate: u32,
    ) -> Vec<f32> {
        let mut current_chord = start_chord.to_vec();
        let mut progression = vec![current_chord.clone()];

        for _ in 0..num_chords.saturating_sub(1) {
            let next_chord = EvolutionaryComposer::discover_bach_progression(&current_chord);
            progression.push(next_chord.clone());
            current_chord = next_chord;
        }

        let mut full_audio_buffer = Vec::new();
        for chord in progression {
            let mut chord_audio = SineSynth::render_chord(&chord, seconds_per_chord, sample_rate);
            full_audio_buffer.append(&mut chord_audio);
        }

        full_audio_buffer
    }
    
    // ---------------------------------------------------------
    // Visual Domain
    // ---------------------------------------------------------
    pub fn generate_visual_art(num_shapes: usize, points_per_shape: usize) -> Vec<Shape> {
        VisualComposer::generate_art(num_shapes, points_per_shape)
    }
    
    // ---------------------------------------------------------
    // Mechanics Domain
    // ---------------------------------------------------------
    pub fn optimize_mechanics_truss() -> Truss {
        MechanicsOptimizer::optimize_truss()
    }
    
    // ---------------------------------------------------------
    // Materials Domain
    // ---------------------------------------------------------
    pub fn match_pbr_material(target_front_rgb: [f64; 3], target_edge_rgb: [f64; 3]) -> PbrMaterial {
        MaterialMatcher::match_material(target_front_rgb, target_edge_rgb)
    }
}
