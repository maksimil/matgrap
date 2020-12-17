#[derive(Debug)]
pub struct Point {
    pub label: String,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Line {
    pub start: String,
    pub finish: String,
}

#[derive(Debug)]
pub struct Canvas {
    points: Vec<Point>,
    lines: Vec<Line>,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            points: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn add_point(&mut self, p: Point) {
        self.points.push(p);

        let mut i = self.points.len() - 1;

        while 0 < i && self.points[i].label < self.points[i - 1].label {
            self.points.swap(i, i - 1);
            i -= 1;
        }
    }

    pub fn get_point_index(&self, label: &str) -> Option<usize> {
        let mut l = 0;
        let mut r = self.points.len();

        while r - l > 1 {
            let m = (r + l) / 2;
            let mlabel = self.points[m].label.as_str();

            if label < mlabel {
                r = m
            } else {
                l = m;
            }
        }

        if self.points[l].label.as_str() == label {
            Some(l)
        } else {
            None
        }
    }

    pub fn get_point(&self, label: &str) -> Option<&Point> {
        self.get_point_index(label).map(|i| &self.points[i])
    }

    pub fn get_point_mut(&mut self, label: &str) -> Option<&mut Point> {
        let i = self.get_point_index(label);
        match i {
            Some(i) => Some(&mut self.points[i]),
            None => None,
        }
    }

    pub fn add_line(&mut self, line: Line) -> bool {
        if self.get_point_index(&line.start).is_some()
            && self.get_point_index(&line.finish).is_some()
        {
            self.lines.push(line);
            true
        } else {
            false
        }
    }
}
