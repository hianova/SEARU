mod science;
mod music;

use music::composer::EvolutionaryComposer;

fn main() {
    println!("🎹 Welcome to SEARU: The Algorithmic Music Suite");
    
    // We start with C4 (Middle C) which is MIDI note 60.
    // The EvolutionaryComposer will use TheCrucible (Monte Carlo Annealing)
    // to search for two other notes that form the most mathematically 
    // consonant triad based on Plomp-Levelt psychoacoustic dissonance curves.
    EvolutionaryComposer::discover_pure_triad(60.0);
}
