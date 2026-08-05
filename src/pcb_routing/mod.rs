use crate::science::crucible::{Gene, TheCrucible};

#[derive(Clone, Debug)]
pub struct Trace {
    pub points: Vec<(f64, f64)>,
}

pub struct PcbRouter;

impl PcbRouter {
    pub fn route() -> Vec<Trace> {
        let nets = vec![
            ((50.0, 50.0), (350.0, 350.0)),
            ((50.0, 350.0), (350.0, 50.0)), // Crossing nets
        ];
        
        let mut genes = Vec::new();
        for (i, _) in nets.iter().enumerate() {
            genes.push(Gene { name: format!("W{}_X", i), bounds: (0.0, 400.0), current_value: 200.0 });
            genes.push(Gene { name: format!("W{}_Y", i), bounds: (0.0, 400.0), current_value: 200.0 });
        }
        
        let (_, best_genes) = TheCrucible::anneal(
            genes,
            |g| {
                let p1 = (g[0].current_value, g[1].current_value);
                let p2 = (g[2].current_value, g[3].current_value);
                
                let mut cost = 0.0;
                
                // Manhattan distance
                cost += (nets[0].0.0 - p1.0).abs() + (nets[0].0.1 - p1.1).abs();
                cost += (p1.0 - nets[0].1.0).abs() + (p1.1 - nets[0].1.1).abs();
                
                cost += (nets[1].0.0 - p2.0).abs() + (nets[1].0.1 - p2.1).abs();
                cost += (p2.0 - nets[1].1.0).abs() + (p2.1 - nets[1].1.1).abs();
                
                // Penalty for being too close (avoid shorts)
                let dist_between_waypoints = ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt();
                if dist_between_waypoints < 100.0 {
                    cost += 10000.0 / (dist_between_waypoints + 1.0); 
                }
                
                cost
            },
            10000
        );
        
        vec![
            Trace { points: vec![nets[0].0, (best_genes[0].current_value, best_genes[1].current_value), nets[0].1] },
            Trace { points: vec![nets[1].0, (best_genes[2].current_value, best_genes[3].current_value), nets[1].1] },
        ]
    }

    pub fn to_svg_string(traces: &[Trace]) -> String {
        let mut out = String::new();
        out.push_str("<svg width=\"400\" height=\"400\" xmlns=\"http://www.w3.org/2000/svg\">\n");
        out.push_str("  <rect width=\"100%\" height=\"100%\" fill=\"#022c22\" />\n"); 
        
        let colors = ["#facc15", "#38bdf8"];
        for (i, t) in traces.iter().enumerate() {
            let mut path = String::new();
            for (j, p) in t.points.iter().enumerate() {
                if j == 0 { path.push_str(&format!("M {:.1} {:.1} ", p.0, p.1)); }
                else { path.push_str(&format!("L {:.1} {:.1} ", p.0, p.1)); }
            }
            out.push_str(&format!(
                "  <path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"8\" stroke-linejoin=\"round\" />\n",
                path, colors[i % colors.len()]
            ));
            
            // Draw pads
            out.push_str(&format!("  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"10\" fill=\"#94a3b8\" />\n", t.points.first().unwrap().0, t.points.first().unwrap().1));
            out.push_str(&format!("  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"10\" fill=\"#94a3b8\" />\n", t.points.last().unwrap().0, t.points.last().unwrap().1));
        }
        
        out.push_str("</svg>\n");
        out
    }
}
