use crate::architecture::FloorPlanner;
use crate::pcb_routing::PcbRouter;
use crate::ui_layout::UiOptimizer;
use crate::visual::composer::VisualComposer;

pub struct FractalEngine;

impl FractalEngine {
    pub fn generate_universe() -> String {
        println!("[FractalEngine] Igniting Universal Seed... Depth 0");
        let mut svg = String::new();
        svg.push_str("<svg id=\"fractal-svg\" width=\"100%\" height=\"100%\" viewBox=\"0 0 1000 1000\" xmlns=\"http://www.w3.org/2000/svg\">\n");
        svg.push_str("  <rect width=\"100%\" height=\"100%\" fill=\"#020617\" />\n");

        let content = Self::generate_recursive(0, 3, 0.0, 0.0, 1000.0, 1000.0);
        svg.push_str(&content);

        svg.push_str("</svg>\n");
        svg
    }

    fn generate_recursive(
        depth: usize,
        max_depth: usize,
        offset_x: f64,
        offset_y: f64,
        scale_w: f64,
        scale_h: f64,
    ) -> String {
        if depth > max_depth {
            return String::new();
        }

        let mut out = String::new();
        out.push_str(&format!("<g id=\"depth-{}\">\n", depth));

        match depth {
            0 => {
                // Depth 0: Architecture (0 to 500 mapped to scale_w, scale_h)
                let rooms = FloorPlanner::optimize_layout(crate::profile::ArchProfile::default());
                for r in rooms {
                    let rx = offset_x + (r.x / 500.0) * scale_w;
                    let ry = offset_y + (r.y / 500.0) * scale_h;
                    let rw = (r.w / 500.0) * scale_w;
                    let rh = (r.h / 500.0) * scale_h;

                    out.push_str(&format!(
                        "  <rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"#0f172a\" stroke=\"#38bdf8\" stroke-width=\"{:.2}\" opacity=\"0.9\" />\n",
                        rx, ry, rw, rh, scale_w * 0.005
                    ));

                    let children = Self::generate_recursive(depth + 1, max_depth, rx, ry, rw, rh);
                    out.push_str(&children);
                }
            }
            1 => {
                // Depth 1: UI Layout (Fills the room perfectly, handles aspect ratio natively)
                let nodes = UiOptimizer::optimize();
                let total_flex: f64 = nodes.iter().map(|n| n.flex_grow).sum();
                let mut current_y = offset_y;

                for n in nodes {
                    let height = (n.flex_grow / total_flex.max(1.0)) * scale_h;
                    let margin = (n.margin / 50.0) * (scale_w.min(scale_h) * 0.05); // safe uniform margin

                    let nx = offset_x + margin;
                    let ny = current_y + margin;
                    let nw = scale_w - margin * 2.0;
                    let nh = height - margin * 2.0;

                    if nw > 0.0 && nh > 0.0 {
                        out.push_str(&format!(
                            "  <rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"#1e293b\" stroke=\"#10b981\" stroke-width=\"{:.2}\" rx=\"{:.2}\" />\n",
                            nx, ny, nw, nh, scale_w.min(scale_h) * 0.02, scale_w.min(scale_h) * 0.05
                        ));

                        let children =
                            Self::generate_recursive(depth + 1, max_depth, nx, ny, nw, nh);
                        out.push_str(&children);
                    }

                    current_y += height;
                }
            }
            2 => {
                // Depth 2: PCB Routing (Centered Square inside the UI flex node)
                // We force it into a square so the traces are never squashed
                let pcb_size = scale_w.min(scale_h) * 0.8;
                let cx = offset_x + scale_w / 2.0;
                let cy = offset_y + scale_h / 2.0;
                let pcb_x = cx - pcb_size / 2.0;
                let pcb_y = cy - pcb_size / 2.0;

                out.push_str(&format!(
                    "  <rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"#022c22\" stroke=\"#10b981\" stroke-width=\"{:.2}\" />\n", 
                    pcb_x, pcb_y, pcb_size, pcb_size, pcb_size * 0.02
                ));

                let traces = PcbRouter::route();
                for t in traces {
                    let mut path = String::new();
                    for (j, p) in t.points.iter().enumerate() {
                        let px = pcb_x + (p.0 / 400.0) * pcb_size;
                        let py = pcb_y + (p.1 / 400.0) * pcb_size;
                        if j == 0 {
                            path.push_str(&format!("M {:.2} {:.2} ", px, py));
                        } else {
                            path.push_str(&format!("L {:.2} {:.2} ", px, py));
                        }
                    }
                    out.push_str(&format!(
                        "  <path d=\"{}\" fill=\"none\" stroke=\"#facc15\" stroke-width=\"{:.2}\" stroke-linejoin=\"round\" />\n",
                        path, pcb_size * 0.03
                    ));

                    // Massive Pads so the Visual Art inside is actually visible!
                    let pad_radius = pcb_size * 0.15;
                    for pad_pt in [t.points.first().unwrap(), t.points.last().unwrap()] {
                        let px = pcb_x + (pad_pt.0 / 400.0) * pcb_size;
                        let py = pcb_y + (pad_pt.1 / 400.0) * pcb_size;
                        out.push_str(&format!(
                            "  <circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\" fill=\"#334155\" stroke=\"#94a3b8\" stroke-width=\"{:.2}\" />\n", 
                            px, py, pad_radius, pcb_size * 0.01
                        ));

                        let children = Self::generate_recursive(
                            depth + 1,
                            max_depth,
                            px - pad_radius,
                            py - pad_radius,
                            pad_radius * 2.0,
                            pad_radius * 2.0,
                        );
                        out.push_str(&children);
                    }
                }
            }
            3 => {
                // Depth 3: Visual Art (Mapped perfectly inside the circular Pad)
                let visuals = VisualComposer::generate_art(
                    depth + offset_x as usize,
                    "Fractal",
                    &[],
                    &crate::profile::VisualProfile::default(),
                );
                let svg_elements = crate::visual::exporter::SvgExporter::to_svg_elements(&visuals);
                out.push_str(&format!(
                    "  <svg x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" viewBox=\"0 0 800 800\">\n{}\n  </svg>\n",
                    offset_x, offset_y, scale_w, scale_h, svg_elements
                ));
            }
            _ => {}
        }

        out.push_str("</g>\n");
        out
    }
}
