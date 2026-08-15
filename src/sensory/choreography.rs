use crate::science::crucible::Gene;

pub fn decode_genes_to_choreography(genes: &[Gene]) -> String {
    if genes.len() < 5 {
        return "Stillness".to_string();
    }

    let values: Vec<f64> = genes.iter().map(|g| g.current_value).collect();
    
    // Calculate energy (smoothness vs erratic)
    let mut energy = 0.0;
    for i in 1..values.len() {
        let delta = values[i] - values[i - 1];
        energy += delta.abs();
    }

    let spine_bend = values[0];
    let shoulder_tension = values[1];
    let hip_extension = values[2];
    
    let style = if energy < 1.0 {
        "Tai-Chi Flow (太極流動)"
    } else if energy < 3.0 {
        "Contemporary Ballet (現代芭蕾)"
    } else {
        "Krumping / Locking (狂派/鎖舞)"
    };

    let posture = if spine_bend > 0.6 {
        "Arching Upward"
    } else if spine_bend < 0.4 {
        "Curled Inward"
    } else {
        "Neutral Stance"
    };

    let tension = if shoulder_tension > 0.7 {
        "Tense & Jagged"
    } else {
        "Relaxed & Fluid"
    };

    format!("[{}] {} with {} motion. Hip Extension: {:.2}", style, posture, tension, hip_extension)
}
