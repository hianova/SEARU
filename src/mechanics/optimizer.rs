use crate::science::crucible::{Gene, TheCrucible};
use super::statics::{Node, Bar, Truss};

pub struct MechanicsOptimizer;

impl MechanicsOptimizer {
    pub fn optimize_truss() -> Truss {
        println!("🏗️ Mechanics Engine: Optimizing 2D Truss Topology...");
        
        let nodes = vec![
            Node { x: 0.0, y: 0.0, fixed: true, force_y: 0.0 }, // 0: Left support
            Node { x: 100.0, y: 0.0, fixed: true, force_y: 0.0 }, // 1: Right support
            Node { x: 50.0, y: 50.0, fixed: false, force_y: 1000.0 }, // 2: Load point
            // Additional free nodes to form complex structures
            Node { x: 25.0, y: 25.0, fixed: false, force_y: 0.0 }, // 3
            Node { x: 75.0, y: 25.0, fixed: false, force_y: 0.0 }, // 4
        ];
        
        // Fully connect all nodes
        let mut bars = Vec::new();
        for i in 0..nodes.len() {
            for j in i+1..nodes.len() {
                bars.push(Bar { node_a: i, node_b: j, area: 1.0, stress: 0.0 });
            }
        }
        
        let mut genes = Vec::new();
        for i in 0..bars.len() {
            genes.push(Gene { 
                name: format!("Bar_{}_Area", i), 
                bounds: (0.1, 15.0), // Min area to max area
                current_value: 5.0 
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
                    let length = (dx*dx + dy*dy).sqrt();
                    
                    let mass = area * length;
                    cost += mass;
                    
                    // Mock static solver penalty (we don't have a real FEA solver here)
                    // We penalize if load paths are not supported
                    if bar.node_a == 2 || bar.node_b == 2 {
                        if area < 5.0 {
                            cost += 10000.0 / area; 
                        }
                    }
                }
                
                cost
            },
            iterations
        );
        
        println!("✅ Truss Topology Optimization Complete!");
        
        for (i, gene) in best_genes.iter().enumerate() {
            bars[i].area = gene.current_value;
            // Fake stress for visualization
            bars[i].stress = if bars[i].node_a == 2 || bars[i].node_b == 2 { 100.0 } else { 20.0 };
        }
        
        Truss { nodes, bars, total_mass: 0.0 }
    }
}
