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

impl Default for PhysicsProfile {
    fn default() -> Self {
        Self {
            dissonance_tolerance: 2.0,
            fractal_chaos: 5.0,
        }
    }
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

impl ArchProfile {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.density == 0 || self.density > 200 {
            return Err("Density must be between 1 and 200.");
        }
        if !(0.0..=1.0).contains(&self.zoning_ratio) {
            return Err("Zoning ratio must be between 0.0 and 1.0.");
        }
        if self.max_wind_force < 0.0 || self.max_wind_force > 500.0 {
            return Err("Max wind force must be between 0.0 and 500.0.");
        }
        Ok(())
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

impl VisualProfile {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.fractal_depth == 0 || self.fractal_depth > 30 {
            return Err("Fractal depth must be between 1 and 30.");
        }
        if !(0.0..=360.0).contains(&self.base_hue) {
            return Err("Base hue must be between 0.0 and 360.0.");
        }
        Ok(())
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

impl MechanicsProfile {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !(0.0..=1.0).contains(&self.target_r)
            || !(0.0..=1.0).contains(&self.target_g)
            || !(0.0..=1.0).contains(&self.target_b)
        {
            return Err("Target RGB components must be within [0.0, 1.0].");
        }
        Ok(())
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

impl MegaCityProfile {
    pub fn validate(&self) -> Result<(), &'static str> {
        self.arch.validate()?;
        self.visual.validate()?;
        self.mechanics.validate()?;
        Ok(())
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
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.culture.phrase_length_bars == 0 || self.culture.phrase_length_bars > 64 {
            return Err("Phrase length must be between 1 and 64 bars.");
        }
        if self.physics.dissonance_tolerance < 0.0 || self.physics.dissonance_tolerance > 20.0 {
            return Err("Dissonance tolerance must be between 0.0 and 20.0.");
        }
        Ok(())
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_validation_defaults() {
        let arch = ArchProfile::default();
        assert!(arch.validate().is_ok());

        let visual = VisualProfile::default();
        assert!(visual.validate().is_ok());

        let mechanics = MechanicsProfile::default();
        assert!(mechanics.validate().is_ok());

        let megacity = MegaCityProfile::default();
        assert!(megacity.validate().is_ok());

        let artist = ArtistProfile::default();
        assert!(artist.validate().is_ok());
    }

    #[test]
    fn test_profile_validation_invalid() {
        let mut arch = ArchProfile::default();
        arch.density = 0;
        assert!(arch.validate().is_err());

        let mut visual = VisualProfile::default();
        visual.base_hue = 400.0;
        assert!(visual.validate().is_err());

        let mut artist = ArtistProfile::default();
        artist.culture.phrase_length_bars = 100;
        assert!(artist.validate().is_err());
    }
}

