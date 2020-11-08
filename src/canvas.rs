use std::collections::HashMap;

pub struct Point {
    label: String,
    x: f64,
    y: f64,
}

pub struct Line<'a> {
    start: &'a Point,
    finish: &'a Point,
}

pub struct Canvas<'a> {
    points: HashMap<String, Point>,
    lines: Vec<Line<'a>>,
}

impl<'a> Canvas<'a> {
    pub fn new<'b>() -> Canvas<'b> {
        Canvas {
            points: HashMap::new(),
            lines: Vec::new(),
        }
    }
}
