use crate::music::arranger::Arranger;
use crate::music::dsp::exporter::AudioExporter;
use crate::visual::composer::VisualComposer;
use crate::visual::exporter::SvgExporter;
use std::fs;
use rayon::prelude::*;

pub struct AlbumProducer;

impl AlbumProducer {
    pub fn release_album(track_count: usize) {
        let release_dir = "release";
        fs::create_dir_all(release_dir).unwrap_or_else(|_| println!("Could not create release dir"));
        
        println!("💿 Starting Album Production: {} tracks", track_count);
        
        (1..=track_count).into_par_iter().for_each(|i| {
            let track_name = format!("Track_{:02}", i);
            println!("🎛️ Producing {}...", track_name);
            
            // Randomize BPM and Seed Chord
            let bpm = 70.0 + rand::random::<f32>() * 40.0; // 70 to 110 BPM (Slower, chill pop vibe)
            let root = 48.0 + (rand::random::<f32>() * 12.0).round() as f64; // C3 to B3
            let seed_chord = [root, root + 4.0, root + 7.0]; // Major triad
            
            // Generate Audio
            let sample_rate = 44100;
            // Generate 8 bars for demo speed, can be increased for full tracks
            let audio_data = Arranger::compose_track(&seed_chord, 8, bpm, sample_rate); 
            
            // Save WAV
            let wav_path = format!("{}/{}.wav", release_dir, track_name);
            AudioExporter::save_to_wav_file(&wav_path, &audio_data, sample_rate).unwrap();
            
            // Generate Album Art
            let shapes = VisualComposer::generate_art(4, i); // use 'i' as seed
            let svg_path = format!("{}/{}.svg", release_dir, track_name);
            SvgExporter::save_to_svg(&svg_path, &shapes).unwrap();
            
            println!("✅ {} completed! Saved to {}/", track_name, release_dir);
        });
        
        println!("🎉 Album Release Pipeline Complete!");
    }
}
