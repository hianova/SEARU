use crate::music::arranger::Arranger;
use crate::music::dsp::exporter::AudioExporter;
use crate::music::macro_arranger::MacroArranger;
use rayon::prelude::*;
use std::fs;

pub struct AlbumProducer;

impl AlbumProducer {
    pub fn release_album(track_count: usize) {
        let release_dir = "release";
        fs::create_dir_all(release_dir)
            .unwrap_or_else(|_| println!("Could not create release dir"));

        println!("💿 Starting Album Production: {} tracks", track_count);

        (1..=track_count).into_par_iter().for_each(|i| {
            let track_name = format!("生長_{:02}", i);
            println!("🎛️ Producing {}...", track_name);

            // Randomize BPM and Seed Chord
            let bpm = 70.0 + rand::random::<f32>() * 40.0; // 70 to 110 BPM (Slower, chill pop vibe)
            let root = 48.0 + (rand::random::<f32>() * 12.0).round() as f64; // C3 to B3
            let seed_chord = [root, root + 4.0, root + 7.0]; // Major triad

            // Generate Audio
            let sample_rate = 44100;
            // Generate 3 minutes of music
            let length_minutes = 3.0;
            let total_bars = (length_minutes * bpm / 4.0).round() as usize;

            let profile =
                crate::profile::ArtistProfile::load_or_default("public/searu_profile.json");
            let energy_curve = MacroArranger::evolve_energy_curve(total_bars);
            let (audio_data, _cost_scores, track_score) = Arranger::compose_track(
                &seed_chord,
                total_bars,
                bpm,
                sample_rate,
                &energy_curve,
                &profile,
            );

            // Save WAV
            let wav_path = format!("{}/{}.wav", release_dir, track_name);
            if let Err(e) = AudioExporter::save_to_wav_file(&wav_path, &audio_data, sample_rate) {
                eprintln!("❌ Failed to save WAV {}: {:?}", track_name, e);
                return;
            }

            // Save MIDI
            let midi_path = format!("{}/{}.mid", release_dir, track_name);
            let midi_data = crate::music::midi::MidiExporter::export_to_midi(&track_score, bpm);
            if let Err(e) = std::fs::write(&midi_path, midi_data) {
                eprintln!("❌ Failed to save MIDI {}: {:?}", track_name, e);
                return;
            }

            println!("✅ {} completed! Saved to {}/", track_name, release_dir);
        });

        println!("🎉 Album Release Pipeline Complete!");
    }
}
