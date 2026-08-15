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
pub enum ShapeType {
    Polygon(Vec<Point>),
    Circle {
        center: Point,
        radius: f64,
    },
    Rect {
        pos: Point,
        width: f64,
        height: f64,
        rx: f64,
    },
    Line {
        start: Point,
        end: Point,
    },
    Path(String), // SVG path d-string
    Text {
        pos: Point,
        text: String,
        font_size: f64,
    },
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub color: Color,
    pub fill_opacity: f64,
    pub stroke_color: Option<Color>,
    pub stroke_width: f64,
}
