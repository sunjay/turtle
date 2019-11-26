//! https://en.wikipedia.org/wiki/B%C3%A9zier_curve

use turtle::{Turtle, Point};

struct CubicBezier {
    point0: Point,
    point1: Point,
    point2: Point,
    point3: Point,
}

impl CubicBezier {
    /// Returns the value of this curve at the given point
    pub fn at(&self, t: f64) -> Point {
        let &Self {point0, point1, point2, point3} = self;
        // Copying the formula from here verbatim:
        // https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Cubic_B%C3%A9zier_curves
        (1.0 - t).powi(3) * point0
            + 3.0*(1.0 - t)*(1.0 - t)*t*point1
            + 3.0*(1.0 - t)*t*t*point2
            + t.powi(3)*point3
    }
}

fn main() {
    let mut turtle = Turtle::new();

    let curve = CubicBezier {
        point0: Point {x: -200.0, y: -100.0},
        point1: Point {x: -100.0, y: 400.0},
        point2: Point {x: 100.0, y: -500.0},
        point3: Point {x: 300.0, y: 200.0},
    };

    let start = curve.at(0.0);
    turtle.pen_up();
    turtle.go_to(start);
    turtle.pen_down();

    let samples = 100;
    for i in 0..samples {
        let t = i as f64 / samples as f64;
        let point = curve.at(t);
        turtle.go_to(point);
    }
}
