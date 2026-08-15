use crate::music::orchestrator::{BarScore, Orchestrator, StepEvent};
use crate::science::crucible::{Gene, TheCrucible};
use rand::Rng;

pub struct MelodyEngine;

impl MelodyEngine {
    /// Generates a fractal melody using The Crucible (1/f constraints & harmonic gravity)
    pub fn generate_melody(
        mut track_score: Vec<BarScore>,
        profile: &crate::profile::ArtistProfile,
    ) -> Vec<BarScore> {
        let mut rng = rand::rng();

        for bar in 0..track_score.len() {
            let energy = track_score[bar].energy;

            // Only play melody if energy is high enough
            if energy < 0.3 {
                continue;
            }

            // Generate a new melody: A-A-A-B phrasing based on profile
            let phrase_len = profile.culture.phrase_length_bars;
            if bar % phrase_len != 0 && bar % phrase_len != (phrase_len - 1) {
                // Repeat motif from first bar of phraseing: Copy Motif A to bars 1 and 2
                for step in 0..16 {
                    track_score[bar].lead[step] = track_score[bar - 1].lead[step].clone();
                }
                continue;
            }

            println!(
                "🧬 Melody Engine: Evolving Fractal Melody for Bar {}...",
                bar
            );

            let scale = &track_score[bar].scale;
            let root = 72.0; // Anchor around C5

            // Create a 2-octave library of offsets from the discovered scale
            let mut scale_offsets = Vec::new();
            for note in scale {
                scale_offsets.push(note - scale[0]);
            }
            for i in 0..scale.len() {
                scale_offsets.push(scale_offsets[i] + 12.0);
            }

            // 16 genes for 16 steps. A step is either Rest (< 0.0) or a Note Index (0.0 to N)
            let mut genes = vec![];
            for i in 0..16 {
                genes.push(Gene {
                    name: format!("step_{}", i),
                    bounds: (-1.0, scale_offsets.len() as f64 - 0.01),
                    current_value: -1.0, // Initial guess: Rest
                });
            }

            let (_, best_genes) = TheCrucible::anneal(
                genes,
                |current_genes| {
                    let mut penalty = 0.0;

                    let mut active_notes = 0;
                    let mut last_note_idx: Option<f64> = None;

                    for i in 0..16 {
                        let val = current_genes[i].current_value;
                        if val >= 0.0 {
                            active_notes += 1;

                            // 1. 1/f Fractal Pitch Contour (Penalize large leaps based on profile)
                            if let Some(last_idx) = last_note_idx {
                                let jump_size = (val - last_idx).abs();
                                penalty += jump_size * jump_size * profile.physics.fractal_chaos;
                            }
                            last_note_idx = Some(val);

                            // 2. Harmonic Gravity & Intentional Flaws (Suspensions)
                            let tension_target = track_score[bar].tension_target;

                            if i % 4 == 0 {
                                let offset = scale_offsets[val as usize];
                                let is_consonant = offset == 0.0 || offset == 7.0 || offset == 12.0;

                                if tension_target > 1.0 {
                                    // High tension: REWARD "flaws" (dissonance/suspensions) on strong beats
                                    if is_consonant {
                                        penalty += 30.0; // Penalize being too boring when tension should be high
                                    }
                                } else {
                                    // Low tension: DEMAND resolution
                                    if !is_consonant {
                                        penalty += 30.0; // Penalize unresolved notes
                                    }
                                }
                            }
                        }
                    }

                    // 3. Rhythmic Breathing
                    let target_notes = (energy * 12.0) as i32;
                    penalty += (active_notes as i32 - target_notes).abs() as f64 * 20.0;

                    // Force breathing space at the end of a phrase
                    if current_genes[14].current_value >= 0.0 {
                        penalty += 15.0;
                    }
                    if current_genes[15].current_value >= 0.0 {
                        penalty += 30.0;
                    }

                    penalty
                },
                5000,
            );

            // Apply evolved genes to the track score
            for i in 0..16 {
                let val = best_genes[i].current_value;
                if val >= 0.0 {
                    let scale_idx = val as usize;
                    let offset = scale_offsets[scale_idx];

                    // Bug Fix: Make melody loud enough to be heard over pad/bass.
                    // Previously `smooth_step(0.4, 0.9)` made it 10% volume at energy 0.5.
                    let vel = Orchestrator::smooth_step(0.2, 0.8, energy).max(0.3)
                        * (0.8 + rng.random::<f32>() * 0.2);

                    // Calculate note length (legato until next note)
                    let mut length = 1.0;
                    for j in (i + 1)..16 {
                        if best_genes[j].current_value >= 0.0 {
                            break;
                        }
                        length += 1.0;
                    }
                    length = (length * (0.5 + rng.random::<f32>() * 0.5)).max(1.0);

                    track_score[bar].lead[i] = StepEvent {
                        velocity: vel,
                        length,
                        pitch: root + offset,
                    };
                }
            }
        }

        track_score
    }
}
