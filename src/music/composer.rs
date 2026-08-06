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

    /// Discovers the next chord in a progression, obeying Counterpoint rules
    /// to avoid parallel 5ths/8ves and preferring smooth voice leading.
    pub fn discover_bach_progression(chord_a: &[f64]) -> Vec<f64> {
        println!(
            "🎵 Bach Engine: Searching for the next chord after {:?}...",
            chord_a
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

        let (best_fitness, best_genes) = TheCrucible::anneal(
            genes,
            |current_genes| {
                // Force standard piano keys (integer semitones) by rounding
                let mut chord_b_midi = Vec::new();
                for g in current_genes {
                    chord_b_midi.push(g.current_value.round());
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

                // Combine into a single fitness score (Cost to minimize)
                roughness + (vl_distance * 0.5) + parallel_penalty + stagnation_penalty
            },
            iterations,
        );

        println!("✅ Bach Progression Discovery Complete!");
        println!("Minimum Cost Score: {:.4}", best_fitness);
        println!("Previous Chord (A): {:?}", chord_a);
        println!("Next Chord (B):");

        let mut result = Vec::new();
        for g in best_genes {
            let val = g.current_value.round();
            println!(" - {}: {:.0}", g.name, val);
            result.push(val);
        }

        result
    }
}
