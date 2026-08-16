use crate::science::crucible::Gene;

const TARGET_VOID_SPECTRUM: [f64; 32] = [
    3.84, -4.5509996, 4.7208, -4.4534006, -0.68, 0.022600003, 2.78, 3.0142, 
    -0.24, -1.37, -3.7832, -3.43, -4.9512, -13.29, 1.8088, -2.5544, 
    2.56, -4.1606, 0.532, -4.1848, 3.7256, 0.21679999, 1.42, 4.37, 
    2.8636, 4.9129996, -2.78, -1.91, 2.6391997, 1.173, -4.4136, -1.6322
];

pub fn evaluate_metamaterial(genes: &[Gene]) -> f64 {
    let mut total_mse = 0.0;
    
    for (i, g) in genes.iter().enumerate() {
        let void_dim = g.current_value;
        let structural_reflection = (void_dim * 1.5).sin() + (void_dim * std::f64::consts::PI).cos();
        let target_response = TARGET_VOID_SPECTRUM[i];
        
        total_mse += (structural_reflection - target_response).powi(2);
    }

    (total_mse * 100.0).max(0.0)
}
