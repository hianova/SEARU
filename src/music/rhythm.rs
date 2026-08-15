use crate::music::orchestrator::{BarScore, Orchestrator, StepEvent};
use rand::Rng;

pub struct RhythmEngine;

impl RhythmEngine {
    /// Populates the score with rhythmic events using continuous probabilistic mapping
    pub fn generate_rhythm(
        mut track_score: Vec<BarScore>,
        profile: &crate::profile::ArtistProfile,
    ) -> Vec<BarScore> {
        let mut rng = rand::rng();

        for bar in 0..track_score.len() {
            let energy = track_score[bar].energy;
            let density = track_score[bar].density;

            for step in 0..16 {
                // --- KICK DRUM ---
                // Kick drum appears clearly on downbeats (0, 4, 8, 12) at energy > 0.4
                // Syncopated kicks appear probabilistically as density increases
                let mut kick_vel = 0.0;
                let kick_downbeat = if profile.culture.rhythmic_grid == "4/4" {
                    4
                } else {
                    3
                }; // 4/4 vs Polyrhythm (3/4 overlay)

                if step % kick_downbeat == 0 {
                    kick_vel = Orchestrator::smooth_step(0.4, 0.8, energy);
                } else if profile.culture.rhythmic_grid == "4/4" && step % 2 != 0 {
                    // 16th note syncopation
                    let kick_prob = Orchestrator::smooth_step(0.6, 1.0, density) * 0.4;
                    if rng.random::<f32>() < kick_prob {
                        kick_vel = energy * 0.7;
                    }
                } else if step % 4 == 2 {
                    // 8th note syncopation (up-beats)
                    let kick_prob = Orchestrator::smooth_step(0.5, 0.9, density) * 0.8;
                    if rng.random::<f32>() < kick_prob {
                        kick_vel = energy * 0.8;
                    }
                }

                if kick_vel > 0.0 {
                    track_score[bar].kick[step] = StepEvent {
                        velocity: kick_vel,
                        length: 1.0,
                        pitch: 0.0,
                    };
                }

                // --- BASS ---
                // Bass drone on beat 1 is highly probable at all energies > 0.2
                // At high densities, bass plays 8th notes or syncopation
                let mut bass_vel = 0.0;
                let mut bass_len = 1.0;

                if step == 0 {
                    // Start from 0.1 instead of 0.0 to ensure it's not totally inaudible
                    bass_vel = Orchestrator::smooth_step(0.0, 0.5, energy).max(0.15) * energy;
                    bass_len = 4.0; // Quarter note pulse instead of a 16.0 full bar drone (prevents deep sea noise)
                } else {
                    let bass_prob = Orchestrator::smooth_step(0.5, 1.0, density) * 0.6;
                    if rng.random::<f32>() < bass_prob {
                        bass_vel = energy * 0.8;
                        bass_len = 2.0; // Short pluck
                        // If we pluck, cancel the drone from step 0
                        track_score[bar].bass[0].length = (step as f32).max(1.0);
                    }
                }

                if bass_vel > 0.0 {
                    track_score[bar].bass[step] = StepEvent {
                        velocity: bass_vel,
                        length: bass_len,
                        pitch: 0.0,
                    };
                }

                // --- PAD ---
                // Pads are atmospheric, almost always on beat 1
                if step == 0 {
                    let pad_vel = Orchestrator::smooth_step(0.0, 0.4, energy).max(0.1) * energy;
                    if pad_vel > 0.0 {
                        track_score[bar].pad[step] = StepEvent {
                            velocity: pad_vel,
                            length: 16.0,
                            pitch: 0.0,
                        };
                    }
                }

                // --- ARP ---
                // Arps depend heavily on density. The denser, the more 16th notes.
                let arp_prob = Orchestrator::smooth_step(0.3, 0.9, density);
                if rng.random::<f32>() < arp_prob {
                    let arp_vel = energy * (0.5 + rng.random::<f32>() * 0.5);
                    track_score[bar].arp[step] = StepEvent {
                        velocity: arp_vel,
                        length: 1.0,
                        pitch: 0.0,
                    };
                }

                // --- HI-HAT ---
                // Hi-hats provide high-frequency drive.
                let mut hat_vel = 0.0;
                if step % 2 == 0 {
                    hat_vel = Orchestrator::smooth_step(0.3, 0.7, energy); // 8th note drive
                } else {
                    // 16th notes driven by density
                    let hat_prob = Orchestrator::smooth_step(0.6, 1.0, density);
                    if rng.random::<f32>() < hat_prob {
                        hat_vel = energy * 0.6;
                    }
                }

                if hat_vel > 0.0 {
                    // Add some velocity variance (humanization)
                    hat_vel *= 0.8 + rng.random::<f32>() * 0.4;
                    track_score[bar].hat[step] = StepEvent {
                        velocity: hat_vel.clamp(0.0, 1.0),
                        length: 1.0,
                        pitch: 0.0,
                    };
                }
            }
        }

        track_score
    }
}
