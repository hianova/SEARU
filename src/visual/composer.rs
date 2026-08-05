use crate::science::crucible::{Gene, TheCrucible};
use super::geometry::{Point, Color, Shape};

pub struct VisualComposer;

impl VisualComposer {
    pub fn generate_art(num_shapes: usize, points_per_shape: usize) -> Vec<Shape> {
        println!("🎨 Visual Engine: Generating Generative SVG Art with {} shapes...", num_shapes);
        
        let mut genes = Vec::new();
        
        for s in 0..num_shapes {
            for p in 0..points_per_shape {
                genes.push(Gene { name: format!("S{}_P{}_X", s, p), bounds: (50.0, 750.0), current_value: 400.0 });
                genes.push(Gene { name: format!("S{}_P{}_Y", s, p), bounds: (50.0, 550.0), current_value: 300.0 });
            }
            genes.push(Gene { name: format!("S{}_H", s), bounds: (0.0, 360.0), current_value: 180.0 });
            genes.push(Gene { name: format!("S{}_S", s), bounds: (0.3, 1.0), current_value: 0.8 });
            genes.push(Gene { name: format!("S{}_L", s), bounds: (0.3, 0.8), current_value: 0.5 });
        }
        
        let iterations = 10000;
        let (_, best_genes) = TheCrucible::anneal(
            genes,
            |current_genes| {
                let shapes = Self::decode(current_genes, num_shapes, points_per_shape);
                let mut cost = 0.0;
                
                // Rule 1: Distribute shapes nicely (repulsion)
                for i in 0..shapes.len() {
                    for j in i+1..shapes.len() {
                        let c1 = Self::center(&shapes[i]);
                        let c2 = Self::center(&shapes[j]);
                        let dist = ((c1.x - c2.x).powi(2) + (c1.y - c2.y).powi(2)).sqrt();
                        if dist < 150.0 {
                            cost += 2000.0 / (dist + 1.0); // Penalty for being too close
                        }
                    }
                }
                
                // Rule 2: Shape regularity
                for shape in &shapes {
                    let center = Self::center(shape);
                    let mut max_radius = 0.0_f64;
                    let mut min_radius = f64::MAX;
                    for p in &shape.points {
                        let r = ((p.x - center.x).powi(2) + (p.y - center.y).powi(2)).sqrt();
                        if r > max_radius { max_radius = r; }
                        if r < min_radius { min_radius = r; }
                    }
                    cost += (max_radius - min_radius).abs() * 0.5;
                }
                
                // Rule 3: Color harmony
                if shapes.len() >= 2 {
                    for i in 0..shapes.len()-1 {
                        let diff = (shapes[i].color.h - shapes[i+1].color.h).abs();
                        // Analogous (<45) or Complementary (~180)
                        if diff > 45.0 && (diff - 180.0).abs() > 45.0 {
                            cost += 100.0;
                        }
                    }
                }

                cost
            },
            iterations
        );
        
        println!("✅ Visual Art Discovery Complete!");
        Self::decode(&best_genes, num_shapes, points_per_shape)
    }
    
    fn decode(genes: &[Gene], num_shapes: usize, points_per_shape: usize) -> Vec<Shape> {
        let mut shapes = Vec::new();
        let mut idx = 0;
        
        for _ in 0..num_shapes {
            let mut points = Vec::new();
            for _ in 0..points_per_shape {
                points.push(Point { x: genes[idx].current_value, y: genes[idx+1].current_value });
                idx += 2;
            }
            let color = Color {
                h: genes[idx].current_value,
                s: genes[idx+1].current_value,
                l: genes[idx+2].current_value,
            };
            idx += 3;
            shapes.push(Shape { points, color });
        }
        
        shapes
    }
    
    fn center(shape: &Shape) -> Point {
        let mut cx = 0.0;
        let mut cy = 0.0;
        for p in &shape.points {
            cx += p.x;
            cy += p.y;
        }
        let len = shape.points.len() as f64;
        Point { x: cx / len, y: cy / len }
    }
}
