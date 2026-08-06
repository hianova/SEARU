use super::statics::Truss;
use std::fs::File;
use std::io::Write;

pub struct TrussExporter;

impl TrussExporter {
    pub fn to_svg_string(truss: &Truss) -> String {
        let mut out = String::new();
        out.push_str("<svg width=\"800\" height=\"600\" xmlns=\"http://www.w3.org/2000/svg\">\n");
        out.push_str("  <rect width=\"100%\" height=\"100%\" fill=\"#f0f0f0\" />\n");

        // Translate to center
        let dx = 250.0;
        let dy = 250.0;
        let scale = 3.0;

        for bar in &truss.bars {
            let n1 = &truss.nodes[bar.node_a];
            let n2 = &truss.nodes[bar.node_b];
            let thickness = bar.area;
            let color = if bar.stress > 80.0 {
                "#e74c3c"
            } else {
                "#3498db"
            };

            out.push_str(&format!(
                "  <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"{:.1}\" opacity=\"0.8\" />\n",
                n1.x * scale + dx, n1.y * scale + dy,
                n2.x * scale + dx, n2.y * scale + dy,
                color, thickness
            ));
        }

        for node in &truss.nodes {
            let color = if node.fixed {
                "#2c3e50"
            } else if node.force_y > 0.0 {
                "#e67e22"
            } else {
                "#95a5a6"
            };
            let radius = if node.fixed { 8.0 } else { 5.0 };

            out.push_str(&format!(
                "  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"{}\" />\n",
                node.x * scale + dx,
                node.y * scale + dy,
                radius,
                color
            ));
        }

        out.push_str("</svg>\n");
        out
    }

    pub fn save_to_svg(filename: &str, truss: &Truss) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        let content = Self::to_svg_string(truss);
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
