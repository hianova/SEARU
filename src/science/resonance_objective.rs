use crate::science::crucible::Gene;

pub fn evaluate_resonance(genes: &[Gene]) -> f64 {
    let mut total_prob = 0.0;
    for g in genes.iter() {
        let x = g.current_value;
        // target_prob(x) = 3.290 * x^2 * exp(-0.785 * x^2)
        let prob = 3.290 * x.powi(2) * (-0.785 * x.powi(2)).exp();
        total_prob += prob;
    }

    let mut repulsion_penalty = 0.0;
    for i in 0..genes.len() {
        for j in (i+1)..genes.len() {
            let diff = (genes[i].current_value - genes[j].current_value).abs();
            if diff < 0.1 {
                repulsion_penalty += (0.1 - diff) * 10.0;
            }
        }
    }

    let base_score = 1000.0 - (total_prob * 100.0);
    (base_score + repulsion_penalty * 100.0).max(0.0)
}

