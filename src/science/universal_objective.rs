use crate::science::crucible::Gene;

pub fn evaluate_order(genes: &[Gene]) -> f64 {
    let values: Vec<f64> = genes.iter().map(|g| g.current_value).collect();
    
    // Order demands zero energy (perfectly flat)
    let mut energy = 0.0;
    for i in 1..values.len() {
        let delta = values[i] - values[i - 1];
        energy += delta * delta;
    }
    
    // Order demands low entropy (perfect predictability)
    let mut bins = [0; 10];
    for &v in &values {
        let mut idx = (v * 10.0) as usize;
        if idx >= 10 { idx = 9; }
        bins[idx] += 1;
    }
    
    let mut entropy = 0.0;
    let n = values.len() as f64;
    for &count in &bins {
        if count > 0 {
            let p = (count as f64) / n;
            entropy -= p * p.log2();
        }
    }
    
    energy * 10.0 + entropy * 10.0
}

pub fn evaluate_chaos(genes: &[Gene]) -> f64 {
    let values: Vec<f64> = genes.iter().map(|g| g.current_value).collect();
    
    // Chaos demands extreme energy
    let mut energy = 0.0;
    for i in 1..values.len() {
        let delta = values[i] - values[i - 1];
        energy += delta * delta;
    }
    
    // Chaos demands max entropy (flat histogram)
    let mut bins = [0; 10];
    for &v in &values {
        let mut idx = (v * 10.0) as usize;
        if idx >= 10 { idx = 9; }
        bins[idx] += 1;
    }
    
    let mut entropy = 0.0;
    let n = values.len() as f64;
    for &count in &bins {
        if count > 0 {
            let p = (count as f64) / n;
            entropy -= p * p.log2();
        }
    }
    
    let energy_deficit = (5.0 - energy).max(0.0);
    let entropy_deficit = (3.32 - entropy).max(0.0);
    
    energy_deficit * 10.0 + entropy_deficit * 10.0
}

/// Evaluates the Dissonance between Absolute Order and Absolute Chaos.
/// The crucible attempts to minimize this dissonance, forcing the system to 
/// discover new dimensions to escape the Pareto Collapse.
pub fn evaluate_dissonance(genes: &[Gene]) -> (f64, f64) {
    if genes.len() < 3 {
        return (1000.0, 0.0);
    }

    let thesis = evaluate_order(genes);
    let antithesis = evaluate_chaos(genes);
    
    // The conflict metric. If genes don't have enough dimensions, this sum will always be high
    // because Order and Chaos are mutually exclusive.
    // However, if dimension expands, the system might encode Chaos in one half and Order in the other,
    // or use the extra dimensions as a modulator.
    let dissonance = thesis + antithesis;
    
    // Sublime is achieved when dissonance approaches 0 (the Synthesis)
    let sublime = if dissonance < 2.0 { 1.0 / (dissonance + 0.1) } else { 0.1 };

    (dissonance, sublime)
}
