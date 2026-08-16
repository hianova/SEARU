use crate::profile::{ArchProfile, ArtistProfile, CultureProfile, MechanicsProfile, MegaCityProfile, PhysicsProfile, VisualProfile};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DesignIntent {
    /// 0.0 to 1.0. Higher means more structural asymmetry, musical dissonance, and faster tempo.
    pub aggression: f64,
    /// 0.0 to 1.0. Higher means slender, symmetrical trusses and harmonic consonance.
    pub elegance: f64,
    /// 0.0 to 1.0. Higher means more truss bars, higher polyphony, and rhythmic complexity.
    pub density: f64,
    /// 0.0 to 1.0. Shifts towards rigidity in architecture and distorted FM synthesis in music.
    pub industrialism: f64,
}

impl Default for DesignIntent {
    fn default() -> Self {
        Self {
            aggression: 0.5,
            elegance: 0.5,
            density: 0.5,
            industrialism: 0.5,
        }
    }
}

impl DesignIntent {
    pub fn compile(&self) -> (MegaCityProfile, ArtistProfile) {
        // Architecture mappings
        let arch_density = 10 + (self.density * 100.0) as usize;
        
        let max_wind_force = 10.0 + (self.aggression * 200.0);
        let zoning_ratio = 0.2 + (self.elegance * 0.6); 
        
        let fractal_depth = 5 + (self.density * 15.0) as usize;
        let base_hue = self.industrialism * 360.0; 

        let megacity_profile = MegaCityProfile {
            arch: ArchProfile {
                density: arch_density,
                zoning_ratio,
                max_wind_force,
            },
            visual: VisualProfile {
                fractal_depth,
                base_hue,
            },
            mechanics: MechanicsProfile {
                target_r: self.industrialism,
                target_g: self.elegance,
                target_b: self.aggression,
            },
        };

        // Music mappings
        let phrase_length_bars = 4 + (self.elegance * 4.0).round() as usize * 4; 
        
        let dissonance_tolerance = self.aggression * 15.0 + self.industrialism * 5.0;
        
        let fractal_chaos = self.density * 10.0 + self.aggression * 10.0;

        let artist_profile = ArtistProfile {
            culture: CultureProfile {
                tuning: if self.industrialism > 0.7 { "Bohlen-Pierce".to_string() } else { "12-TET".to_string() },
                phrase_length_bars,
                rhythmic_grid: if self.density > 0.7 { "7/8".to_string() } else { "4/4".to_string() },
            },
            physics: PhysicsProfile {
                dissonance_tolerance,
                fractal_chaos,
            },
        };

        (megacity_profile, artist_profile)
    }
}
