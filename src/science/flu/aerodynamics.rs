use crate::science::ScienceObjective;

pub const N_POINTS: usize = 128; // X dimension
pub const NY: usize = 32; // Y dimension
pub const MAX_HEIGHT: usize = 16;

#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceState {
    pub heights: [f32; N_POINTS],
}

pub struct FluidObjective;

impl Default for FluidObjective {
    fn default() -> Self {
        Self::new()
    }
}

impl FluidObjective {
    pub fn new() -> Self {
        Self
    }
}

// LBM D2Q9 Weights and Directions
#[allow(dead_code)]
const W: [f32; 9] = [
    4.0 / 9.0,
    1.0 / 9.0,
    1.0 / 9.0,
    1.0 / 9.0,
    1.0 / 9.0,
    1.0 / 36.0,
    1.0 / 36.0,
    1.0 / 36.0,
    1.0 / 36.0,
];
const CX: [i32; 9] = [0, 1, 0, -1, 0, 1, -1, -1, 1];
const CY: [i32; 9] = [0, 0, 1, 0, -1, 1, 1, -1, -1];
const OPP: [usize; 9] = [0, 3, 4, 1, 2, 7, 8, 5, 6];

impl ScienceObjective<SurfaceState> for FluidObjective {
    fn evaluate_fitness(&self, candidate: &SurfaceState) -> (u32, u32) {
        let omega_q16: i32 = 109226; // 1.0 / 0.6 = 1.6666667 in Q16
        let u0_q16: i32 = 6553; // 0.1 in Q16
        let steps = 400;

        let w_q16: [i32; 9] = [
            29127, // 4/9
            7281, 7281, 7281, 7281, // 1/9
            1820, 1820, 1820, 1820, // 1/36
        ];

        let mut solid = [[false; NY]; N_POINTS];
        let mut total_material_q16: i32 = 0;
        for x in 0..N_POINTS {
            let h = (candidate.heights[x] * MAX_HEIGHT as f32)
                .max(0.0)
                .min((MAX_HEIGHT - 1) as f32) as usize;
            for y in 0..=h {
                solid[x][y] = true;
            }
            total_material_q16 += (candidate.heights[x] * 65536.0) as i32;
        }

        let mut f = [[[0_i32; 9]; NY]; N_POINTS];
        for x in 0..N_POINTS {
            for y in 0..NY {
                for i in 0..9 {
                    let cu = u0_q16 * CX[i];
                    let u2 = ((u0_q16 as i64 * u0_q16 as i64) >> 16) as i32;
                    let cu2 = ((cu as i64 * cu as i64) >> 16) as i32;

                    let term1 = 65536;
                    let term2 = cu * 3;
                    let term3 = ((cu2 as i64 * 294912) >> 16) as i32; // 4.5
                    let term4 = ((u2 as i64 * 98304) >> 16) as i32; // 1.5

                    let bracket = term1 + term2 + term3 - term4;
                    f[x][y][i] = ((w_q16[i] as i64 * bracket as i64) >> 16) as i32;
                }
            }
        }

        let mut drag_force_x_q16: i64 = 0;

        for _step in 0..steps {
            let mut f_new = f;

            for x in 0..N_POINTS {
                for y in 0..NY {
                    if solid[x][y] {
                        continue;
                    }

                    let mut rho = 0;
                    let mut ux = 0;
                    let mut uy = 0;
                    for i in 0..9 {
                        rho += f[x][y][i];
                        ux += f[x][y][i] * CX[i];
                        uy += f[x][y][i] * CY[i];
                    }

                    if rho > 0 {
                        ux = (((ux as i64) << 16) / rho as i64) as i32;
                        uy = (((uy as i64) << 16) / rho as i64) as i32;
                    }

                    let u2 = ((ux as i64 * ux as i64) >> 16) as i32
                        + ((uy as i64 * uy as i64) >> 16) as i32;
                    let mut f_eq = [0_i32; 9];

                    for i in 0..9 {
                        let cu = ux * CX[i] + uy * CY[i];
                        let cu2 = ((cu as i64 * cu as i64) >> 16) as i32;

                        let term1 = 65536;
                        let term2 = cu * 3;
                        let term3 = ((cu2 as i64 * 294912) >> 16) as i32;
                        let term4 = ((u2 as i64 * 98304) >> 16) as i32;

                        let bracket = term1 + term2 + term3 - term4;
                        let rho_w = ((rho as i64 * w_q16[i] as i64) >> 16) as i32;
                        f_eq[i] = ((rho_w as i64 * bracket as i64) >> 16) as i32;

                        let diff = f[x][y][i] - f_eq[i];
                        let relax = ((diff as i64 * omega_q16 as i64) >> 16) as i32;
                        let post_collision = f[x][y][i] - relax;

                        let nx = x as i32 + CX[i];
                        let ny = y as i32 + CY[i];

                        if nx >= 0 && nx < N_POINTS as i32 && ny >= 0 && ny < NY as i32 {
                            let nux = nx as usize;
                            let nuy = ny as usize;
                            if solid[nux][nuy] {
                                f_new[x][y][OPP[i]] = post_collision;
                                drag_force_x_q16 += post_collision as i64 * 2 * CX[i] as i64;
                            } else {
                                f_new[nux][nuy][i] = post_collision;
                            }
                        }
                    }
                }
            }

            for y in 0..NY {
                if !solid[0][y] {
                    for i in 0..9 {
                        let cu = u0_q16 * CX[i];
                        let u2 = ((u0_q16 as i64 * u0_q16 as i64) >> 16) as i32;
                        let cu2 = ((cu as i64 * cu as i64) >> 16) as i32;

                        let bracket = 65536 + cu * 3 + ((cu2 as i64 * 294912) >> 16) as i32
                            - ((u2 as i64 * 98304) >> 16) as i32;
                        f_new[0][y][i] = ((w_q16[i] as i64 * bracket as i64) >> 16) as i32;
                    }
                }
                for i in 0..9 {
                    f_new[N_POINTS - 1][y][i] = f_new[N_POINTS - 2][y][i];
                }
            }

            for x in 0..N_POINTS {
                f_new[x][NY - 1][4] = f_new[x][NY - 2][4];
                f_new[x][NY - 1][7] = f_new[x][NY - 2][7];
                f_new[x][NY - 1][8] = f_new[x][NY - 2][8];
            }

            f = f_new;
        }

        let drag_force_f32 = drag_force_x_q16 as f32 / 65536.0;
        let total_material = total_material_q16 as f32 / 65536.0;

        let volume_penalty = if total_material < (N_POINTS as f32) * 0.2 {
            ((N_POINTS as f32) * 0.2 - total_material) * 1000.0
        } else {
            0.0
        };

        let mut smoothness_penalty = 0.0;
        for x in 0..N_POINTS - 1 {
            let diff = candidate.heights[x] - candidate.heights[x + 1];
            smoothness_penalty += diff * diff;
        }

        let score_f32 = drag_force_f32 * 10.0 + volume_penalty + smoothness_penalty * 50.0;
        let score = if score_f32.is_nan() {
            1000000
        } else {
            score_f32.max(0.0) as u32
        };
        (score, score)
    }

    fn generate_seed(&self, mut seed: usize, parent: Option<&SurfaceState>) -> SurfaceState {
        if let Some(p) = parent {
            return p.clone();
        }
        let mut heights = [0.0; N_POINTS];
        for i in 0..N_POINTS {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let val = (seed % 1000) as f32 / 1000.0;
            // Smooth it out a bit initially
            heights[i] = 0.2 + val * 0.3;
        }
        SurfaceState { heights }
    }

    fn perturb(&self, candidate: &SurfaceState, scale: f32, mut seed: usize) -> SurfaceState {
        let mut child = candidate.clone();
        let num_mutations = (scale * 20.0).max(1.0).ceil() as usize;

        for _ in 0..num_mutations {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let center = seed % N_POINTS;

            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let noise = (((seed % 2000) as f32 / 1000.0) - 1.0) * scale * 0.2; // -0.2 to 0.2

            // Apply gaussian bump mutation to keep the surface physically realizable
            let width = 5.0;
            for i in 0..N_POINTS {
                let dist = (i as i32 - center as i32).abs() as f32;
                let bump = noise * (-dist * dist / width).exp();
                child.heights[i] = (child.heights[i] + bump).clamp(0.0, 1.0);
            }
        }
        child
    }

    fn is_valid(&self, _candidate: &SurfaceState) -> bool {
        true
    }

    fn check_archival(&self, _candidate: &SurfaceState, fitness: (u32, u32)) -> bool {
        if fitness.0 == 0 {
            return true;
        }
        false
    }

    fn periodic_validate_and_visualize(&self, candidate: &SurfaceState) {
        let json_out = format!("{{\"heights\":{:?}}}", candidate.heights);
        let _ = std::fs::write("data/research/aerodynamics_best.json", json_out);

        let py_script = r#"
import json
import matplotlib.pyplot as plt
import numpy as np
plt.style.use('dark_background')
with open('data/research/aerodynamics_best.json', 'r') as f:
    data = json.load(f)
y = data['heights']
x = np.linspace(0, len(y), len(y))
fig, ax = plt.subplots(figsize=(10, 5))
ax.plot(x, y, color='#00ffcc', linewidth=3)
ax.fill_between(x, y, 0, color='#00ffcc', alpha=0.3)
ax.set_title("LBM Aerodynamic Surface Profile")
ax.set_xlabel("X (Grid Units)")
ax.set_ylabel("Height")
ax.grid(color='#333333', linestyle='--')
plt.savefig('data/research/aerodynamics.jpg', dpi=150, bbox_inches='tight')
"#;
        std::thread::spawn(move || {
            let _ = std::process::Command::new("/Users/kuangtalin/.gemini/antigravity/brain/12fceec4-4945-4ad6-ab94-814e41f7cd12/scratch/.venv/bin/python")
                .arg("-c")
                .arg(py_script)
                .spawn();
        });
    }
}
