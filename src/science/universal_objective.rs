use crate::science::crucible::Gene;

/// Evaluates the smoothness score (penalizing high delta variations and variance)
pub fn evaluate_order(genes: &[Gene]) -> f64 {
    let values: Vec<f64> = genes.iter().map(|g| g.current_value).collect();
    
    // Penalize steep step deltas
    let mut energy = 0.0;
    for i in 1..values.len() {
        let delta = values[i] - values[i - 1];
        energy += delta * delta;
    }
    
    // Evaluate distribution entropy
    let mut bins = [0; 10];
    for &v in &values {
        let idx = (v * 10.0).clamp(0.0, 9.0) as usize;
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

/// Evaluates the complexity / diversity score (encouraging exploration)
pub fn evaluate_chaos(genes: &[Gene]) -> f64 {
    let values: Vec<f64> = genes.iter().map(|g| g.current_value).collect();
    
    let mut energy = 0.0;
    for i in 1..values.len() {
        let delta = values[i] - values[i - 1];
        energy += delta * delta;
    }
    
    let mut bins = [0; 10];
    for &v in &values {
        let idx = (v * 10.0).clamp(0.0, 9.0) as usize;
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

/// Evaluates multi-objective balance between smoothness and pattern diversity.
/// Returns (Loss Score, Quality Score).
pub fn evaluate_dissonance(genes: &[Gene]) -> (f64, f64) {
    if genes.len() < 3 {
        return (1000.0, 0.0);
    }

    let smoothness_loss = evaluate_order(genes);
    let diversity_loss = evaluate_chaos(genes);
    
    let total_loss = smoothness_loss + diversity_loss;
    let quality_score = if total_loss < 2.0 { 1.0 / (total_loss + 0.1) } else { 0.1 };

    (total_loss, quality_score)
}
