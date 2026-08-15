use super::geometry::{Color, Point, Shape, ShapeType};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub struct VisualComposer;

impl VisualComposer {
    fn draw_branch(
        shapes: &mut Vec<Shape>,
        rng: &mut StdRng,
        x: f64,
        y: f64,
        length: f64,
        angle: f64,
        depth: usize,
        hue_base: f64,
    ) {
        if depth == 0 {
            return;
        }

        let x_end = x + length * angle.cos();
        let y_end = y + length * angle.sin();

        let stroke_w = (depth as f64) * 0.8;
        // The trunk (high depth) should be darker, the leaves (low depth) brighter
        let depth_norm = depth as f64 / 12.0;
        let lightness = 0.9 - depth_norm * 0.6;
        let hue = (hue_base + (12.0 - depth as f64) * 10.0) % 360.0;

        shapes.push(Shape {
            shape_type: ShapeType::Path(format!("M {:.2} {:.2} L {:.2} {:.2}", x, y, x_end, y_end)),
            color: Color {
                h: hue,
                s: 0.8,
                l: lightness,
            },
            fill_opacity: 0.0,
            stroke_color: Some(Color {
                h: hue,
                s: 0.8,
                l: lightness,
            }),
            stroke_width: stroke_w.max(0.5),
        });

        // Randomize spread and length a bit for organic look
        let spread1 = 0.35 + rng.random_range(-0.1..0.15);
        let spread2 = 0.35 + rng.random_range(-0.1..0.15);
        let length_factor = 0.75 + rng.random_range(-0.05..0.05);

        Self::draw_branch(
            shapes,
            rng,
            x_end,
            y_end,
            length * length_factor,
            angle - spread1,
            depth - 1,
            hue_base,
        );
        Self::draw_branch(
            shapes,
            rng,
            x_end,
            y_end,
            length * length_factor,
            angle + spread2,
            depth - 1,
            hue_base,
        );
    }

    pub fn generate_art(
        seed: usize,
        track_name: &str,
        _cost_scores: &[f64],
        profile: &crate::profile::VisualProfile,
    ) -> Vec<Shape> {
        println!(
            "🌱 Visual Engine: Growing Fractal Tree for {}...",
            track_name
        );

        let mut rng_std = StdRng::seed_from_u64(seed as u64 + 777);
        let hue_base = profile.base_hue;
        let mut shapes = Vec::new();

        // Background
        shapes.push(Shape {
            shape_type: ShapeType::Rect {
                pos: Point { x: 0.0, y: 0.0 },
                width: 800.0,
                height: 800.0,
                rx: 0.0,
            },
            color: Color {
                h: hue_base,
                s: 0.05,
                l: 0.04,
            }, // Dark background
            fill_opacity: 1.0,
            stroke_color: None,
            stroke_width: 0.0,
        });

        // Horizon Line
        let horizon_y = 650.0;
        shapes.push(Shape {
            shape_type: ShapeType::Path(format!("M 0.0 {:.2} L 800.0 {:.2}", horizon_y, horizon_y)),
            color: Color {
                h: hue_base,
                s: 0.5,
                l: 0.3,
            },
            fill_opacity: 0.0,
            stroke_color: Some(Color {
                h: hue_base,
                s: 0.5,
                l: 0.3,
            }),
            stroke_width: 2.0,
        });

        // Generate the Fractal Tree
        let tree_depth = profile.fractal_depth; // Configurable complexity
        let initial_length = 130.0;
        let root_x = 400.0;
        let root_y = horizon_y;
        let root_angle = -std::f64::consts::PI / 2.0; // Pointing straight up

        Self::draw_branch(
            &mut shapes,
            &mut rng_std,
            root_x,
            root_y,
            initial_length,
            root_angle,
            tree_depth,
            hue_base,
        );

        // Add track label
        shapes.push(Shape {
            shape_type: ShapeType::Text {
                pos: Point { x: 40.0, y: 740.0 },
                text: format!("SEARU // {} // FRACTAL GROWTH", track_name.to_uppercase()),
                font_size: 28.0,
            },
            color: Color {
                h: hue_base,
                s: 0.3,
                l: 0.7,
            },
            fill_opacity: 1.0,
            stroke_color: None,
            stroke_width: 0.0,
        });

        shapes
    }
}
