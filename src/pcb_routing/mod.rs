use std::collections::{BinaryHeap, HashSet, HashMap};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Trace {
    pub points: Vec<(f64, f64, i32)>, // x, y, layer
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Pos(i32, i32, i32);

#[derive(Copy, Clone, PartialEq, Eq)]
struct State {
    cost: usize,
    position: Pos,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.0.cmp(&other.position.0))
            .then_with(|| self.position.1.cmp(&other.position.1))
            .then_with(|| self.position.2.cmp(&other.position.2))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star(start: Pos, goal_x: i32, goal_y: i32, obstacles: &HashSet<Pos>) -> Option<Vec<Pos>> {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut came_from = HashMap::new();

    dist.insert(start, 0);
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position.0 == goal_x && position.1 == goal_y {
            let mut path = vec![position];
            let mut current = position;
            while let Some(&prev) = came_from.get(&current) {
                path.push(prev);
                current = prev;
                if current == start { break; }
            }
            path.reverse();
            return Some(path);
        }

        if cost > *dist.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }

        let dirs = [
            (0, 1, 0, 1), (1, 0, 0, 1), (0, -1, 0, 1), (-1, 0, 0, 1), // 2D movement
            (0, 0, 1, 5), (0, 0, -1, 5) // Layer change (via) costs 5
        ];

        for &(dx, dy, dz, move_cost) in &dirs {
            let next = Pos(position.0 + dx, position.1 + dy, position.2 + dz);
            
            if next.0 < 0 || next.0 >= 40 || next.1 < 0 || next.1 >= 40 || next.2 < 0 || next.2 > 1 {
                continue;
            }
            
            if obstacles.contains(&next) && (next.0 != goal_x || next.1 != goal_y) {
                continue;
            }

            let next_cost = cost + move_cost;
            if next_cost < *dist.get(&next).unwrap_or(&usize::MAX) {
                came_from.insert(next, position);
                dist.insert(next, next_cost);
                let heuristic = (next.0 - goal_x).abs() + (next.1 - goal_y).abs() + (next.2 * 5).abs();
                heap.push(State { cost: next_cost + heuristic as usize, position: next });
            }
        }
    }
    None
}

pub struct PcbRouter;

impl PcbRouter {
    pub fn route() -> Vec<Trace> {
        println!("🔌 PCB Engine: Routing traces via A* Pathfinding...");
        let nets = vec![
            (Pos(5, 5, 0), 35, 35),
            (Pos(5, 35, 0), 35, 5),
            (Pos(20, 5, 0), 20, 35),
        ];

        let mut obstacles = HashSet::new();
        let mut traces = Vec::new();

        for (start, gx, gy) in nets {
            if let Some(path) = a_star(start, gx, gy, &obstacles) {
                for p in &path {
                    obstacles.insert(*p);
                }
                let points = path.into_iter().map(|p| (p.0 as f64 * 10.0, p.1 as f64 * 10.0, p.2)).collect();
                traces.push(Trace { points });
            }
        }
        
        println!("✅ PCB Routing Complete!");
        traces
    }

    pub fn to_svg_string(traces: &[Trace]) -> String {
        let mut out = String::new();
        out.push_str("<svg width=\"400\" height=\"400\" xmlns=\"http://www.w3.org/2000/svg\">\n");
        out.push_str("  <rect width=\"100%\" height=\"100%\" fill=\"#022c22\" />\n");

        let colors = ["#facc15", "#38bdf8", "#fb7185"];
        for (i, t) in traces.iter().enumerate() {
            let mut path = String::new();
            let mut vias = String::new();
            let mut prev_layer = t.points.first().unwrap().2;
            
            for (j, p) in t.points.iter().enumerate() {
                if j == 0 {
                    path.push_str(&format!("M {:.1} {:.1} ", p.0, p.1));
                } else {
                    if p.2 != prev_layer {
                        // Draw a via
                        vias.push_str(&format!(
                            "  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"5\" fill=\"#ffffff\" stroke=\"#64748b\" stroke-width=\"2\" />\n",
                            p.0, p.1
                        ));
                    }
                    path.push_str(&format!("L {:.1} {:.1} ", p.0, p.1));
                }
                prev_layer = p.2;
            }
            
            // Dashed line if layer 1
            let stroke_dash = if t.points.last().unwrap().2 == 1 { "stroke-dasharray=\"5,5\"" } else { "" };
            
            out.push_str(&format!(
                "  <path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"4\" stroke-linejoin=\"round\" {} />\n",
                path, colors[i % colors.len()], stroke_dash
            ));
            
            out.push_str(&vias);

            // Draw pads
            out.push_str(&format!(
                "  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"8\" fill=\"#94a3b8\" />\n",
                t.points.first().unwrap().0,
                t.points.first().unwrap().1
            ));
            out.push_str(&format!(
                "  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"8\" fill=\"#94a3b8\" />\n",
                t.points.last().unwrap().0,
                t.points.last().unwrap().1
            ));
        }

        out.push_str("</svg>\n");
        out
    }
}
