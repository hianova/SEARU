use crate::music::composer::EvolutionaryComposer;
use crate::music::dsp::noise::{NoiseColor, NoiseGenerator};
use crate::music::dsp::synth::SineSynth;

use crate::visual::composer::VisualComposer;
use crate::visual::geometry::Shape;

use crate::mechanics::optimizer::MechanicsOptimizer;
use crate::mechanics::statics::Truss;

use crate::materials::matcher::MaterialMatcher;
use crate::materials::pbr::PbrMaterial;

use crate::architecture::{FloorPlanner, Room};
use crate::pcb_routing::{PcbRouter, Trace};
use crate::procedural_animation::{AnimationCurve, AnimationOptimizer};
use crate::typography::{Glyph, TypographyGenerator};
use crate::ui_layout::{LayoutNode, UiOptimizer};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct BachRequest {
    pub root_note: Option<f64>,
    pub num_chords: Option<usize>,
    pub seconds_per_chord: Option<f32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VisualRequest {
    pub num_shapes: Option<usize>,
    pub base_hue: Option<f64>,
    pub fractal_depth: Option<usize>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MaterialRequest {
    pub target_r: f64,
    pub target_g: f64,
    pub target_b: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrackInfo {
    pub id: String,
    pub name: String,
    pub wav_url: String,
    pub midi_url: String,
    pub svg_url: String,
}

pub struct SearuApi;

impl SearuApi {
    pub fn generate_noise(color: NoiseColor, seconds: f32, sample_rate: u32) -> Vec<f32> {
        NoiseGenerator::generate(color, seconds, sample_rate)
    }

    pub fn generate_music_with_profile(profile: &crate::profile::ArtistProfile) -> Vec<f32> {
        use crate::music::arranger::Arranger;
        use crate::music::macro_arranger::MacroArranger;

        let sample_rate = 44100;
        let bpm = 120.0;
        // Use the profile's phrase length for the loop
        let bars = profile.culture.phrase_length_bars;
        let seed_chord = [60.0, 64.0, 67.0]; // C Major
        let energy_curve = MacroArranger::evolve_energy_curve(bars);

        let (audio_data, _, _) =
            Arranger::compose_track(&seed_chord, bars, bpm, sample_rate, &energy_curve, profile);
        audio_data
    }

    pub fn generate_bach_progression(
        start_chord: &[f64],
        num_chords: usize,
        seconds_per_chord: f32,
        sample_rate: u32,
    ) -> Vec<f32> {
        let mut current_chord = start_chord.to_vec();
        let mut history = vec![current_chord.clone()];
        let mut progression = vec![current_chord.clone()];

        for _ in 0..num_chords.saturating_sub(1) {
            let (next_chord, _) = EvolutionaryComposer::discover_bach_progression(&history, 0.0);
            progression.push(next_chord.clone());
            history.push(next_chord.clone());
            current_chord = next_chord;
        }

        let mut full_audio_buffer = Vec::new();
        for chord in progression {
            let mut chord_audio = SineSynth::render_chord(&chord, seconds_per_chord, sample_rate);
            full_audio_buffer.append(&mut chord_audio);
        }

        full_audio_buffer
    }

    pub fn generate_visual_art(num_shapes: usize, base_hue: f64, depth: usize) -> Vec<Shape> {
        let profile = crate::profile::VisualProfile {
            fractal_depth: depth,
            base_hue,
        };
        VisualComposer::generate_art(
            num_shapes,
            "SEARU_Art",
            &[1.2, 0.8, 1.5, 0.4],
            &profile,
        )
    }

    pub fn optimize_mechanics_truss() -> Truss {
        MechanicsOptimizer::optimize_truss()
    }

    pub fn match_pbr_material(
        target_front_rgb: [f64; 3],
        target_edge_rgb: [f64; 3],
    ) -> PbrMaterial {
        MaterialMatcher::match_material(target_front_rgb)
    }

    pub fn optimize_floorplan(profile: crate::profile::ArchProfile) -> Vec<Room> {
        FloorPlanner::optimize_layout(profile)
    }

    pub fn optimize_ui_layout() -> Vec<LayoutNode> {
        UiOptimizer::optimize()
    }

    pub fn route_pcb() -> Vec<Trace> {
        PcbRouter::route()
    }

    pub fn generate_glyph() -> Glyph {
        TypographyGenerator::generate_glyph()
    }

    pub fn optimize_animation_transition() -> AnimationCurve {
        AnimationOptimizer::optimize_transition()
    }

    pub fn list_album_tracks() -> Vec<TrackInfo> {
        let mut tracks = Vec::new();
        let release_dir = std::path::Path::new("release");
        if release_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(release_dir) {
                let mut base_names = std::collections::BTreeSet::new();
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        if ext == "wav" || ext == "mid" || ext == "svg" {
                            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                                base_names.insert(stem.to_string());
                            }
                        }
                    }
                }
                for name in base_names {
                    tracks.push(TrackInfo {
                        id: name.clone(),
                        name: name.clone(),
                        wav_url: format!("/api/album/track/{}.wav", name),
                        midi_url: format!("/api/album/track/{}.mid", name),
                        svg_url: format!("/api/album/track/{}.svg", name),
                    });
                }
            }
        }
        tracks
    }
}

