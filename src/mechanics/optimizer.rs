use super::statics::{Bar, Node, Truss};
use crate::science::crucible::{Gene, TheCrucible};

pub struct MechanicsOptimizer;

impl MechanicsOptimizer {
    pub fn optimize_truss() -> Truss {
        println!("🏗️ Mechanics Engine: Optimizing 2D Truss Topology...");

        let nodes = vec![
            Node {
                x: 0.0,
                y: 0.0,
                fixed: true,
                force_y: 0.0,
            }, // 0: Left support
            Node {
                x: 100.0,
                y: 0.0,
                fixed: true,
                force_y: 0.0,
            }, // 1: Right support
            Node {
                x: 50.0,
                y: 50.0,
                fixed: false,
                force_y: 1000.0,
            }, // 2: Load point
            // Additional free nodes to form complex structures
            Node {
                x: 25.0,
                y: 25.0,
                fixed: false,
                force_y: 0.0,
            }, // 3
            Node {
                x: 75.0,
                y: 25.0,
                fixed: false,
                force_y: 0.0,
            }, // 4
        ];

        // Fully connect all nodes
        let mut bars = Vec::new();
        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                bars.push(Bar {
                    node_a: i,
                    node_b: j,
                    area: 1.0,
                    stress: 0.0,
                });
            }
        }

        let mut genes = Vec::new();
        for i in 0..bars.len() {
            genes.push(Gene {
                name: format!("Bar_{}_Area", i),
                bounds: (0.1, 15.0), // Min area to max area
                current_value: 5.0,
            });
        }

        let iterations = 10000;
        let (_, best_genes) = TheCrucible::anneal(
            genes,
            |current_genes| {
                let mut cost = 0.0;

                for (i, gene) in current_genes.iter().enumerate() {
                    let area = gene.current_value;
                    let bar = &bars[i];
                    let dx = nodes[bar.node_a].x - nodes[bar.node_b].x;
                    let dy = nodes[bar.node_a].y - nodes[bar.node_b].y;
                    let length = (dx * dx + dy * dy).sqrt();

                    let mass = area * length;
                    cost += mass;
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
                    let bar = &bars[i];
                    let n1 = bar.node_a;
                    let n2 = bar.node_b;
                    
                    let dx = nodes[n2].x - nodes[n1].x;
                    let dy = nodes[n2].y - nodes[n1].y;
                    let length = (dx * dx + dy * dy).sqrt();
                    let c = dx / length;
                    let s = dy / length;
                    
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

                // Apply External Forces
                for (i, node) in nodes.iter().enumerate() {
                    f_vec[i*2+1] -= node.force_y; // Downward force
                }

                // Apply Boundary Conditions (Penalty Method)
                for (i, node) in nodes.iter().enumerate() {
                    if node.fixed {
                        let penalty = 1e12;
                        k_mat[i*2][i*2] += penalty;
                        k_mat[i*2+1][i*2+1] += penalty;
                    }
                }

                // Solve K * U = F using Gaussian Elimination
                let mut u = f_vec.clone();
                for i in 0..ndof {
                    // Pivot
                    let mut max_row = i;
                    for j in i+1..ndof {
                        if k_mat[j][i].abs() > k_mat[max_row][i].abs() {
                            max_row = j;
                        }
                    }
                    k_mat.swap(i, max_row);
                    u.swap(i, max_row);
                    
                    if k_mat[i][i].abs() < 1e-9 { continue; } // Singular
                    
                    let pivot = k_mat[i][i];
                    for j in i..ndof { k_mat[i][j] /= pivot; }
                    u[i] /= pivot;
                    
                    for j in 0..ndof {
                        if i != j {
                            let factor = k_mat[j][i];
                            for k in i..ndof {
                                k_mat[j][k] -= factor * k_mat[i][k];
                            }
                            u[j] -= factor * u[i];
                        }
                    }
                }

                // Calculate Stress Penalty
                let yield_stress = 250.0; // MPa
                for (i, gene) in current_genes.iter().enumerate() {
                    let bar = &bars[i];
                    let n1 = bar.node_a;
                    let n2 = bar.node_b;
                    let dx = nodes[n2].x - nodes[n1].x;
                    let dy = nodes[n2].y - nodes[n1].y;
                    let length = (dx * dx + dy * dy).sqrt();
                    let c = dx / length;
                    let s = dy / length;
                    
                    let u1 = u[n1*2];
                    let v1 = u[n1*2+1];
                    let u2 = u[n2*2];
                    let v2 = u[n2*2+1];
                    
                    // stress = E/L * [ -c -s c s ] * [u1, v1, u2, v2]^T
                    let strain = (-c*u1 - s*v1 + c*u2 + s*v2) / length;
                    let stress = (e_modulus * strain).abs();
                    
                    if stress > yield_stress {
                        cost += (stress - yield_stress).powi(2) * 10.0; // Heavy penalty for breaking
                    }
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
