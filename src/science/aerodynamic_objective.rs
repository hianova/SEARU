use crate::science::crucible::Gene;

const TARGET_ENSTROPHY: f64 = 5.074092;
const TARGET_DISSIPATION: f64 = 2.951551;

pub fn evaluate_aerodynamic(genes: &[Gene]) -> f64 {
    let mut simulated_enstrophy = 0.0;
    let mut simulated_dissipation = 0.0;
    
    for (i, g) in genes.iter().enumerate() {
        let angle = g.current_value;
        let flow_disruption = angle.sin().powi(2) + (angle * 1.5).cos().abs();
        simulated_enstrophy += flow_disruption * (1.1 + (i as f64 * 0.05));
        simulated_dissipation += (angle * 2.0).cos().powi(2) * 0.5;
    }

    let enstrophy_error = (simulated_enstrophy - TARGET_ENSTROPHY).abs();
    let dissipation_error = (simulated_dissipation - TARGET_DISSIPATION).abs();

    let total_mse = enstrophy_error.powi(2) + dissipation_error.powi(2);
    
    (total_mse * 1000.0).max(0.0)
}
