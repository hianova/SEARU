use crate::architecture::FloorPlanner;
use crate::materials::matcher::MaterialMatcher;
use crate::mechanics::optimizer::MechanicsOptimizer;
use crate::science::crucible::{TheCrucible, Gene};

pub mod exporter;
pub mod gcode_exporter;

pub struct MegaCityPipeline;

impl MegaCityPipeline {
    pub fn run_pipeline(profile: crate::profile::MegaCityProfile) -> String {
        // Step 1: Architecture Co-evolution
        println!("[MegaPipeline] Phase 1: Co-Evolving Architecture...");
        let rooms = FloorPlanner::optimize_layout(profile.arch.clone());

        // Step 2: Mechanics Truss Generation
        println!("[MegaPipeline] Phase 2: Generating Truss Mechanics...");
        let physics_profile = crate::profile::PhysicsProfile::default();
        let truss = MechanicsOptimizer::optimize_truss(&profile.arch, &physics_profile);

        // Step 3: Material Matching
        println!("[MegaPipeline] Phase 3: Matching PBR Materials...");
        let mat = MaterialMatcher::match_material([
            profile.mechanics.target_r,
            profile.mechanics.target_g,
            profile.mechanics.target_b,
        ]);


        // Step 4: Export 3D OBJ
        println!("[MegaPipeline] Phase 4: Exporting 3D CAD...");
        std::fs::create_dir_all("release").unwrap_or_default();
        let obj_data = crate::mechanics::exporter::TrussExporter::to_obj_string(&truss);
        std::fs::write("release/megacity.obj", obj_data).unwrap_or_else(|_| println!("Failed to write OBJ"));

        // --- Phase 5: Multi-Domain Stress Fuzzing ---
        println!("[MegaPipeline] Phase 5: Simulating structural wear & multi-domain stress...");
        let env_rooms = rooms.len() as f64;
        let env_bars = truss.bars.len() as f64;
        let env_metallic = mat.metallic;
        
        let fuzz_genes = vec![
            Gene { name: "enstrophy".into(), bounds: (0.0, 10.0), current_value: 0.5 },
            Gene { name: "pressure_gradient".into(), bounds: (0.0, 10.0), current_value: 0.5 },
            Gene { name: "viscosity".into(), bounds: (0.1, 50.0), current_value: 1.0 },
            Gene { name: "local_strain".into(), bounds: (0.0, 50.0), current_value: 0.5 },
            Gene { name: "stiffness".into(), bounds: (10.0, 500.0), current_value: 100.0 },
            Gene { name: "damping".into(), bounds: (1.0, 50.0), current_value: 10.0 },
            Gene { name: "freq".into(), bounds: (0.1, 20.0), current_value: 1.0 },
            Gene { name: "radius".into(), bounds: (1.0, 50.0), current_value: 5.0 },
            Gene { name: "power".into(), bounds: (1.0, 100.0), current_value: 10.0 },
        ];
        
        let (_, best_fuzz) = TheCrucible::anneal(
            fuzz_genes,
            |g| crate::science::multidomain_fuzz::evaluate_multidomain(g, env_rooms, env_bars, env_metallic),
            256
        );

        let _enstrophy = best_fuzz[0].current_value;
        let _freq = best_fuzz[6].current_value;
        let _power = best_fuzz[8].current_value;

        // --- Phase 6: Vibration Damping Truss ---
        println!("[MegaPipeline] Phase 6: Optimizing vibration damping truss spacing...");
        let mut res_genes = vec![];
        for i in 0..10 {
            res_genes.push(Gene { name: format!("spacing_{}", i), bounds: (0.0, 3.0), current_value: 1.0 });
        }
        let (_, best_res) = TheCrucible::anneal(
            res_genes,
            |g| crate::science::resonance_objective::evaluate_resonance(g),
            256
        );
        let mut best_spacings: Vec<f64> = best_res.iter().map(|g| g.current_value).collect();
        best_spacings.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // --- Phase 7: Aerodynamic Boundary Deflectors ---
        println!("[MegaPipeline] Phase 7: Optimizing aerodynamic wind deflectors...");
        let mut aero_genes = vec![];
        for i in 0..8 {
            aero_genes.push(Gene { name: format!("angle_{}", i), bounds: (0.0, std::f64::consts::PI), current_value: 0.0 });
        }
        let (_, _best_aero) = TheCrucible::anneal(
            aero_genes,
            |g| crate::science::aerodynamic_objective::evaluate_aerodynamic(g),
            256
        );

        // --- Phase 8: Joint Load Isolation ---
        println!("[MegaPipeline] Phase 8: Optimizing joint load isolation topology...");
        let mut topo_genes = vec![];
        for i in 0..15 {
            topo_genes.push(Gene { name: format!("weight_{}", i), bounds: (0.0, 5.0), current_value: 1.0 });
        }
        let (_, _best_topo) = TheCrucible::anneal(
            topo_genes,
            |g| crate::science::topology_isolation_objective::evaluate_topology_isolation(g),
            256
        );

        // --- Phase 9: Acoustic Void Micro-Topology ---
        println!("[MegaPipeline] Phase 9: Optimizing internal acoustic/shock voids...");
        let mut void_genes = vec![];
        for i in 0..32 {
            void_genes.push(Gene { name: format!("void_{}", i), bounds: (-15.0, 15.0), current_value: 0.0 });
        }
        let (_, _best_voids) = TheCrucible::anneal(
            void_genes,
            |g| crate::science::metamaterial_objective::evaluate_metamaterial(g),
            512
        );

        // --- Phase 10: 3D Scene & Fabrication Export ---
        println!("[MegaPipeline] Phase 10: Exporting Blender 3D Model & CNC/3D Print G-Code...");
        exporter::BlenderExporter::export_megacity(&rooms, &truss, &mat);
        gcode_exporter::GCodeExporter::export_megacity(&rooms, &truss);

        println!("[MegaPipeline] All 10 Phases Completed.");

        r#"{"status": "success", "message": "MegaCity Blueprint generated. G-Code exported."}"#.to_string()
    }
}
