#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug)]
pub struct Color {
    pub h: f64, // 0-360
    pub s: f64, // 0-1
    pub l: f64, // 0-1
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub points: Vec<Point>,
    pub color: Color,
}
