use super::geometry::{Color, Point, Shape, ShapeType};
use crate::science::chaos_state::{ChaosState, MicroTweak, RngState, step_forward_nd};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::f64::consts::PI;

pub struct VisualComposer;

impl VisualComposer {
    pub fn generate_art(seed: usize, track_name: &str) -> Vec<Shape> {
        println!("🎨 Visual Engine: Igniting Kaleidoscope Chaos for {}...", track_name);
        
        let mut rng_std = StdRng::seed_from_u64(seed as u64 + 777);
        let hue_base = rng_std.random_range(0.0..360.0);
        let mut shapes = Vec::new();
        let num_symmetry = 6; // Hexagonal radial symmetry (Mandala effect)

        // Reduced from 3 strands to 2 to prevent excessive center tangling
        for i in 0..2 {
            let strand_hue = (hue_base + (i as f64 * 60.0)) % 360.0;
            
            let mut chaos = ChaosState::<3, 2>::new([0.0, 0.0]);
            let mut chaos_rng = RngState::new(seed as u32 + i as u32 * 100);
            
            let tweak = MicroTweak {
                s_exponent: 1.2 + rng_std.random_range(0.0_f32..0.5_f32), // More long jumps, less local clustering
                max_elements: 1000,
            };

            let num_steps = 300; // Reduced from 800 to prevent the 'yarn ball' effect in the center
            let mut raw_points = Vec::with_capacity(num_steps);
            
            for _ in 0..num_steps {
                chaos = step_forward_nd(&chaos, &tweak, &mut chaos_rng);
                raw_points.push(Point {
                    x: chaos.base_values[0] as f64,
                    y: chaos.base_values[1] as f64,
                });
            }

            // Normalize points
            let (mut min_x, mut max_x) = (f64::MAX, f64::MIN);
            let (mut min_y, mut max_y) = (f64::MAX, f64::MIN);
            
            for p in &raw_points {
                if p.x < min_x { min_x = p.x; }
                if p.x > max_x { max_x = p.x; }
                if p.y < min_y { min_y = p.y; }
                if p.y > max_y { max_y = p.y; }
            }

            let range_x = max_x - min_x;
            let range_y = max_y - min_y;
            let max_range = range_x.max(range_y);
            
            // Keep the chaos somewhat centered
            let padding = 150.0;
            let canvas_size = 800.0 - (padding * 2.0);
            let scale = if max_range > 0.0 { canvas_size / max_range } else { 1.0 };
            
            let mut normalized_points = Vec::with_capacity(num_steps);
            for p in &raw_points {
                let nx = (p.x - min_x) - (range_x / 2.0);
                let ny = (p.y - min_y) - (range_y / 2.0);
                normalized_points.push(Point {
                    x: nx * scale,
                    y: ny * scale,
                });
            }

            // Apply Radial Symmetry and Smoothing
            for sym in 0..num_symmetry {
                let angle = (sym as f64) * (2.0 * PI / num_symmetry as f64);
                let cos_a = angle.cos();
                let sin_a = angle.sin();
                let center = 400.0;

                let mut rotated_points = Vec::with_capacity(num_steps);
                for p in &normalized_points {
                    // Rotate around origin, then translate to center
                    let rx = p.x * cos_a - p.y * sin_a;
                    let ry = p.x * sin_a + p.y * cos_a;
                    rotated_points.push(Point {
                        x: rx + center,
                        y: ry + center,
                    });
                }

                // 1. Draw smooth bezier path
                let mut path_d = String::new();
                for (j, p) in rotated_points.iter().enumerate() {
                    if j == 0 {
                        path_d.push_str(&format!("M {:.1} {:.1} ", p.x, p.y));
                    } else if j < rotated_points.len() - 1 {
                        // Quadratic bezier smoothing: use midpoint as the target, and current point as control
                        let next = &rotated_points[j + 1];
                        let mid_x = (p.x + next.x) / 2.0;
                        let mid_y = (p.y + next.y) / 2.0;
                        path_d.push_str(&format!("Q {:.1} {:.1} {:.1} {:.1} ", p.x, p.y, mid_x, mid_y));
                    } else {
                        path_d.push_str(&format!("L {:.1} {:.1} ", p.x, p.y));
                    }
                }

                shapes.push(Shape {
                    shape_type: ShapeType::Path(path_d),
                    color: Color { h: strand_hue, s: 0.9, l: 0.6 },
                    fill_opacity: 0.0,
                    stroke_color: Some(Color { h: strand_hue, s: 1.0, l: 0.7 }),
                    stroke_width: 0.2, // Ultra-thin line to reduce center clutter
                });

                // 2. Add Bokeh Particles at random intervals, but ONLY outside the center!
                for (j, p) in rotated_points.iter().enumerate() {
                    if j % 8 == 0 { // Increased frequency since we have fewer steps
                        let dist_to_center = ((p.x - center).powi(2) + (p.y - center).powi(2)).sqrt();
                        
                        // Keep the center clean (no bokeh inside radius 80)
                        if dist_to_center > 80.0 {
                            let radius = 5.0 + (dist_to_center / 30.0);
                            
                            shapes.push(Shape {
                                shape_type: ShapeType::Circle { center: p.clone(), radius },
                                color: Color { h: strand_hue, s: 0.9, l: 0.8 },
                                fill_opacity: 0.06, // Slightly brighter since there are fewer
                                stroke_color: None,
                                stroke_width: 0.0,
                            });
                        }
                    }
                }
            }
        }
        
        // Add track label
        shapes.push(Shape {
            shape_type: ShapeType::Text {
                pos: Point { x: 40.0, y: 740.0 },
                text: format!("SEARU // {}", track_name.to_uppercase()),
                font_size: 32.0,
            },
            color: Color { h: hue_base, s: 0.1, l: 0.9 },
            fill_opacity: 1.0,
            stroke_color: None,
            stroke_width: 0.0,
        });
        
        shapes
    }
}
