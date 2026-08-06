use crate::science::crucible::{Gene, TheCrucible};

#[derive(Clone, Debug)]
pub struct Glyph {
    pub p0: (f64, f64),
    pub p1: (f64, f64),
    pub p2: (f64, f64),
    pub p3: (f64, f64),
}

pub struct TypographyGenerator;

impl TypographyGenerator {
    pub fn generate_glyph() -> Glyph {
        let mut genes = Vec::new();
        // p0 is fixed, p3 is fixed. Optimize p1 and p2 for a smooth curve.
        genes.push(Gene {
            name: "P1_X".to_string(),
            bounds: (0.0, 100.0),
            current_value: 50.0,
        });
        genes.push(Gene {
            name: "P1_Y".to_string(),
            bounds: (0.0, 100.0),
            current_value: 50.0,
        });
        genes.push(Gene {
            name: "P2_X".to_string(),
            bounds: (0.0, 100.0),
            current_value: 50.0,
        });
        genes.push(Gene {
            name: "P2_Y".to_string(),
            bounds: (0.0, 100.0),
            current_value: 50.0,
        });

        let p0 = (10.0, 90.0);
        let p3 = (90.0, 10.0);

        let (_, best_genes) = TheCrucible::anneal(
            genes,
            |g| {
                let p1 = (g[0].current_value, g[1].current_value);
                let p2 = (g[2].current_value, g[3].current_value);

                let mut cost = 0.0;
                // Target a specific curve length (e.g. 150)
                let d1 = ((p1.0 - p0.0).powi(2) + (p1.1 - p0.1).powi(2)).sqrt();
                let d2 = ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt();
                let d3 = ((p3.0 - p2.0).powi(2) + (p3.1 - p2.1).powi(2)).sqrt();
                let len = d1 + d2 + d3;
                cost += (len - 150.0).abs();

                cost
            },
            5000,
        );

        Glyph {
            p0,
            p1: (best_genes[0].current_value, best_genes[1].current_value),
            p2: (best_genes[2].current_value, best_genes[3].current_value),
            p3,
        }
    }

    pub fn to_svg_string(glyph: &Glyph) -> String {
        format!(
            "<svg width=\"100\" height=\"100\" xmlns=\"http://www.w3.org/2000/svg\">\n  <rect width=\"100%\" height=\"100%\" fill=\"#fff\" />\n  <path d=\"M {:.1} {:.1} C {:.1} {:.1}, {:.1} {:.1}, {:.1} {:.1}\" fill=\"none\" stroke=\"#000\" stroke-width=\"4\" />\n</svg>\n",
            glyph.p0.0,
            glyph.p0.1,
            glyph.p1.0,
            glyph.p1.1,
            glyph.p2.0,
            glyph.p2.1,
            glyph.p3.0,
            glyph.p3.1
        )
    }
}
