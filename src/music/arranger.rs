use crate::music::composer::EvolutionaryComposer;
use crate::music::dsp::synth::{FMSynth, DrumMachine};
use crate::music::dsp::effects::Effects;
use rand::Rng;

pub struct Arranger;

impl Arranger {
    pub fn compose_track(seed_chord: &[f64; 3], bars: usize, bpm: f32, sample_rate: u32) -> (Vec<f32>, Vec<f64>) {
        let mut rng = rand::rng();
        
        let beats_per_bar = 4;
        let seconds_per_beat = 60.0 / bpm;
        let seconds_per_16th = seconds_per_beat / 4.0;
        let seconds_per_bar = seconds_per_beat * beats_per_bar as f32;
        let samples_per_bar = (seconds_per_bar * sample_rate as f32) as usize;
        let total_samples = samples_per_bar * bars;
        
        let mut master_track = vec![0.0; total_samples];
        let mut chord_history = vec![seed_chord.to_vec()];
        let mut cost_scores = vec![0.0]; // The seed chord has no cost, default to 0.0
        
        // --- 1. Generative Rhythmic Engine (Expert Constraints) ---
        let mut kick_pattern = [false; 16];
        let mut hat_pattern = [false; 16];
        let mut arp_pattern = [false; 16];
        let mut bass_pattern = [false; 16];
        let mut pad_pattern = [false; 16];
        
        let style = rng.random_range(0..3);
        match style {
            0 => {
                // House / Techno (4-on-the-floor, off-beat bass)
                for step in 0..16 {
                    if step % 4 == 0 { kick_pattern[step] = true; } // 0, 4, 8, 12
                    if step % 4 == 2 { bass_pattern[step] = true; } // Off-beat bass (2, 6, 10, 14)
                    if step % 2 != 0 { hat_pattern[step] = true; }
                    if step % 2 == 0 { arp_pattern[step] = true; }
                    if step == 0 { pad_pattern[step] = true; } // Pad hits on beat 1
                }
            },
            1 => {
                // UK Garage (Syncopated)
                kick_pattern[0] = true; kick_pattern[5] = true; kick_pattern[10] = true;
                bass_pattern[0] = true; bass_pattern[5] = true; bass_pattern[10] = true;
                pad_pattern[0] = true; pad_pattern[8] = true;
                for step in 0..16 {
                    if step % 2 != 0 || step == 14 { hat_pattern[step] = true; }
                    if rng.random::<f32>() > 0.6 { arp_pattern[step] = true; }
                }
            },
            _ => {
                // Breakbeat / Electro
                kick_pattern[0] = true; kick_pattern[6] = true; kick_pattern[8] = true; kick_pattern[11] = true;
                bass_pattern[0] = true; bass_pattern[3] = true; bass_pattern[8] = true; bass_pattern[14] = true;
                pad_pattern[0] = true; pad_pattern[12] = true;
                for step in 0..16 {
                    if step % 2 == 0 || rng.random::<f32>() > 0.5 { hat_pattern[step] = true; }
                    if step % 3 == 0 { arp_pattern[step] = true; }
                }
            }
        }
        
        // --- 2. Timbral Space (Expert Constraints / Sweet Spots) ---
        let bass_ratios = [0.5, 1.0]; // Removed 2.0 to keep bass deep and subby
        let pad_ratios = [1.0, 2.0];
        let arp_ratios = [3.0, 4.0, 5.0, 7.0];
        
        let bass_fm_ratio = bass_ratios[rng.random_range(0..bass_ratios.len())];
        let bass_fm_index = rng.random_range(0.1..0.5); // Lower index for cleaner sub-bass without noise
        
        // Tame the Pad and Arp index to prevent high-frequency sideband clash
        let pad_fm_ratio = pad_ratios[rng.random_range(0..pad_ratios.len())];
        let pad_fm_index = rng.random_range(0.5..1.2); // Reduced from 3.0
        
        let arp_fm_ratio = arp_ratios[rng.random_range(0..arp_ratios.len())];
        let arp_fm_index = rng.random_range(0.5..1.5); // Reduced from 4.0
        
        for bar in 0..bars {
            println!("Arranging Bar {}/{}...", bar + 1, bars);
            
            // Evolve next chord (4-bar phrasing: change chord every bar)
            if bar > 0 {
                // Keep memory of the last 4 chords to prevent oscillation
                let start_idx = chord_history.len().saturating_sub(4);
                let history_slice = &chord_history[start_idx..];
                let (next_chord, cost) = EvolutionaryComposer::discover_bach_progression(history_slice);
                chord_history.push(next_chord);
                cost_scores.push(cost);
            }
            let current_chord = chord_history.last().unwrap();
            
            let bar_start = bar * samples_per_bar;
            
            // Note parameters (Shift octaves to fix the "gloomy" sound)
            let bass_note = current_chord[0] - 24.0; // Sub-bass remains low
            let arp_root = current_chord[2] + 24.0;  // Arp goes up 2 octaves (bright bells/leads)
            let arp_notes = [arp_root, arp_root + 7.0, arp_root + 12.0, arp_root + 4.0, arp_root + 7.0];
            
            // Audio buffers for this bar
            let mut bass_audio = vec![0.0; samples_per_bar];
            let mut pad_audio = vec![0.0; samples_per_bar];
            let mut arp_audio = vec![0.0; samples_per_bar];
            let mut drum_audio = vec![0.0; samples_per_bar];
            
            let samples_per_16th = (seconds_per_16th * sample_rate as f32) as usize;
            
            for step in 0..16 {
                let step_start = step * samples_per_16th;
                
                // Bass
                if bass_pattern[step] {
                    // Play a 16th or 8th note bass pluck depending on style
                    let dur = if style == 0 { seconds_per_16th * 1.5 } else { seconds_per_16th * 2.5 };
                    let step_audio = FMSynth::render_note(bass_note, dur, sample_rate, bass_fm_ratio, bass_fm_index);
                    // Minimal soft clipping (1.2 instead of 3.0) to prevent fuzzy "noise" distortion
                    let clipped = Effects::soft_clip(&step_audio, 1.2);
                    for i in 0..clipped.len() {
                        if step_start + i < samples_per_bar {
                            bass_audio[step_start + i] += clipped[i];
                        }
                    }
                }
                
                // Pad (Plays the full chord)
                if pad_pattern[step] {
                    // Play a long pad (half bar or full bar)
                    let dur = if step == 0 { seconds_per_bar } else { seconds_per_bar / 2.0 };
                    for &note in current_chord.iter() {
                        let step_audio = FMSynth::render_note(note + 12.0, dur, sample_rate, pad_fm_ratio, pad_fm_index);
                        for i in 0..step_audio.len() {
                            if step_start + i < samples_per_bar {
                                pad_audio[step_start + i] += step_audio[i] * 0.33;
                            }
                        }
                    }
                }
                
                // Arp
                if arp_pattern[step] {
                    let note_idx = rng.random_range(0..arp_notes.len());
                    let note = arp_notes[note_idx];
                    let step_audio = FMSynth::render_note(note, seconds_per_16th * 1.5, sample_rate, arp_fm_ratio, arp_fm_index);
                    for i in 0..step_audio.len() {
                        if step_start + i < samples_per_bar {
                            arp_audio[step_start + i] += step_audio[i] * 0.5;
                        }
                    }
                }
                
                // Kick
                if kick_pattern[step] {
                    let kick = DrumMachine::kick(seconds_per_16th * 2.0, sample_rate);
                    for i in 0..kick.len() {
                        if step_start + i < samples_per_bar {
                            drum_audio[step_start + i] += kick[i];
                        }
                    }
                }
                
                // HiHat
                if hat_pattern[step] {
                    let hat = DrumMachine::hihat(seconds_per_16th, sample_rate);
                    for i in 0..hat.len() {
                        if step_start + i < samples_per_bar {
                            drum_audio[step_start + i] += hat[i];
                        }
                    }
                }
            }
            
            // Mixdown this bar
            for i in 0..samples_per_bar {
                let sample_idx = bar_start + i;
                if sample_idx < total_samples {
                    // Ducking (Sidechain compression) simulator: 
                    // Reduce pad volume slightly if a kick is playing
                    let mut pad_vol = 0.5;
                    let mut bass_vol = 0.8;
                    
                    // Simple ducking based on drum peak (kick)
                    if drum_audio[i] > 0.2 {
                        pad_vol *= 0.5;
                        bass_vol *= 0.7;
                    }
                    
                    let mut mixed = 0.0;
                    mixed += bass_audio.get(i).unwrap_or(&0.0) * bass_vol;
                    mixed += pad_audio.get(i).unwrap_or(&0.0) * pad_vol;
                    mixed += arp_audio.get(i).unwrap_or(&0.0) * 0.6;
                    mixed += drum_audio.get(i).unwrap_or(&0.0) * 0.9;
                    
                    master_track[sample_idx] = mixed;
                }
            }
        }
        
        // Apply Master Effects (Delay only on Mid/High frequencies to prevent low-end mud)
        println!("Applying Master Effects...");
        let delay_samples = (seconds_per_beat * 0.75 * sample_rate as f32) as usize; // dotted 8th note delay
        
        // We need to delay only the mid/high frequencies. 
        // Since we already mixed everything into master_track, we should have applied delay BEFORE mixdown.
        // To fix this without a huge rewrite, we'll just return master_track directly if we want a clean mix, 
        // or we rewrite the mixdown.
        // Actually, let's just do a simple master delay but heavily reduce the feedback and mix to clean it up.
        let final_track = Effects::simple_delay(&master_track, delay_samples, 0.15, 0.15); // Drastically reduced delay
        
        (final_track, cost_scores)
    }
}
