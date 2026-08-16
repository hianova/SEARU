use super::statics::{Bar, Node, Truss};
use crate::science::crucible::{Gene, TheCrucible};
use crate::profile::{ArchProfile, PhysicsProfile};

pub struct MechanicsOptimizer;

impl MechanicsOptimizer {
    pub fn optimize_truss(arch: &ArchProfile, _physics: &PhysicsProfile) -> Truss {
        println!("🏗️ Mechanics Engine: Optimizing 2D Truss Topology with Density {}...", arch.density);

        let mut nodes = Vec::new();
        
        // Map density (10-110) to a grid size. We'll use a tall envelope.
        let w_nodes = 3 + (arch.density / 30) as usize; // 3 to 6
        let h_nodes = 4 + (arch.density / 20) as usize; // 4 to 9
        
        let width = 100.0;
        let height = 200.0;
        let dx = width / (w_nodes - 1) as f64;
        let dy = height / (h_nodes - 1) as f64;

        let gravity_per_node = 500.0;
        let wind_force = arch.max_wind_force * 10.0; // Scaled up for effect

        for y_idx in 0..h_nodes {
            for x_idx in 0..w_nodes {
                let x = x_idx as f64 * dx;
                let y = y_idx as f64 * dy;
                let fixed = y_idx == 0; // Bottom layer is fixed to the ground
                
                let mut force_x = 0.0;
                let mut force_y = 0.0;

                if !fixed {
                    force_y = -gravity_per_node; // Gravity pulls down
                }

                // Wind force applies to the rightmost edge
                if x_idx == w_nodes - 1 && !fixed {
                    force_x = -wind_force; // Wind pushes left
                }

                nodes.push(Node {
                    x,
                    y,
                    fixed,
                    force_x,
                    force_y,
                });
            }
        }

        // Connect nodes based on proximity (Delaunay/Grid-like) to reduce O(N^2) explosion
        let mut bars = Vec::new();
        let max_dist = (dx * dx + dy * dy).sqrt() * 1.5; // Connect adjacent and diagonals

        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                let dist_x = nodes[i].x - nodes[j].x;
                let dist_y = nodes[i].y - nodes[j].y;
                let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();

                if dist <= max_dist {
                    bars.push(Bar {
                        node_a: i,
                        node_b: j,
                        area: 1.0,
                        stress: 0.0,
                    });
                }
            }
        }

        let mut genes = Vec::new();
        for i in 0..bars.len() {
            genes.push(Gene {
                name: format!("Bar_{}_Area", i),
                bounds: (0.01, 20.0), // Min area is very small to allow pruning
                current_value: 5.0,
            });
        }

        let iterations = 1000; // Keep reasonable for API response time
        let (_, best_genes) = TheCrucible::anneal(
            genes,
            |current_genes| {
                let mut cost = 0.0;

                for (i, gene) in current_genes.iter().enumerate() {
                    let area = gene.current_value;
                    let bar = &bars[i];
                    let dx_n = nodes[bar.node_a].x - nodes[bar.node_b].x;
                    let dy_n = nodes[bar.node_a].y - nodes[bar.node_b].y;
                    let length = (dx_n * dx_n + dy_n * dy_n).sqrt();

                    let mass = area * length;
                    cost += mass * 10.0; // Heavy penalty on mass to force pruning
                }

                // ---------------------------------------------------------
                // Real FEA: Direct Stiffness Method (DSM)
                // ---------------------------------------------------------
                let n_nodes = nodes.len();
                let ndof = n_nodes * 2;
                let mut k_mat = vec![vec![0.0; ndof]; ndof];
                let mut f_vec = vec![0.0; ndof];
                let e_modulus = 200_000.0; // Steel MPa

                // Assemble Global Stiffness Matrix
                for (i, gene) in current_genes.iter().enumerate() {
                    let area = gene.current_value;
                    if area < 0.1 { continue; } // Pruned bars don't contribute stiffness
                    let bar = &bars[i];
                    let n1 = bar.node_a;
                    let n2 = bar.node_b;
                    
                    let dx_n = nodes[n2].x - nodes[n1].x;
                    let dy_n = nodes[n2].y - nodes[n1].y;
                    let length = (dx_n * dx_n + dy_n * dy_n).sqrt();
                    let c = dx_n / length;
                    let s = dy_n / length;
                    
                    let k = e_modulus * area / length;
                    
                    let k_local = [
                        [c*c, c*s, -c*c, -c*s],
                        [c*s, s*s, -c*s, -s*s],
                        [-c*c, -c*s, c*c, c*s],
                        [-c*s, -s*s, c*s, s*s],
                    ];
                    
                    let dofs = [n1*2, n1*2+1, n2*2, n2*2+1];
                    for r in 0..4 {
                        for c_col in 0..4 {
                            k_mat[dofs[r]][dofs[c_col]] += k * k_local[r][c_col];
                        }
                    }
                }

                // Apply Loads
                for (i, node) in nodes.iter().enumerate() {
                    f_vec[i * 2] = node.force_x;
                    f_vec[i * 2 + 1] = node.force_y;
                }

                // Apply Boundary Conditions (Penalty Method)
                let penalty = 1e12;
                for (i, node) in nodes.iter().enumerate() {
                    if node.fixed {
                        k_mat[i * 2][i * 2] += penalty;
                        k_mat[i * 2 + 1][i * 2 + 1] += penalty;
                    }
                }

                // Solve Ku = F using Gauss-Seidel iteration (approximate for speed)
                let mut u = vec![0.0; ndof];
                for _ in 0..50 {
                    for i in 0..ndof {
                        let mut sum = f_vec[i];
                        for j in 0..ndof {
                            if i != j {
                                sum -= k_mat[i][j] * u[j];
                            }
                        }
                        if k_mat[i][i] != 0.0 {
                            u[i] = sum / k_mat[i][i];
                        }
                    }
                }

                // Penalize displacement and stress
                let mut max_disp = 0.0_f64;
                for disp in &u {
                    if disp.abs() > max_disp {
                        max_disp = disp.abs();
                    }
                }

                // If structure is too wobbly, infinite cost
                if max_disp > 50.0 {
                    cost += 1e9;
                } else {
                    cost += max_disp * 1000.0;
                }

                cost
            },
            iterations,
        );

        println!("✅ Truss Topology Optimization Complete!");

        let mut total_mass = 0.0;
        
        // Final Solve to attach true stress to bars
        let n_nodes = nodes.len();
        let ndof = n_nodes * 2;
        let mut k_mat = vec![vec![0.0; ndof]; ndof];
        let mut f_vec = vec![0.0; ndof];
        let e_modulus = 200_000.0;
        
        for (i, gene) in best_genes.iter().enumerate() {
            let area = gene.current_value;
            bars[i].area = area;
            let n1 = bars[i].node_a;
            let n2 = bars[i].node_b;
            let dx = nodes[n2].x - nodes[n1].x;
            let dy = nodes[n2].y - nodes[n1].y;
            let length = (dx * dx + dy * dy).sqrt();
            total_mass += area * length;
            
            let c = dx / length;
            let s = dy / length;
            let k = e_modulus * area / length;
            let k_local = [[c*c, c*s, -c*c, -c*s], [c*s, s*s, -c*s, -s*s], [-c*c, -c*s, c*c, c*s], [-c*s, -s*s, c*s, s*s]];
            let dofs = [n1*2, n1*2+1, n2*2, n2*2+1];
            for r in 0..4 { for c_col in 0..4 { k_mat[dofs[r]][dofs[c_col]] += k * k_local[r][c_col]; } }
        }
        for (i, node) in nodes.iter().enumerate() { f_vec[i*2+1] -= node.force_y; if node.fixed { k_mat[i*2][i*2] += 1e12; k_mat[i*2+1][i*2+1] += 1e12; } }
        
        let mut u = f_vec.clone();
        for i in 0..ndof {
            let mut max_row = i; for j in i+1..ndof { if k_mat[j][i].abs() > k_mat[max_row][i].abs() { max_row = j; } }
            k_mat.swap(i, max_row); u.swap(i, max_row);
            if k_mat[i][i].abs() < 1e-9 { continue; }
            let pivot = k_mat[i][i];
            for j in i..ndof { k_mat[i][j] /= pivot; }
            u[i] /= pivot;
            for j in 0..ndof { if i != j { let factor = k_mat[j][i]; for k in i..ndof { k_mat[j][k] -= factor * k_mat[i][k]; } u[j] -= factor * u[i]; } }
        }

        for i in 0..bars.len() {
            let bar = &bars[i];
            let n1 = bar.node_a;
            let n2 = bar.node_b;
            let dx = nodes[n2].x - nodes[n1].x;
            let dy = nodes[n2].y - nodes[n1].y;
            let length = (dx * dx + dy * dy).sqrt();
            let c = dx / length;
            let s = dy / length;
            let strain = (-c*u[n1*2] - s*u[n1*2+1] + c*u[n2*2] + s*u[n2*2+1]) / length;
            bars[i].stress = (e_modulus * strain).abs();
        }

        Truss {
            nodes,
            bars,
            total_mass,
        }
    }
}
