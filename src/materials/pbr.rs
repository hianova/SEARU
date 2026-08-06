#[derive(Clone, Debug)]
pub struct PbrMaterial {
    pub albedo: [f64; 3], // R, G, B (0-1)
    pub roughness: f64,   // 0-1
    pub metallic: f64,    // 0-1
}

impl PbrMaterial {
    /// Simulates a very simplified BRDF response at a specific view angle (0 to PI/2).
    /// Returns the RGB spectral response.
    pub fn simulate_reflectance(&self, view_angle: f64) -> [f64; 3] {
        let n_dot_v = view_angle.cos().max(0.0);

        // Fresnel approximation (Schlick)
        let f0 = if self.metallic > 0.5 {
            self.albedo
        } else {
            [0.04, 0.04, 0.04]
        };

        let mut response = [0.0; 3];
        for i in 0..3 {
            let fresnel = f0[i] + (1.0 - f0[i]) * (1.0 - n_dot_v).powi(5);

            // Diffuse component
            let diffuse = self.albedo[i] * (1.0 - self.metallic) / std::f64::consts::PI;

            // Specular component (highly simplified microfacet)
            let specular = fresnel * (1.0 - self.roughness) / (4.0 * n_dot_v + 0.01);

            response[i] = (diffuse + specular).clamp(0.0, 1.0);
        }

        response
    }
}
