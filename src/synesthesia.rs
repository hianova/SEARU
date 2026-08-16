use crate::intent::DesignIntent;
use crate::megacity::MegaCityPipeline;
use crate::api::SearuApi;
use crate::music::dsp::exporter::AudioExporter;
use std::fs;

pub struct SynesthesiaEngine;

impl SynesthesiaEngine {
    pub fn generate_experience(intent: DesignIntent) -> Result<(String, String), String> {
        let (mega_profile, artist_profile) = intent.compile();

        // Ensure release directory exists
        fs::create_dir_all("release").map_err(|e| format!("Failed to create release dir: {}", e))?;

        // 1. Generate Architecture (OBJ)
        println!("[Synesthesia] Generating Architecture based on DesignIntent...");
        MegaCityPipeline::run_pipeline(mega_profile);
        
        let obj_path = "release/megacity.obj".to_string();

        // 2. Generate Music (WAV)
        println!("[Synesthesia] Generating Music based on DesignIntent...");
        // SearuApi::generate_music_with_profile returns Vec<f32> audio samples
        let audio_samples = SearuApi::generate_music_with_profile(&artist_profile);
        
        let wav_path = "release/synesthesia.wav".to_string();
        AudioExporter::save_to_wav_file(&wav_path, &audio_samples, 44100).map_err(|e| format!("Failed to save WAV: {}", e))?;

        // Also we can rename megacity.obj to synesthesia.obj
        let final_obj_path = "release/synesthesia.obj".to_string();
        if fs::rename(&obj_path, &final_obj_path).is_err() {
            fs::copy(&obj_path, &final_obj_path).map_err(|e| format!("Failed to copy OBJ: {}", e))?;
        }

        println!("[Synesthesia] Experience generated successfully.");
        Ok((final_obj_path, wav_path))
    }
}
