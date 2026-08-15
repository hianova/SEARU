use crate::music::dsp::effects::Effects;
use crate::music::dsp::evo_synth::EvolutionarySynth;
use crate::music::dsp::synth::DrumMachine;
use crate::music::melody::MelodyEngine;
use crate::music::mix_evolver::MixEvolver;
use crate::music::orchestrator::Orchestrator;
use crate::music::rhythm::RhythmEngine;
use crate::music::timbre_evolver::{InstrumentType, TimbreEvolver};
use rand::Rng;

pub struct Arranger;

impl Arranger {
    pub fn compose_track(
        seed_chord: &[f64; 3],
        bars: usize,
        bpm: f32,
        sample_rate: u32,
        energy_curve: &[f64],
        profile: &crate::profile::ArtistProfile,
    ) -> (
        Vec<f32>,
        Vec<f64>,
        Vec<crate::music::orchestrator::BarScore>,
    ) {
        let mut rng = rand::rng();

        let beats_per_bar = 4;
        let seconds_per_beat = 60.0 / bpm;
        let seconds_per_16th = seconds_per_beat / 4.0;
        let seconds_per_bar = seconds_per_beat * beats_per_bar as f32;
        let samples_per_bar = (seconds_per_bar * sample_rate as f32) as usize;
        let total_samples = samples_per_bar * bars;

        let mut master_track = vec![0.0; total_samples];

        // --- 1. CONTINUOUS ORCHESTRATION ---
        println!("🧠 Generating Continuous Score Matrix...");
        let base_score = Orchestrator::orchestrate_track(seed_chord, bars, energy_curve, profile);
        let rhythmic_score = RhythmEngine::generate_rhythm(base_score, profile);
        let track_score = MelodyEngine::generate_melody(rhythmic_score, profile);

        // --- 2. EVOLUTIONARY TIMBRES ---
        println!("🧬 Evolving Timbre Space...");
        let bass_profile = TimbreEvolver::evolve_instrument(InstrumentType::Bass);
        let pad_profile = TimbreEvolver::evolve_instrument(InstrumentType::Pad);
        let arp_profile = TimbreEvolver::evolve_instrument(InstrumentType::Arp);
        let lead_profile = TimbreEvolver::evolve_instrument(InstrumentType::Lead);

        println!("🎚️ Evolving Cinematic Mix Spaces...");
        let pink_mix = MixEvolver::evolve_mix(
            &bass_profile,
            &pad_profile,
            &arp_profile,
            &lead_profile,
            0.6,
            0.3,
            0.1,
        );
        let brown_mix = MixEvolver::evolve_mix(
            &bass_profile,
            &pad_profile,
            &arp_profile,
            &lead_profile,
            0.9,
            0.1,
            0.0,
        );

        let mut cost_scores = Vec::with_capacity(bars);
        let mut dissonance_log = vec![0.0; bars];
        let mut anti_drop_events = vec![false; bars];

        for bar in 0..bars {
            let score = &track_score[bar];
            let current_energy = score.energy;

            cost_scores.push(score.cost);
            dissonance_log[bar] = score.tension_target;
            anti_drop_events[bar] = score.is_anti_drop;

            // --- DYNAMIC SPECTRAL MORPHING (Bipolar Interpolation) ---
            let lerp = |a: f32, b: f32, t: f32| a + (b - a) * t;
            let dynamic_vol_bass = lerp(brown_mix.vol_bass, pink_mix.vol_bass, current_energy);
            let dynamic_vol_pad = lerp(brown_mix.vol_pad, pink_mix.vol_pad, current_energy);
            let dynamic_vol_arp = lerp(brown_mix.vol_arp, pink_mix.vol_arp, current_energy);
            let dynamic_vol_lead = lerp(brown_mix.vol_lead, pink_mix.vol_lead, current_energy);
            let dynamic_vol_kick = lerp(brown_mix.vol_kick, pink_mix.vol_kick, current_energy);
            let dynamic_vol_hat = lerp(brown_mix.vol_hat, pink_mix.vol_hat, current_energy);

            // --- NEURAL MAPPING: Energy Curve to Timbre Brightness ---
            let dynamic_pad_brightness = 0.2 + current_energy * 0.8;
            let dynamic_bass_brightness = 0.5 + current_energy * 0.5;
            let dynamic_arp_brightness = 0.4 + current_energy * 0.6;
            let dynamic_lead_brightness = 0.6 + current_energy * 0.4;

            println!(
                "Arranging Bar {}/{}... (Energy: {:.2})",
                bar + 1,
                bars,
                current_energy
            );

            let bar_start = bar * samples_per_bar;
            let current_chord = &score.chord;

            // Note parameters
            let bass_note = current_chord[0] - 12.0; // Sub/Mid bass (~65 Hz) instead of extreme infra-bass (~32 Hz)
            let arp_root = current_chord[2] + 24.0;
            // A classic cinematic / trance arpeggio pattern
            let arp_notes = [
                arp_root,
                arp_root + 7.0,
                arp_root + 12.0,
                arp_root + 19.0,
                arp_root + 12.0,
                arp_root + 7.0,
            ];

            let mut bass_audio = vec![0.0; samples_per_bar];
            let mut pad_audio = vec![0.0; samples_per_bar];
            let mut arp_audio = vec![0.0; samples_per_bar];
            let mut lead_audio = vec![0.0; samples_per_bar];
            let mut drum_audio = vec![0.0; samples_per_bar];

            let samples_per_16th = (seconds_per_16th * sample_rate as f32) as usize;

            for step in 0..16 {
                let step_start = step * samples_per_16th;

                // Bass
                if score.bass[step].velocity > 0.0 {
                    let jitter = rng.random_range(0.0..0.015);
                    let actual_start = step_start + (jitter * sample_rate as f32) as usize;

                    let dur = seconds_per_16th * score.bass[step].length;
                    let step_audio = EvolutionarySynth::render_note(
                        bass_note,
                        dur,
                        sample_rate,
                        &bass_profile,
                        dynamic_bass_brightness,
                    );
                    let clipped = Effects::soft_clip(&step_audio, 1.2 + current_energy);

                    for i in 0..clipped.len() {
                        if actual_start + i < samples_per_bar {
                            bass_audio[actual_start + i] += clipped[i] * score.bass[step].velocity;
                        }
                    }
                }

                // Pad
                if score.pad[step].velocity > 0.0 {
                    let dur = seconds_per_16th * score.pad[step].length;
                    for note in current_chord.iter() {
                        let step_audio = EvolutionarySynth::render_note(
                            note + 12.0,
                            dur,
                            sample_rate,
                            &pad_profile,
                            dynamic_pad_brightness,
                        );
                        for i in 0..step_audio.len() {
                            if step_start + i < samples_per_bar {
                                pad_audio[step_start + i] +=
                                    step_audio[i] * 0.33 * score.pad[step].velocity;
                            }
                        }
                    }
                }

                // Arp
                if score.arp[step].velocity > 0.0 {
                    // Deterministic sequencing instead of random banging
                    let seq_index = (bar * 16 + step) % arp_notes.len();
                    let note = arp_notes[seq_index];
                    let step_audio = EvolutionarySynth::render_note(
                        note,
                        seconds_per_16th * score.arp[step].length,
                        sample_rate,
                        &arp_profile,
                        dynamic_arp_brightness,
                    );
                    for i in 0..step_audio.len() {
                        if step_start + i < samples_per_bar {
                            arp_audio[step_start + i] +=
                                step_audio[i] * 0.5 * score.arp[step].velocity;
                        }
                    }
                }

                // Lead Melody
                if score.lead[step].velocity > 0.0 {
                    let note = score.lead[step].pitch;
                    let step_audio = EvolutionarySynth::render_note(
                        note,
                        seconds_per_16th * score.lead[step].length,
                        sample_rate,
                        &lead_profile,
                        dynamic_lead_brightness,
                    );
                    for i in 0..step_audio.len() {
                        if step_start + i < samples_per_bar {
                            lead_audio[step_start + i] +=
                                step_audio[i] * 1.5 * score.lead[step].velocity; // Lead is prominent
                        }
                    }
                }

                // Kick
                if score.kick[step].velocity > 0.0 {
                    let jitter = rng.random_range(0.0..0.01);
                    let actual_start = step_start + (jitter * sample_rate as f32) as usize;
                    let kick = DrumMachine::kick(seconds_per_16th * 2.0, sample_rate);
                    for i in 0..kick.len() {
                        if actual_start + i < samples_per_bar {
                            drum_audio[actual_start + i] +=
                                kick[i] * dynamic_vol_kick * score.kick[step].velocity;
                        }
                    }
                }

                // HiHat
                if score.hat[step].velocity > 0.0 {
                    let jitter = rng.random_range(0.0..0.02);
                    let actual_start = step_start + (jitter * sample_rate as f32) as usize;
                    let hat = DrumMachine::hihat(seconds_per_16th, sample_rate);
                    for i in 0..hat.len() {
                        if actual_start + i < samples_per_bar {
                            drum_audio[actual_start + i] +=
                                hat[i] * dynamic_vol_hat * score.hat[step].velocity;
                        }
                    }
                }
            }

            // Mixdown
            for i in 0..samples_per_bar {
                let sample_idx = bar_start + i;
                if sample_idx < total_samples {
                    let mut p_vol = dynamic_vol_pad;
                    let mut b_vol = dynamic_vol_bass;

                    // Simple Ducking
                    if drum_audio[i] > 0.2 {
                        p_vol *= 0.5;
                        b_vol *= 0.7;
                    }

                    p_vol *= 0.5 + current_energy * 0.5;
                    b_vol *= 0.3 + current_energy * 0.7;

                    let mut mixed = 0.0;
                    mixed += bass_audio.get(i).unwrap_or(&0.0) * b_vol;
                    mixed += pad_audio.get(i).unwrap_or(&0.0) * p_vol;
                    mixed += arp_audio.get(i).unwrap_or(&0.0) * dynamic_vol_arp;
                    mixed += lead_audio.get(i).unwrap_or(&0.0) * dynamic_vol_lead;
                    mixed += drum_audio.get(i).unwrap_or(&0.0);

                    // The Anti-Drop
                    if score.is_anti_drop && i > samples_per_bar / 2 {
                        mixed = 0.0;
                    }

                    // Master Limiter
                    let safe_mixed = (mixed * 0.5).tanh() * 0.95;
                    master_track[sample_idx] = safe_mixed;
                }
            }
        }

        println!("🌌 Igniting Chaotic Acoustic Space (Environmental Reverb)...");
        let final_track = Effects::process_reverb(&master_track, 0.2, 0.95, 0.2);

        Self::generate_analytics_chart(bars, energy_curve, &dissonance_log, &anti_drop_events);
        println!("✅ Track Mixdown Complete!");
        (final_track, cost_scores, track_score)
    }

    fn generate_analytics_chart(
        bars: usize,
        energy_curve: &[f64],
        dissonance: &[f64],
        anti_drop: &[bool],
    ) {
        let width = 1200;
        let height = 400;
        let padding = 50;

        let mut svg = format!(
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" style=\"background-color:#111;\">\n",
            width, height
        );

        let x_step = (width - padding * 2) as f64 / bars as f64;
        let y_max = (height - padding) as f64;
        let y_min = padding as f64;
        let graph_h = y_max - y_min;

        let mut max_diss = 0.1;
        for &d in dissonance {
            if d > max_diss {
                max_diss = d;
            }
        }

        let mut path_d = String::new();
        for (i, &e) in energy_curve.iter().enumerate() {
            let x = padding as f64 + i as f64 * x_step;
            let y = y_max - (e * graph_h);
            if i == 0 {
                path_d.push_str(&format!("M {:.1} {:.1} ", x, y));
            } else {
                path_d.push_str(&format!("L {:.1} {:.1} ", x, y));
            }
        }
        svg.push_str(&format!(
            "<path d=\"{}\" stroke=\"#4A90E2\" stroke-width=\"3\" fill=\"none\" />\n",
            path_d
        ));

        let mut path_d = String::new();
        for (i, &d) in dissonance.iter().enumerate() {
            let x = padding as f64 + i as f64 * x_step;
            let y = y_max - ((d / max_diss) * graph_h);
            if i == 0 {
                path_d.push_str(&format!("M {:.1} {:.1} ", x, y));
            } else {
                path_d.push_str(&format!("L {:.1} {:.1} ", x, y));
            }
        }
        svg.push_str(&format!("<path d=\"{}\" stroke=\"#E24A4A\" stroke-width=\"2\" stroke-dasharray=\"5,5\" fill=\"none\" />\n", path_d));

        for (i, &is_drop) in anti_drop.iter().enumerate() {
            if is_drop {
                let x = padding as f64 + i as f64 * x_step;
                svg.push_str(&format!("<line x1=\"{0}\" y1=\"{1}\" x2=\"{0}\" y2=\"{2}\" stroke=\"#F5A623\" stroke-width=\"4\" />\n", x, y_min, y_max));
            }
        }

        svg.push_str("</svg>");
        std::fs::write("analytics_chart.svg", svg).unwrap();
        println!("📊 Analytics Validation Chart written to analytics_chart.svg");
    }
}
