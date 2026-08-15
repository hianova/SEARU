use crate::science::crucible::Gene;

pub fn decode_genes_to_gastronomy(genes: &[Gene]) -> String {
    if genes.len() < 5 {
        return "Water".to_string();
    }

    let acidity = genes[0].current_value;
    let sweetness = genes[1].current_value;
    let umami = genes[2].current_value;
    let bitterness = genes[3].current_value;
    let salty = genes[4].current_value;
    
    // Variance dictates texture (low variance = smooth, high variance = fizzy/crunchy)
    let mean = (acidity + sweetness + umami + bitterness + salty) / 5.0;
    let variance = ((acidity - mean).powi(2) + (sweetness - mean).powi(2) + (umami - mean).powi(2) + (bitterness - mean).powi(2) + (salty - mean).powi(2)) / 5.0;

    let texture = if variance > 0.05 {
        "Fizzy & Carbonated (氣泡感)"
    } else if variance > 0.02 {
        "Viscous & Syrupy (濃稠感)"
    } else {
        "Silky Smooth (絲滑感)"
    };

    // Find the dominant flavor
    let mut dominant = "Acidity";
    let mut max_val = acidity;
    
    if sweetness > max_val { dominant = "Sweetness"; max_val = sweetness; }
    if umami > max_val { dominant = "Umami"; max_val = umami; }
    if bitterness > max_val { dominant = "Bitterness"; max_val = bitterness; }
    if salty > max_val { dominant = "Salty"; max_val = salty; }

    format!("[{}] Dominant Note: {}. (A: {:.2}, S: {:.2}, U: {:.2}, B: {:.2}, Sa: {:.2})", 
        texture, dominant, acidity, sweetness, umami, bitterness, salty)
}
