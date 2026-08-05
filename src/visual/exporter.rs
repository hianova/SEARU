use super::geometry::{Shape, Color};
use std::fs::File;
use std::io::Write;

pub struct SvgExporter;

impl SvgExporter {
    pub fn to_svg_string(shapes: &[Shape]) -> String {
        let mut out = String::new();
        out.push_str("<svg width=\"800\" height=\"600\" xmlns=\"http://www.w3.org/2000/svg\">\n");
        out.push_str("  <rect width=\"100%\" height=\"100%\" fill=\"#1a1a1a\" />\n");
        
        for shape in shapes {
            let mut path = String::new();
            for (i, p) in shape.points.iter().enumerate() {
                if i == 0 {
                    path.push_str(&format!("M {:.1} {:.1} ", p.x, p.y));
                } else {
                    path.push_str(&format!("L {:.1} {:.1} ", p.x, p.y));
                }
            }
            path.push_str("Z");
            
            out.push_str(&format!(
                "  <path d=\"{}\" fill=\"{}\" opacity=\"0.7\" />\n",
                path,
                Self::hsl_to_rgb(&shape.color)
            ));
        }
        
        out.push_str("</svg>\n");
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
