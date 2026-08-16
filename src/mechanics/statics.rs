#[derive(Clone, Debug, serde::Serialize)]
pub struct Node {
    pub x: f64,
    pub y: f64,
    pub fixed: bool,
    pub force_x: f64,
    pub force_y: f64,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Bar {
    pub node_a: usize,
    pub node_b: usize,
    pub area: f64,
    pub stress: f64, // calculated
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Truss {
    pub nodes: Vec<Node>,
    pub bars: Vec<Bar>,
    pub total_mass: f64, // calculated
}
