use crate::science::crucible::{Gene, TheCrucible};

#[derive(Clone, Debug)]
pub struct Room {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

pub struct FloorPlanner;

impl FloorPlanner {
    pub fn optimize_layout() -> Vec<Room> {
        let room_names = vec!["Living Room", "Kitchen", "Bedroom 1", "Bedroom 2", "Bathroom"];
        let mut genes = Vec::new();
        
        for (i, _name) in room_names.iter().enumerate() {
            genes.push(Gene { name: format!("R{}_X", i), bounds: (0.0, 400.0), current_value: 100.0 });
            genes.push(Gene { name: format!("R{}_Y", i), bounds: (0.0, 400.0), current_value: 100.0 });
            genes.push(Gene { name: format!("R{}_W", i), bounds: (50.0, 200.0), current_value: 100.0 });
            genes.push(Gene { name: format!("R{}_H", i), bounds: (50.0, 200.0), current_value: 100.0 });
        }
        
        let building_w = 500.0;
        let building_h = 500.0;
        
        let (_, best_genes) = TheCrucible::anneal(
            genes,
            |g| {
                let mut rooms = Vec::new();
                for i in 0..room_names.len() {
                    let idx = i * 4;
                    rooms.push((g[idx].current_value, g[idx+1].current_value, g[idx+2].current_value, g[idx+3].current_value));
                }
                
                let mut cost = 0.0;
                
                for i in 0..rooms.len() {
                    let r1 = rooms[i];
                    
                    // Penalize out of bounds
                    if r1.0 < 0.0 { cost += r1.0.abs() * 100.0; }
                    if r1.1 < 0.0 { cost += r1.1.abs() * 100.0; }
                    if r1.0 + r1.2 > building_w { cost += (r1.0 + r1.2 - building_w) * 100.0; }
                    if r1.1 + r1.3 > building_h { cost += (r1.1 + r1.3 - building_h) * 100.0; }
                    
                    // Penalize overlap
                    for j in i+1..rooms.len() {
                        let r2 = rooms[j];
                        let no_overlap = r1.0 > r2.0 + r2.2 || r1.0 + r1.2 < r2.0 || r1.1 > r2.1 + r2.3 || r1.1 + r1.3 < r2.1;
                        if !no_overlap {
                            let overlap_area = (r1.0.max(r2.0) - (r1.0 + r1.2).min(r2.0 + r2.2)).abs() *
                                               (r1.1.max(r2.1) - (r1.1 + r1.3).min(r2.1 + r2.3)).abs();
                            cost += overlap_area * 1000.0;
                        }
                    }
                }
                
                cost
            },
            10000
        );
        
        let mut result = Vec::new();
        for (i, name) in room_names.iter().enumerate() {
            let idx = i * 4;
            result.push(Room {
                name: name.to_string(),
                x: best_genes[idx].current_value,
                y: best_genes[idx+1].current_value,
                w: best_genes[idx+2].current_value,
                h: best_genes[idx+3].current_value,
            });
        }
        result
    }

    pub fn to_svg_string(rooms: &[Room]) -> String {
        let mut out = String::new();
        out.push_str("<svg width=\"500\" height=\"500\" xmlns=\"http://www.w3.org/2000/svg\">\n");
        out.push_str("  <rect width=\"100%\" height=\"100%\" fill=\"#e2e8f0\" />\n");
        
        let colors = ["#ef4444", "#3b82f6", "#10b981", "#f59e0b", "#8b5cf6"];
        for (i, r) in rooms.iter().enumerate() {
            out.push_str(&format!(
                "  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\" stroke=\"#fff\" stroke-width=\"2\" opacity=\"0.8\" />\n",
                r.x, r.y, r.w, r.h, colors[i % colors.len()]
            ));
            out.push_str(&format!(
                "  <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"sans-serif\" font-size=\"14\" fill=\"#fff\">{}</text>\n",
                r.x + 10.0, r.y + 20.0, r.name
            ));
        }
        
        out.push_str("</svg>\n");
        out
    }
}
