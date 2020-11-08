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

pub struct Canvas {
    points: HashMap<String, Point>,
    lines: Vec<Line>;
}
