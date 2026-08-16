use crate::science::crucible::Gene;

pub fn evaluate_multidomain(genes: &[Gene], env_rooms: f64, env_truss_bars: f64, env_metallic: f64) -> f64 {
    // Fluid (Navier-Stokes)
    let enstrophy = genes[0].current_value;
    let pressure_gradient = genes[1].current_value;
    let viscosity = genes[2].current_value;
    let local_strain = genes[3].current_value;
    // Robotics (Kinematics)
    let stiffness = genes[4].current_value;
    let damping = genes[5].current_value;
    let freq = genes[6].current_value;
    // Photonics (Silicon)
    let radius = genes[7].current_value;
    let power = genes[8].current_value;

    let mut total_penalty = 0.0;

    let fluid_growth = (enstrophy * pressure_gradient * local_strain) / (viscosity + 0.01);
    let fluid_bound = (enstrophy * enstrophy) / (viscosity * viscosity + 0.01);
    if fluid_growth > fluid_bound {
        total_penalty += (fluid_growth - fluid_bound) * 100.0;
    }

    let resonance = (stiffness / 10.0).sqrt();
    total_penalty += (freq - resonance).abs() * 50.0;
    total_penalty += damping * 2.0 + (100.0 / (damping + 0.1));

    let opt_loss = 10.0 / (radius + 0.1) + radius * 0.1;
    let opt_heat = power * 2.0 + (100.0 / (power + 0.1));
    total_penalty += opt_loss * 10.0 + opt_heat * 5.0;

    let effective_heat = 100.0 / (power + 0.1) * (1.0 - env_metallic * 0.5);
    total_penalty += (viscosity - effective_heat).powi(2) * 5.0;

    let expected_strain = stiffness / (10.0 + env_rooms);
    total_penalty += (local_strain - expected_strain).powi(2) * 2.0;

    let effective_vibration = freq * 2.0 / (1.0 + env_truss_bars * 0.01);
    if radius < effective_vibration {
        total_penalty += (effective_vibration - radius).powi(2) * 10.0;
    }

    total_penalty
}
