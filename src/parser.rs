use crate::{Canvas, Line, Point};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref POINT_REGEX: Regex = Regex::new(
        "([A-Z]+[0-9]*)[[:space:]]*\\(([0-9|\\.]*)[[:space:]]*,[[:space:]]*([0-9|\\.]*)\\)"
    )
    .unwrap();
    static ref LINE_REGEX: Regex = Regex::new("([A-Z]+[0-9]*)[[:space:]]*([A-Z]+[0-9]*)").unwrap();
}

pub fn parse(text: &str) -> Canvas {
    let mut canvas = Canvas::new();

    // points
    for cap in POINT_REGEX.captures_iter(text) {
        canvas.add_point(Point {
            label: String::from(cap.get(1).expect("Failed to get point label").as_str()),
            x: cap
                .get(2)
                .map(|v| {
                    v.as_str()
                        .parse::<f64>()
                        .expect("Failed to parse f64 from point position")
                })
                .expect("Failed to get x coord"),
            y: cap
                .get(3)
                .map(|v| {
                    v.as_str()
                        .parse::<f64>()
                        .expect("Failed to parse f64 from point position")
                })
                .expect("Failed to get y coord"),
        });
    }

    // lines
    for cap in LINE_REGEX.captures_iter(text) {
        if !canvas.add_line(Line {
            start: String::from(cap.get(1).expect("Failed to get start of line").as_str()),
            finish: String::from(cap.get(2).expect("Failed to get finish of line").as_str()),
        }) {
            panic!("Points were not found");
        }
    }

    canvas
}
