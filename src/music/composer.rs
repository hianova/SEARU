//! The Evolutionary Composer.
//! Connects the science module (The Crucible) to music theory, allowing it to
//! "discover" optimal chords or scales by treating them as an optimization problem.

use crate::music::theory::{Counterpoint, Note, chord_roughness};
use crate::science::crucible::{Gene, TheCrucible};

pub struct EvolutionaryComposer;

impl EvolutionaryComposer {
    /// Discovers a 3-note chord that minimizes roughness (dissonance)
    /// while maintaining a specific interval structure or boundaries.
    pub fn discover_pure_triad(root_midi: f64) {
        println!(
            "🎵 Evolutionary Composer: Searching for the most consonant triad above root {}...",
            root_midi
        );

        // Define our "Genes". The root note is fixed, but the other two notes
        // are free to evolve within a 2-octave range above the root.
        let genes = vec![
            Gene {
                name: "third".to_string(),
                bounds: (root_midi + 2.0, root_midi + 7.0), // Search between a major second and a perfect fifth
                current_value: root_midi + 4.0,             // Initial guess: Major 3rd
            },
            Gene {
                name: "fifth".to_string(),
                bounds: (root_midi + 5.0, root_midi + 12.0), // Search between a perfect fourth and an octave
                current_value: root_midi + 7.0,              // Initial guess: Perfect 5th
            },
        ];

        let iterations = 10000;

        let (best_roughness, best_genes) = TheCrucible::anneal(
            genes,
            |current_genes| {
                let n1 = Note::new(root_midi);
                let n2 = Note::new(current_genes[0].current_value);
                let n3 = Note::new(current_genes[1].current_value);

                // The "Fitness" is the roughness. We want to minimize it.
                chord_roughness(&[n1, n2, n3], 6) // Evaluate up to 6 harmonics
            },
            iterations,
        );

        println!("✅ Discovery Complete!");
        println!("Minimum Roughness Score: {:.4}", best_roughness);
        println!("Optimal Chord Structure:");
        println!(
            " - Root: {:.2} (Freq: {:.2} Hz)",
            root_midi,
            Note::new(root_midi).to_freq()
        );
        for g in best_genes {
            println!(
                " - {}: {:.2} (Freq: {:.2} Hz)",
                g.name,
                g.current_value,
                Note::new(g.current_value).to_freq()
            );
        }
    }

    /// Discovers a mathematically consonant 5-note scale based on the tonic chord
    pub fn discover_scale(
        tonic_chord: &[f64],
        profile: &crate::profile::ArtistProfile,
    ) -> Vec<f64> {
        println!("🎵 Scale Engine: Generating 5-note melodic scale...");

        let root = tonic_chord[0];
        let mut genes = vec![];

        // Evolve 4 additional notes to form a 5-note scale within an octave
        for i in 1..=4 {
            genes.push(Gene {
                name: format!("scale_degree_{}", i),
                bounds: (root + 1.0, root + 11.0),
                current_value: root + (i as f64 * 2.0), // Initial guess
            });
        }

        let (_, best_genes) = TheCrucible::anneal(
            genes,
            |current_genes| {
                let mut scale_notes = vec![root, root + 12.0]; // Root and Octave are fixed
                for g in current_genes {
                    scale_notes.push(g.current_value);
                }
                scale_notes.sort_by(|a, b| a.partial_cmp(b).unwrap());

                let mut penalty = 0.0;

                // 1. Dissonance against the tonic chord (Harmonic Alignment)
                let mut test_chord = tonic_chord.to_vec();
                for note in &scale_notes {
                    test_chord.push(*note);
                }
                let notes: Vec<Note> = test_chord.iter().map(|&m| Note::new(m)).collect();
                penalty += chord_roughness(&notes, 4) * 0.5;

                // 2. Spacing penalty (notes must be at least 1.5 semitones apart to avoid clusters)
                for i in 0..scale_notes.len() - 1 {
                    let diff = scale_notes[i + 1] - scale_notes[i];
                    if diff < 1.5 {
                        penalty += (1.5 - diff) * 100.0; // Huge penalty for clustered notes
                    }
                }

                penalty
            },
            5000,
        );

        let mut scale = vec![root];
        for g in best_genes {
            if profile.culture.tuning == "12-TET" {
                scale.push(g.current_value.round());
            } else {
                scale.push(g.current_value);
            }
        }
        scale.sort_by(|a, b| a.partial_cmp(b).unwrap());

        println!("✅ Scale Generated: {:?}", scale);
        scale
    }

    /// Discovers the next chord in a progression, obeying Counterpoint rules
    /// to avoid parallel 5ths/8ves and preferring smooth voice leading,
    /// while strictly enforcing a Major diatonic scale and penalizing oscillation.
    pub fn discover_bach_progression(
        history: &[Vec<f64>],
        target_dissonance: f64,
    ) -> (Vec<f64>, f64) {
        let chord_a = history.last().expect("History cannot be empty");
        println!(
            "🎵 Bach Engine: Searching for next chord (Target Tension: {:.2})...",
            target_dissonance
        );

        let mut genes = vec![];
        let names = ["Bass", "Middle", "Soprano"];

        for i in 0..chord_a.len() {
            genes.push(Gene {
                name: names.get(i).unwrap_or(&"Voice").to_string(),
                // Search within an octave below to an octave above the previous note
                bounds: (chord_a[i] - 12.0, chord_a[i] + 12.0),
                current_value: chord_a[i], // Initial guess: stay on the same note
            });
        }

        let iterations = 20000; // Increased iterations for more complex landscape

        let (best_fitness, _best_sublime, best_genes) = TheCrucible::anneal_with_sublime(
            genes,
            crate::science::oracle::DomainContext::Music { tension: 0.8, density: 0.5 },
            |current_genes| {
                // Force to C Major Scale
                let snap_to_scale = |midi: f64| -> f64 {
                    let note = midi.round() as i32;
                    let pc = note.rem_euclid(12);
                    let shift = match pc {
                        1 => -1,
                        3 => -1,
                        6 => 1,
                        8 => -1,
                        10 => -1,
                        _ => 0,
                    };
                    (note + shift) as f64
                };

                let mut chord_b_midi = Vec::new();
                for g in current_genes {
                    chord_b_midi.push(snap_to_scale(g.current_value));
                }

                // 1. Sort both chords to ensure voice order is Bass -> Soprano for rule checking
                let mut sorted_a = chord_a.to_vec();
                sorted_a.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let mut sorted_b = chord_b_midi.clone();
                sorted_b.sort_by(|a, b| a.partial_cmp(b).unwrap());

                // 2. Evaluate individual Roughness of Chord B
                let notes_b: Vec<Note> = chord_b_midi.iter().map(|&m| Note::new(m)).collect();
                let roughness = chord_roughness(&notes_b, 6);

                // 3. Counterpoint Rules
                let parallel_penalty =
                    Counterpoint::parallel_interval_penalty(&sorted_a, &sorted_b);
                let vl_distance = Counterpoint::voice_leading_distance(&sorted_a, &sorted_b);

                // 4. Stay active penalty: discourage staying on the exact same chord
                let mut movement = 0.0;
                for i in 0..sorted_a.len() {
                    movement += (sorted_a[i] - sorted_b[i]).abs();
                }
                let stagnation_penalty = if movement < 2.0 { 100.0 } else { 0.0 };

                // 5. History Penalty: Prevent oscillation (A -> B -> A)
                let mut history_penalty = 0.0;
                for past_chord in history {
                    let mut past_sorted = past_chord.clone();
                    past_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

                    let mut diff = 0.0;
                    for i in 0..sorted_b.len() {
                        diff += (sorted_b[i] - past_sorted[i]).abs();
                    }
                    if diff < 2.0 {
                        history_penalty += 50.0; // Huge penalty for returning to a recent chord
                    }
                }

                // 6. Entropy (Harmonic Gravity) - slight randomness so it doesn't get stuck
                let entropy = rand::random::<f64>() * 2.0;

                // 8. Octave Gravity (Pitch Anchoring)
                // Prevent the "escalator effect" where chords drift infinitely upwards or downwards
                let mut gravity_penalty = 0.0;
                for i in 0..sorted_b.len() {
                    let drift = (sorted_b[i] - 60.0).abs(); // Center around Middle C (MIDI 60)
                    if drift > 12.0 {
                        gravity_penalty += (drift - 12.0) * 20.0; // Huge penalty for leaving the center 2 octaves
                    }
                }

                // Combine into a single fitness score (Cost to minimize)
                // INSTEAD of minimizing roughness, we minimize the distance to the TARGET tension!
                let dissonance_penalty = (roughness - target_dissonance).abs() * 100.0;

                let primary_fitness = dissonance_penalty
                    + (vl_distance * 0.5)
                    + parallel_penalty
                    + stagnation_penalty
                    + history_penalty
                    + entropy
                    + gravity_penalty;

                // 9. Sublime Metric (Hidden Symmetry)
                // If the intervals between the notes are exactly equal (e.g. Augmented triad, Diminished triad),
                // this is a mathematically perfect structure.
                let mut sublime_metric = 0.0;
                if sorted_b.len() == 3 {
                    let int1 = sorted_b[1] - sorted_b[0];
                    let int2 = sorted_b[2] - sorted_b[1];
                    if int1 > 0.0 && int2 > 0.0 {
                        // Continuous symmetry score (1.0 = perfect symmetry)
                        sublime_metric = 1.0 / (1.0 + (int1 - int2).abs());
                    }
                }

                (primary_fitness, sublime_metric)
            },
            iterations,
        );

        println!("✅ Bach Progression Discovery Complete!");
        println!("Minimum Cost Score: {:.4}", best_fitness);
        println!("Previous Chord (A): {:?}", chord_a);
        println!("Next Chord (B):");

        let mut result = Vec::new();
        for g in best_genes {
            let note = g.current_value.round() as i32;
            let pc = note.rem_euclid(12);
            let shift = match pc {
                1 => -1,
                3 => -1,
                6 => 1,
                8 => -1,
                10 => -1,
                _ => 0,
            };
            let val = (note + shift) as f64;

            println!(" - {}: {:.0}", g.name, val);
            result.push(val);
        }

        (result, best_fitness)
    }
}

pub fn decode_genes_to_midi(genes: &[Gene]) -> String {
    let mut notes = Vec::new();
    // Map universal 0.0 - 1.0 energy values into MIDI note domain (e.g., C3 to C5)
    for gene in genes {
        let midi = 48.0 + (gene.current_value * 24.0); // 48 is C3, 24 semitones range
        
        // Quantize to C Major scale for human readability (optional, but requested for music)
        let note = midi.round() as i32;
        let pc = note.rem_euclid(12);
        let shift = match pc {
            1 => -1, 3 => -1, 6 => 1, 8 => -1, 10 => -1, _ => 0,
        };
        let final_midi = (note + shift) as f64;
        notes.push(format!("{:.0}", final_midi));
    }
    format!("[ {} ]", notes.join(", "))
}
