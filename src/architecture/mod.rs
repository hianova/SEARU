use serde::Serialize;
use crate::science::crucible::{TheCrucible, Gene};

#[derive(Clone, Debug, Serialize)]
pub struct Room {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Room {
    pub fn new(name: impl Into<String>, x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            name: name.into(),
            x: x.max(0.0),
            y: y.max(0.0),
            w: w.max(1.0),
            h: h.max(1.0),
        }
    }
}

pub struct FloorPlanner;

impl FloorPlanner {
    pub fn optimize_layout(profile: crate::profile::ArchProfile) -> Vec<Room> {
        let mut seed: usize = 42;
        let mut rand = || -> f64 {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            (seed % 1000) as f64 / 1000.0
        };

        let density = profile.density;
        let zoning = profile.zoning_ratio;
        
        let mut initial_rooms = Vec::new();
        for i in 0..density {
            let is_commercial = rand() < zoning;
            let name = if is_commercial {
                if rand() > 0.5 { "Office" } else { "Shop" }
            } else {
                if rand() > 0.5 { "Bed" } else { "Bath" }
            }.to_string() + &i.to_string();
            initial_rooms.push(name);
        }

        let mut genes = Vec::new();
        for i in 0..density {
            genes.push(Gene { name: format!("room_{}_x", i), bounds: (0.0, 500.0), current_value: rand() * 400.0 });
            genes.push(Gene { name: format!("room_{}_y", i), bounds: (0.0, 500.0), current_value: rand() * 400.0 });
            genes.push(Gene { name: format!("room_{}_w", i), bounds: (20.0, 250.0), current_value: 50.0 + rand() * 100.0 });
            genes.push(Gene { name: format!("room_{}_h", i), bounds: (20.0, 250.0), current_value: 50.0 + rand() * 100.0 });
        }

        let (_, best_genes) = TheCrucible::anneal(
            genes,
            |g| {
                let mut cost = 0.0;
                let building_w = 500.0;
                let building_h = 500.0;
                
                let mut rooms = Vec::new();
                for i in 0..density {
                    rooms.push(Room {
                        name: String::new(),
                        x: g[i*4].current_value,
                        y: g[i*4+1].current_value,
                        w: g[i*4+2].current_value,
                        h: g[i*4+3].current_value,
                    });
                }

                for i in 0..rooms.len() {
                    let r1 = &rooms[i];
                    if r1.x < 0.0 { cost += r1.x.abs() * 100.0; }
                    if r1.y < 0.0 { cost += r1.y.abs() * 100.0; }
                    if r1.x + r1.w > building_w { cost += (r1.x + r1.w - building_w) * 100.0; }
                    if r1.y + r1.h > building_h { cost += (r1.y + r1.h - building_h) * 100.0; }

                    for j in i + 1..rooms.len() {
                        let r2 = &rooms[j];
                        let no_overlap = r1.x > r2.x + r2.w
                            || r1.x + r1.w < r2.x
                            || r1.y > r2.y + r2.h
                            || r1.y + r1.h < r2.y;
                        if !no_overlap {
                            let overlap_area = (r1.x.max(r2.x) - (r1.x + r1.w).min(r2.x + r2.w)).abs()
                                * (r1.y.max(r2.y) - (r1.y + r1.h).min(r2.y + r2.h)).abs();
                            cost += overlap_area * 100.0;
                        }
                    }
                }

                let mut total_area = 0.0;
                let mut cx = 0.0;
                let mut cy = 0.0;
                for r in &rooms {
                    let area = r.w * r.h;
                    total_area += area;
                    cx += (r.x + r.w / 2.0) * area;
                    cy += (r.y + r.h / 2.0) * area;
                }
                cx /= total_area.max(1.0);
                cy /= total_area.max(1.0);

                let eccentricity_x = (cx - 250.0).abs();
                let eccentricity_y = (cy - 250.0).abs();

                // Assume worst-case wind force of 50.0 for robustness
                let wind_damage = (eccentricity_x * 50.0) + (eccentricity_y * 50.0);
                cost += wind_damage;

                cost
            },
            10_000
        );

        let mut final_rooms = Vec::new();
        for i in 0..density {
            final_rooms.push(Room {
                name: initial_rooms[i].clone(),
                x: best_genes[i*4].current_value,
                y: best_genes[i*4+1].current_value,
                w: best_genes[i*4+2].current_value,
                h: best_genes[i*4+3].current_value,
            });
        }

        final_rooms
    }

    pub fn to_svg_string(rooms: &[Room]) -> String {
        let mut out = String::new();
        out.push_str("<svg width=\"500\" height=\"500\" xmlns=\"http://www.w3.org/2000/svg\">\n");
        out.push_str("  <rect width=\"100%\" height=\"100%\" fill=\"#1e293b\" />\n");

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
