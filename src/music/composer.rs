//! The Evolutionary Composer.
//! Connects the science module (The Crucible) to music theory, allowing it to
//! "discover" optimal chords or scales by treating them as an optimization problem.

use crate::science::crucible::{Gene, TheCrucible};
use crate::music::theory::{Note, chord_roughness};

pub struct EvolutionaryComposer;

impl EvolutionaryComposer {
    /// Discovers a 3-note chord that minimizes roughness (dissonance)
    /// while maintaining a specific interval structure or boundaries.
    pub fn discover_pure_triad(root_midi: f64) {
        println!("🎵 Evolutionary Composer: Searching for the most consonant triad above root {}...", root_midi);
        
        // Define our "Genes". The root note is fixed, but the other two notes
        // are free to evolve within a 2-octave range above the root.
        let genes = vec![
            Gene {
                name: "third".to_string(),
                bounds: (root_midi + 2.0, root_midi + 7.0), // Search between a major second and a perfect fifth
                current_value: root_midi + 4.0, // Initial guess: Major 3rd
            },
            Gene {
                name: "fifth".to_string(),
                bounds: (root_midi + 5.0, root_midi + 12.0), // Search between a perfect fourth and an octave
                current_value: root_midi + 7.0, // Initial guess: Perfect 5th
            }
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
            iterations
        );

        println!("✅ Discovery Complete!");
        println!("Minimum Roughness Score: {:.4}", best_roughness);
        println!("Optimal Chord Structure:");
        println!(" - Root: {:.2} (Freq: {:.2} Hz)", root_midi, Note::new(root_midi).to_freq());
        for g in best_genes {
            println!(" - {}: {:.2} (Freq: {:.2} Hz)", g.name, g.current_value, Note::new(g.current_value).to_freq());
        }
    }
}
