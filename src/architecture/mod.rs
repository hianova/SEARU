use crate::science::coevolution_context::{CoEvolutionObjective, DualChaosRunner};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Room {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

#[derive(Clone, Debug)]
pub struct ArchCandidate {
    pub rooms: Vec<Room>,
}

#[derive(Clone, Debug)]
pub struct ArchEnvironment {
    pub wind_force_x: f64,
    pub wind_force_y: f64,
}

pub struct FloorPlanner;

struct BuildingCoEvolution {
    profile: crate::profile::ArchProfile,
}

impl CoEvolutionObjective<ArchCandidate, ArchEnvironment> for BuildingCoEvolution {
    fn evaluate_fitness(&self, candidate: &ArchCandidate, env: &ArchEnvironment) -> (u32, u32) {
        let mut cost = 0.0;
        let building_w = 500.0;
        let building_h = 500.0;

        // 1. Penalize overlap and out of bounds (Basic Architecture constraints)
        for i in 0..candidate.rooms.len() {
            let r1 = &candidate.rooms[i];
            if r1.x < 0.0 {
                cost += r1.x.abs() * 100.0;
            }
            if r1.y < 0.0 {
                cost += r1.y.abs() * 100.0;
            }
            if r1.x + r1.w > building_w {
                cost += (r1.x + r1.w - building_w) * 100.0;
            }
            if r1.y + r1.h > building_h {
                cost += (r1.y + r1.h - building_h) * 100.0;
            }

            for j in i + 1..candidate.rooms.len() {
                let r2 = &candidate.rooms[j];
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

        // 2. Co-Evolution Mechanics: The building must resist the wind force.
        // Wind applies torque/moment if rooms are unbalanced.
        let mut total_area = 0.0;
        let mut cx = 0.0;
        let mut cy = 0.0;
        for r in &candidate.rooms {
            let area = r.w * r.h;
            total_area += area;
            cx += (r.x + r.w / 2.0) * area;
            cy += (r.y + r.h / 2.0) * area;
        }
        cx /= total_area.max(1.0);
        cy /= total_area.max(1.0);

        // Center of building is 250, 250. Distance between cx/cy and 250/250 determines vulnerability to wind.
        let eccentricity_x = (cx - 250.0).abs();
        let eccentricity_y = (cy - 250.0).abs();

        let wind_damage =
            (eccentricity_x * env.wind_force_x.abs()) + (eccentricity_y * env.wind_force_y.abs());
        cost += wind_damage;

        let cand_fit = cost as u32;

        // Environment wants to MAXIMIZE wind damage.
        let mut env_cost = 100000.0 - wind_damage;
        let total_wind = env.wind_force_x.abs() + env.wind_force_y.abs();
        let max_wind = self.profile.max_wind_force;
        if total_wind > max_wind {
            env_cost += (total_wind - max_wind) * 1000.0; // Over budget penalty
        }

        let env_fit = env_cost.max(0.0) as u32;

        (cand_fit, env_fit)
    }

    fn generate_candidate_seed(&self, mut seed: usize) -> ArchCandidate {
        let rand = |s: &mut usize| -> f64 {
            *s = s.wrapping_mul(1664525).wrapping_add(1013904223);
            (*s % 1000) as f64 / 1000.0
        };
        
        let mut rooms = Vec::new();
        let density = self.profile.density;
        let zoning = self.profile.zoning_ratio;
        
        for i in 0..density {
            let is_commercial = rand(&mut seed) < zoning;
            
            let name = if is_commercial {
                if rand(&mut seed) > 0.5 { "Office" } else { "Shop" }
            } else {
                if rand(&mut seed) > 0.5 { "Bed" } else { "Bath" }
            }.to_string() + &i.to_string();
            
            let base_size = if is_commercial { 80.0 } else { 30.0 };
            let var_size = if is_commercial { 150.0 } else { 70.0 };
            
            rooms.push(Room {
                name,
                x: rand(&mut seed) * 400.0,
                y: rand(&mut seed) * 400.0,
                w: base_size + rand(&mut seed) * var_size,
                h: base_size + rand(&mut seed) * var_size,
            });
        }
        ArchCandidate { rooms }
    }

    fn generate_env_seed(&self, mut seed: usize) -> ArchEnvironment {
        let rand = |s: &mut usize| -> f64 {
            *s = s.wrapping_mul(1664525).wrapping_add(1013904223);
            (*s % 1000) as f64 / 500.0 - 1.0
        };
        ArchEnvironment {
            wind_force_x: rand(&mut seed) * 50.0,
            wind_force_y: rand(&mut seed) * 50.0,
        }
    }

    fn perturb_candidate(
        &self,
        candidate: &ArchCandidate,
        scale: f32,
        mut seed: usize,
    ) -> ArchCandidate {
        let rand = |s: &mut usize| -> f64 {
            *s = s.wrapping_mul(1664525).wrapping_add(1013904223);
            (*s % 1000) as f64 / 500.0 - 1.0
        };
        let mut new_cand = candidate.clone();
        for r in &mut new_cand.rooms {
            r.x += rand(&mut seed) * scale as f64 * 10.0;
            r.y += rand(&mut seed) * scale as f64 * 10.0;
            r.w = (r.w + rand(&mut seed) * scale as f64 * 10.0).max(20.0);
            r.h = (r.h + rand(&mut seed) * scale as f64 * 10.0).max(20.0);
        }
        new_cand
    }

    fn perturb_env(&self, env: &ArchEnvironment, scale: f32, mut seed: usize) -> ArchEnvironment {
        let rand = |s: &mut usize| -> f64 {
            *s = s.wrapping_mul(1664525).wrapping_add(1013904223);
            (*s % 1000) as f64 / 500.0 - 1.0
        };
        let mut new_env = env.clone();
        new_env.wind_force_x += rand(&mut seed) * scale as f64 * 5.0;
        new_env.wind_force_y += rand(&mut seed) * scale as f64 * 5.0;
        new_env
    }

    fn check_archival(
        &self,
        _candidate: &ArchCandidate,
        _env: &ArchEnvironment,
        fitness: (u32, u32),
    ) -> bool {
        // Absolute immunity if the candidate score is extremely low despite environment's best efforts
        fitness.0 < 500
    }
}

impl FloorPlanner {
    pub fn optimize_layout(profile: crate::profile::ArchProfile) -> Vec<Room> {
        let runner = DualChaosRunner {
            max_generations: 50_000,
            nash_patience: 1000,
        };
        let (best_candidate, _best_env) = runner.launch(BuildingCoEvolution { profile });
        best_candidate.rooms
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
