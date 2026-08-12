use super::geometry::{Color, Shape, ShapeType};
use std::fs::File;
use std::io::Write;

pub struct SvgExporter;

impl SvgExporter {
    pub fn to_svg_string(shapes: &[Shape]) -> String {
        let mut out = String::new();
        out.push_str("<svg width=\"800\" height=\"800\" viewBox=\"0 0 800 800\" xmlns=\"http://www.w3.org/2000/svg\">\n");
        out.push_str(&Self::to_svg_elements(shapes));
        out.push_str("</svg>\n");
        out
    }

    pub fn to_svg_elements(shapes: &[Shape]) -> String {
        let mut out = String::new();
        
        // Add defs for dark gradients and glow filters
        out.push_str("  <defs>\n");
        out.push_str("    <linearGradient id=\"bgGradient\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"100%\">\n");
        out.push_str("      <stop offset=\"0%\" stop-color=\"#0a0a0f\" />\n");
        out.push_str("      <stop offset=\"100%\" stop-color=\"#1f1f2e\" />\n");
        out.push_str("    </linearGradient>\n");
        out.push_str("    <filter id=\"glow\" x=\"-20%\" y=\"-20%\" width=\"140%\" height=\"140%\">\n");
        out.push_str("      <feGaussianBlur stdDeviation=\"4\" result=\"blur\" />\n");
        out.push_str("      <feComposite in=\"SourceGraphic\" in2=\"blur\" operator=\"over\" />\n");
        out.push_str("    </filter>\n");
        out.push_str("  </defs>\n");
        
        // Background
        out.push_str("  <rect width=\"100%\" height=\"100%\" fill=\"url(#bgGradient)\" />\n");

        for shape in shapes {
            let fill_hex = Self::hsl_to_rgb(&shape.color);
            let stroke_attr = if let Some(ref sc) = shape.stroke_color {
                format!("stroke=\"{}\" stroke-width=\"{:.1}\"", Self::hsl_to_rgb(sc), shape.stroke_width)
            } else {
                String::new()
            };
            let fill_attr = if shape.fill_opacity > 0.0 {
                format!("fill=\"{}\" fill-opacity=\"{:.2}\"", fill_hex, shape.fill_opacity)
            } else {
                "fill=\"none\"".to_string()
            };

            match &shape.shape_type {
                ShapeType::Polygon(points) => {
                    let mut path = String::new();
                    for (i, p) in points.iter().enumerate() {
                        if i == 0 { path.push_str(&format!("M {:.1} {:.1} ", p.x, p.y)); }
                        else { path.push_str(&format!("L {:.1} {:.1} ", p.x, p.y)); }
                    }
                    path.push_str("Z");
                    out.push_str(&format!("  <path d=\"{}\" {} {} />\n", path, fill_attr, stroke_attr));
                }
                ShapeType::Circle { center, radius } => {
                    out.push_str(&format!("  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" {} {} />\n", 
                        center.x, center.y, radius, fill_attr, stroke_attr));
                }
                ShapeType::Rect { pos, width, height, rx } => {
                    out.push_str(&format!("  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"{:.1}\" {} {} />\n", 
                        pos.x, pos.y, width, height, rx, fill_attr, stroke_attr));
                }
                ShapeType::Line { start, end } => {
                    out.push_str(&format!("  <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" {} />\n", 
                        start.x, start.y, end.x, end.y, stroke_attr));
                }
                ShapeType::Path(d) => {
                    out.push_str(&format!("  <path d=\"{}\" {} {} />\n", d, fill_attr, stroke_attr));
                }
                ShapeType::Text { pos, text, font_size } => {
                    out.push_str(&format!("  <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"'Helvetica Neue', Helvetica, Arial, sans-serif\" font-size=\"{:.1}\" font-weight=\"800\" fill=\"{}\" letter-spacing=\"4\" filter=\"url(#glow)\">{}</text>\n",
                        pos.x, pos.y, font_size, fill_hex, text));
                }
            }
        }

        out
    }

    pub fn save_to_svg(filename: &str, shapes: &[Shape]) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        let content = Self::to_svg_string(shapes);
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn hsl_to_rgb(c: &Color) -> String {
        let h = c.h;
        let s = c.s;
        let l = c.l;

        let c_val = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c_val * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c_val / 2.0;

        let (r1, g1, b1) = if h < 60.0 {
            (c_val, x, 0.0)
        } else if h < 120.0 {
            (x, c_val, 0.0)
        } else if h < 180.0 {
            (0.0, c_val, x)
        } else if h < 240.0 {
            (0.0, x, c_val)
        } else if h < 300.0 {
            (x, 0.0, c_val)
        } else {
            (c_val, 0.0, x)
        };

        let r = ((r1 + m) * 255.0).round() as u8;
        let g = ((g1 + m) * 255.0).round() as u8;
        let b = ((b1 + m) * 255.0).round() as u8;

        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}
