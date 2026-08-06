use super::pbr::PbrMaterial;
use crate::science::crucible::{Gene, TheCrucible};

pub struct MaterialMatcher;

impl MaterialMatcher {
    /// Finds the PBR parameters that best match a target spectral response
    pub fn match_material(target_front_rgb: [f64; 3], target_edge_rgb: [f64; 3]) -> PbrMaterial {
        println!("🔮 Materials Engine: Inverse Rendering to match target reflectance...");

        let genes = vec![
            Gene {
                name: "Albedo_R".to_string(),
                bounds: (0.0, 1.0),
                current_value: 0.5,
            },
            Gene {
                name: "Albedo_G".to_string(),
                bounds: (0.0, 1.0),
                current_value: 0.5,
            },
            Gene {
                name: "Albedo_B".to_string(),
                bounds: (0.0, 1.0),
                current_value: 0.5,
            },
            Gene {
                name: "Roughness".to_string(),
                bounds: (0.0, 1.0),
                current_value: 0.5,
            },
            Gene {
                name: "Metallic".to_string(),
                bounds: (0.0, 1.0),
                current_value: 0.0,
            },
        ];

        let iterations = 10000;
        let (_, best_genes) = TheCrucible::anneal(
            genes,
            |current_genes| {
                let mat = PbrMaterial {
                    albedo: [
                        current_genes[0].current_value,
                        current_genes[1].current_value,
                        current_genes[2].current_value,
                    ],
                    roughness: current_genes[3].current_value,
                    metallic: current_genes[4].current_value,
                };

                // Front view (angle 0)
                let sim_front = mat.simulate_reflectance(0.0);
                // Edge view (angle ~80 degrees, 1.4 rad)
                let sim_edge = mat.simulate_reflectance(1.4);

                let mut cost = 0.0;
                for i in 0..3 {
                    cost += (sim_front[i] - target_front_rgb[i]).powi(2);
                    cost += (sim_edge[i] - target_edge_rgb[i]).powi(2);
                }
                cost
            },
            iterations,
        );

        let best_mat = PbrMaterial {
            albedo: [
                best_genes[0].current_value,
                best_genes[1].current_value,
                best_genes[2].current_value,
            ],
            roughness: best_genes[3].current_value,
            metallic: best_genes[4].current_value,
        };

        println!("✅ Material Match Complete!");
        println!(
            "  -> Albedo: [{:.2}, {:.2}, {:.2}]",
            best_mat.albedo[0], best_mat.albedo[1], best_mat.albedo[2]
        );
        println!("  -> Roughness: {:.2}", best_mat.roughness);
        println!("  -> Metallic: {:.2}", best_mat.metallic);

        best_mat
    }
}
