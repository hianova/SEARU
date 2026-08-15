use crate::architecture::FloorPlanner;
use crate::materials::matcher::MaterialMatcher;
use crate::mechanics::optimizer::MechanicsOptimizer;
use crate::visual::composer::VisualComposer;
use crate::science::multidomain_fuzz::MultiDomainFuzzObjective;
use crate::science::chaos_swarm::ChaosSwarm;
use crate::science::assembly_funnel::FunnelConfig;

pub mod exporter;

pub struct MegaCityPipeline;

impl MegaCityPipeline {
    pub fn run_pipeline(profile: crate::profile::MegaCityProfile) -> String {
        // Step 1: Architecture Co-evolution
        println!("[MegaPipeline] Phase 1: Co-Evolving Architecture...");
        let rooms = FloorPlanner::optimize_layout(profile.arch);

        // Step 2: Mechanics Truss Generation
        // We use the center of the architecture as the base for the mechanics.
        println!("[MegaPipeline] Phase 2: Generating Truss Mechanics...");
        let truss = MechanicsOptimizer::optimize_truss();

        // Step 3: Material Matching
        println!("[MegaPipeline] Phase 3: Matching PBR Materials...");
        let mat = MaterialMatcher::match_material([
            profile.mechanics.target_r,
            profile.mechanics.target_g,
            profile.mechanics.target_b,
        ]);

        // Step 4: Visual Generation
        println!("[MegaPipeline] Phase 4: Rendering Visuals...");
        let visuals = VisualComposer::generate_art(20, "MegaCity", &[], &profile.visual);

        // --- NEW: BLENDER EXPORT ---
        exporter::BlenderExporter::export_megacity(&rooms, &truss, &mat);

        // --- Phase 5: Multi-Domain Fuzzing (Physics Erosion) ---
        println!("[MegaPipeline] Phase 5: Fuzzing Physics Erosion...");
        let env_rooms = rooms.len() as f64;
        let env_bars = truss.bars.len() as f64;
        let env_metallic = mat.metallic;
        
        let objective = MultiDomainFuzzObjective::new(env_rooms, env_bars, env_metallic);
        let config = FunnelConfig {
            tier1_population: 500,
            tier2_retention_ratio: 0.1,
            tier3_dfs_depth: 1,
            stagnation_patience: 10,
            stagnation_delta: 0.5,
            rng_seed: 42,
            min_slope_window: 0,
            min_slope_threshold: 0.0,
            hard_limit_gen: 256, // Not used heavily since swarm limits per epoch
            hard_limit_score: 0,
            use_diffusion: true,
        };
        let (_, best_candidate) = ChaosSwarm::launch_swarm_tunable(
            objective, config, "MegaCity Erosion", 
            4, 64, 4 // 4 islands, 64 gens per epoch, 4 epochs = 256 gens total per island
        );
        let fuzz_genes = best_candidate.unwrap_or([0.0; 9]);

        let enstrophy = fuzz_genes[0]; // Turbulence
        let freq = fuzz_genes[6];      // Vibration
        let power = fuzz_genes[8];     // Glow

        // Combine all into a massive SVG
        let mut svg = String::new();
        svg.push_str("<svg width=\"800\" height=\"800\" viewBox=\"0 0 800 800\" xmlns=\"http://www.w3.org/2000/svg\">\n");
        svg.push_str("  <defs>\n");
        // Add Glow filter based on Silicon Power
        svg.push_str(&format!("    <filter id=\"glow\" x=\"-20%\" y=\"-20%\" width=\"140%\" height=\"140%\">\n      <feGaussianBlur stdDeviation=\"{:.1}\" result=\"blur\" />\n      <feComposite in=\"SourceGraphic\" in2=\"blur\" operator=\"over\" />\n    </filter>\n", power * 0.5));
        svg.push_str("  </defs>\n");
        svg.push_str("  <rect width=\"100%\" height=\"100%\" fill=\"#0f172a\" />\n");

        // Render Visual Art Background
        let svg_elements = crate::visual::exporter::SvgExporter::to_svg_elements(&visuals);
        svg.push_str(&format!(
            "<g opacity=\"0.15\" transform=\"scale(1.5)\">\n{}\n</g>\n",
            svg_elements
        ));

        // Render Truss Mechanics with Silicon Power Glow
        let stroke_color = if power > 50.0 { "#38bdf8" } else { "#94a3b8" };
        let filter_str = if power > 30.0 { "filter=\"url(#glow)\"" } else { "" };
        for bar in truss.bars {
            let n1 = &truss.nodes[bar.node_a];
            let n2 = &truss.nodes[bar.node_b];
            let x1 = (n1.x + 5.0) * 80.0;
            let y1 = (n1.y + 5.0) * 80.0;
            let x2 = (n2.x + 5.0) * 80.0;
            let y2 = (n2.y + 5.0) * 80.0;
            svg.push_str(&format!(
                "  <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"3\" opacity=\"0.6\" {} />\n",
                x1, y1, x2, y2, stroke_color, filter_str
            ));
        }

        // Render Architecture Rooms with Vibration Offset
        let vibe_offset = freq * 0.5; // Up to 10px offset
        for r in rooms {
            let hex_color = format!(
                "#{:02x}{:02x}{:02x}",
                (mat.albedo[0] * 255.0) as u8,
                (mat.albedo[1] * 255.0) as u8,
                (mat.albedo[2] * 255.0) as u8
            );
            let shift_x = r.x + 150.0;
            let shift_y = r.y + 150.0;
            
            // Render vibration ghost
            if vibe_offset > 2.0 {
                svg.push_str(&format!(
                    "  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"none\" stroke=\"#ff0055\" stroke-width=\"1\" opacity=\"0.5\" transform=\"translate({:.1}, -{:.1})\" />\n",
                    shift_x, shift_y, r.w, r.h, vibe_offset, vibe_offset
                ));
            }
            
            svg.push_str(&format!(
                "  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\" stroke=\"#fff\" stroke-width=\"3\" opacity=\"0.85\" />\n",
                shift_x, shift_y, r.w, r.h, hex_color
            ));
            svg.push_str(&format!(
                "  <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"monospace\" font-size=\"14\" font-weight=\"bold\" fill=\"#fff\">{}</text>\n",
                shift_x + 10.0, shift_y + 20.0, r.name
            ));
        }

        // Render Fluid Enstrophy (Turbulence Scars)
        if enstrophy > 3.0 {
            let num_lines = (enstrophy * 2.0) as usize;
            svg.push_str("  <g stroke=\"#38bdf8\" stroke-width=\"1\" fill=\"none\" opacity=\"0.3\">\n");
            for i in 0..num_lines {
                let sx = (i * 45) % 800;
                let sy = (i * 67) % 800;
                let cx = sx + 100 + (enstrophy as usize * 10);
                let cy = sy - 100;
                let ex = sx + 200;
                let ey = sy + 50;
                svg.push_str(&format!("    <path d=\"M{} {} Q {} {} {} {}\" />\n", sx, sy, cx, cy, ex, ey));
            }
            svg.push_str("  </g>\n");
        }

        svg.push_str("</svg>\n");
        svg
    }
}
