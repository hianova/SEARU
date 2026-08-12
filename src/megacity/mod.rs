use crate::architecture::FloorPlanner;
use crate::materials::matcher::MaterialMatcher;
use crate::mechanics::optimizer::MechanicsOptimizer;
use crate::visual::composer::VisualComposer;

pub struct MegaCityPipeline;

impl MegaCityPipeline {
    pub fn run_pipeline() -> String {
        // Step 1: Architecture Co-evolution
        println!("[MegaPipeline] Phase 1: Co-Evolving Architecture...");
        let rooms = FloorPlanner::optimize_layout();

        // Step 2: Mechanics Truss Generation
        // We use the center of the architecture as the base for the mechanics.
        println!("[MegaPipeline] Phase 2: Generating Truss Mechanics...");
        let truss = MechanicsOptimizer::optimize_truss();

        // Step 3: Material Matching
        // E.g., matching a heavy duty material based on truss mass.
        println!("[MegaPipeline] Phase 3: Matching PBR Materials...");
        let mat = MaterialMatcher::match_material([0.2, 0.2, 0.2], [0.8, 0.8, 0.8]);

        // Step 4: Visual Generation
        println!("[MegaPipeline] Phase 4: Rendering Visuals...");
        let visuals = VisualComposer::generate_art(20, "MegaCity");

        // Combine all into a massive SVG
        let mut svg = String::new();
        svg.push_str("<svg width=\"800\" height=\"800\" viewBox=\"0 0 800 800\" xmlns=\"http://www.w3.org/2000/svg\">\n");
        svg.push_str("  <rect width=\"100%\" height=\"100%\" fill=\"#0f172a\" />\n");

        // Render Visual Art Background
        let svg_elements = crate::visual::exporter::SvgExporter::to_svg_elements(&visuals);
        svg.push_str(&format!("<g opacity=\"0.15\" transform=\"scale(1.5)\">\n{}\n</g>\n", svg_elements));

        // Render Truss Mechanics
        for bar in truss.bars {
            let n1 = &truss.nodes[bar.node_a];
            let n2 = &truss.nodes[bar.node_b];
            // map truss coords (which are -5.0 to 5.0) to SVG (0 to 800)
            let x1 = (n1.x + 5.0) * 80.0;
            let y1 = (n1.y + 5.0) * 80.0;
            let x2 = (n2.x + 5.0) * 80.0;
            let y2 = (n2.y + 5.0) * 80.0;
            svg.push_str(&format!(
                "  <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#94a3b8\" stroke-width=\"2\" opacity=\"0.5\" />\n",
                x1, y1, x2, y2
            ));
        }

        // Render Architecture Rooms
        for r in rooms {
            let hex_color = format!(
                "#{:02x}{:02x}{:02x}",
                (mat.albedo[0] * 255.0) as u8,
                (mat.albedo[1] * 255.0) as u8,
                (mat.albedo[2] * 255.0) as u8
            );
            // shift architecture to center
            let shift_x = r.x + 150.0;
            let shift_y = r.y + 150.0;
            svg.push_str(&format!(
                "  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\" stroke=\"#fff\" stroke-width=\"3\" opacity=\"0.85\" />\n",
                shift_x, shift_y, r.w, r.h, hex_color
            ));
            svg.push_str(&format!(
                "  <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"monospace\" font-size=\"14\" font-weight=\"bold\" fill=\"#fff\">{}</text>\n",
                shift_x + 10.0, shift_y + 20.0, r.name
            ));
        }

        svg.push_str("</svg>\n");
        svg
    }
}
