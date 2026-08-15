use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CultureProfile {
    pub tuning: String,
    pub phrase_length_bars: usize,
    pub rhythmic_grid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicsProfile {
    pub dissonance_tolerance: f64,
    pub fractal_chaos: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArchProfile {
    pub density: usize,
    pub zoning_ratio: f64,
    pub max_wind_force: f64,
}

impl Default for ArchProfile {
    fn default() -> Self {
        Self {
            density: 20,
            zoning_ratio: 0.5,
            max_wind_force: 50.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisualProfile {
    pub fractal_depth: usize,
    pub base_hue: f64,
}

impl Default for VisualProfile {
    fn default() -> Self {
        Self {
            fractal_depth: 12,
            base_hue: 200.0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MechanicsProfile {
    pub target_r: f64,
    pub target_g: f64,
    pub target_b: f64,
}

impl Default for MechanicsProfile {
    fn default() -> Self {
        Self {
            target_r: 0.2,
            target_g: 0.2,
            target_b: 0.2,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MegaCityProfile {
    pub arch: ArchProfile,
    pub visual: VisualProfile,
    pub mechanics: MechanicsProfile,
}

impl Default for MegaCityProfile {
    fn default() -> Self {
        Self {
            arch: ArchProfile::default(),
            visual: VisualProfile::default(),
            mechanics: MechanicsProfile::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArtistProfile {
    pub culture: CultureProfile,
    pub physics: PhysicsProfile,
}

impl Default for ArtistProfile {
    fn default() -> Self {
        Self {
            culture: CultureProfile {
                tuning: "12-TET".to_string(),
                phrase_length_bars: 4,
                rhythmic_grid: "4/4".to_string(),
            },
            physics: PhysicsProfile {
                dissonance_tolerance: 2.0,
                fractal_chaos: 5.0,
            },
        }
    }
}

impl ArtistProfile {
    pub fn load_or_default(path: &str) -> Self {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(profile) = serde_json::from_str(&content) {
                println!("🔧 Loaded Custom Artist Profile from {}", path);
                return profile;
            }
        }
        println!("⚠️ No valid searu_profile.json found. Using default parameters.");
        Self::default()
    }
}
