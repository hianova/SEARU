
use crate::mechanics::optimizer::MechanicsOptimizer;
use crate::mechanics::statics::Truss;

use crate::materials::matcher::MaterialMatcher;
use crate::materials::pbr::PbrMaterial;

use crate::architecture::{FloorPlanner, Room};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct BachRequest {
    pub root_note: Option<f64>,
    pub num_chords: Option<usize>,
    pub seconds_per_chord: Option<f32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FmRequest {
    pub dissonance: Option<f64>,
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

    pub fn optimize_mechanics_truss() -> Truss {
        MechanicsOptimizer::optimize_truss(
            &crate::profile::ArchProfile::default(),
            &crate::profile::PhysicsProfile::default(),
        )
    }

    pub fn match_pbr_material(
        target_front_rgb: [f64; 3],
        _target_edge_rgb: [f64; 3],
    ) -> PbrMaterial {
        MaterialMatcher::match_material(target_front_rgb)
    }

    pub fn optimize_floorplan(profile: crate::profile::ArchProfile) -> Vec<Room> {
        FloorPlanner::optimize_layout(profile)
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

