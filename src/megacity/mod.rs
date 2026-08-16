use crate::architecture::FloorPlanner;
use crate::materials::matcher::MaterialMatcher;
use crate::mechanics::optimizer::MechanicsOptimizer;
use crate::science::multidomain_fuzz::MultiDomainFuzzObjective;
use crate::science::chaos_swarm::ChaosSwarm;
use crate::science::assembly_funnel::FunnelConfig;
use crate::science::resonance_objective::ResonanceObjective;
use crate::science::aerodynamic_objective::AerodynamicObjective;
use crate::science::topology_isolation_objective::TopologyIsolationObjective;
use crate::science::metamaterial_objective::MetamaterialObjective;

pub mod exporter;
pub mod gcode_exporter;

pub struct MegaCityPipeline;

impl MegaCityPipeline {
    pub fn run_pipeline(profile: crate::profile::MegaCityProfile) -> String {
        // Step 1: Architecture Co-evolution
        println!("[MegaPipeline] Phase 1: Co-Evolving Architecture...");
        let rooms = FloorPlanner::optimize_layout(profile.arch.clone());

        // Step 2: Mechanics Truss Generation
        // We use the center of the architecture as the base for the mechanics.
        println!("[MegaPipeline] Phase 2: Generating Truss Mechanics...");
        // Since MegaCityProfile doesn't have PhysicsProfile directly, we use default for physics
        // or we could refactor MegaCityProfile. Let's use default.
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
            hard_limit_gen: 256,
            hard_limit_score: 0,
            use_diffusion: true,
        };
        let (_, best_candidate) = ChaosSwarm::launch_swarm_tunable(
            objective, config.clone(), "MegaCity Stress Simulation", 
            4, 64, 4
        );
        let fuzz_genes = best_candidate.unwrap_or([0.0; 9]);

        let _enstrophy = fuzz_genes[0];
        let _freq = fuzz_genes[2];
        let _power = fuzz_genes[8];

        // --- Phase 6: Vibration Damping Truss ---
        println!("[MegaPipeline] Phase 6: Optimizing vibration damping truss spacing...");
        let resonance_objective = ResonanceObjective;
        let resonance_swarm = ChaosSwarm::launch_swarm_tunable(
            resonance_objective, config.clone(), "Truss Damping Spacing", 4, 64, 4
        );
        let mut best_spacings = resonance_swarm.1.unwrap_or([1.0; 10]);
        best_spacings.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // --- Phase 7: Aerodynamic Boundary Deflectors ---
        println!("[MegaPipeline] Phase 7: Optimizing aerodynamic wind deflectors...");
        let aero_objective = AerodynamicObjective;
        let aero_swarm = ChaosSwarm::launch_swarm_tunable(
            aero_objective, config.clone(), "Aerodynamic Profiles", 4, 64, 4
        );
        let _best_aero = aero_swarm.1.unwrap_or([0.0; 8]);

        // --- Phase 8: Joint Load Isolation ---
        println!("[MegaPipeline] Phase 8: Optimizing joint load isolation topology...");
        let isolation_objective = TopologyIsolationObjective;
        let isolation_swarm = ChaosSwarm::launch_swarm_tunable(
            isolation_objective, config.clone(), "Joint Load Isolation", 4, 64, 4
        );
        let _best_topology = isolation_swarm.1.unwrap_or([0.0; 15]);

        // --- Phase 9: Acoustic Void Micro-Topology ---
        println!("[MegaPipeline] Phase 9: Optimizing internal acoustic/shock voids...");
        let void_objective = MetamaterialObjective;
        let void_swarm = ChaosSwarm::launch_swarm_tunable(
            void_objective, config.clone(), "Acoustic Micro-Topology", 4, 128, 4
        );
        let _best_voids = void_swarm.1.unwrap_or([0.0; 32]);

        // --- Phase 10: 3D Scene & Fabrication Export ---
        println!("[MegaPipeline] Phase 10: Exporting Blender 3D Model & CNC/3D Print G-Code...");
        exporter::BlenderExporter::export_megacity(&rooms, &truss, &mat);
        gcode_exporter::GCodeExporter::export_megacity(&rooms, &truss);

        println!("[MegaPipeline] All 10 Phases Completed.");

        r#"{"status": "success", "message": "MegaCity Blueprint generated. G-Code exported."}"#.to_string()
    }
}
