use crate::science::crucible::Gene;

const TARGET_LOAD_GAP: f64 = 0.104561;
const TARGET_STRESS_LIMIT: f64 = 0.293704;

pub fn evaluate_topology_isolation(genes: &[Gene]) -> f64 {
    let mut simulated_load_gap = 0.0;
    let mut simulated_stress = 0.0;
    
    for (i, g) in genes.iter().enumerate() {
        let weight = g.current_value;
        let decay = (-weight * (i as f64 * 0.1)).exp();
        simulated_load_gap += decay * 0.01;
        simulated_stress += (weight * 1.2).sin().abs() * 0.05;
    }

    let load_error = (simulated_load_gap - TARGET_LOAD_GAP).abs();
    let stress_error = (simulated_stress - TARGET_STRESS_LIMIT).abs();

    let total_mse = load_error.powi(2) + stress_error.powi(2);
    
    (total_mse * 2000.0).max(0.0)
}
