use turtle::{Angle, Color, Distance, Drawing, Point, Size, Turtle};

trait CubicBezier {
    const DEFAULT_SAMPLES: usize = 100;

    fn bezier_abs_pr(&mut self, samples: usize, points: [Point; 4]);
    fn bezier_rel_pr(&mut self, samples: usize, rel_points: [Point; 3]);

    fn bezier_abs(&mut self, points: [Point; 4]) {
        self.bezier_abs_pr(Self::DEFAULT_SAMPLES, points)
    }

    fn bezier_rel(&mut self, rel_points: [Point; 3]) {
        self.bezier_rel_pr(Self::DEFAULT_SAMPLES, rel_points)
    }

    fn curve_at(t: f64, points: [Point; 4]) -> Point {
        (1.0 - t).powi(3) * points[0]
            + 3.0 * (1.0 - t).powi(2) * t * points[1]
            + 3.0 * (1.0 - t) * t.powi(2) * points[2]
            + t.powi(3) * points[3]
    }
}

impl CubicBezier for Turtle {
    fn bezier_abs_pr(&mut self, samples: usize, points: [Point; 4]) {
        (0..=samples)
            .map(|i| i as f64 / samples as f64)
            .map(|t| Self::curve_at(t, points))
            .for_each(|point| {
                self.turn_towards(point);
                self.go_to(point);
            })
    }

    fn bezier_rel_pr(&mut self, samples: usize, rel_points: [Point; 3]) {
        let pos = self.position();
        self.bezier_abs_pr(
            samples,
            [
                pos,
                pos + rel_points[0],
                pos + rel_points[1],
                pos + rel_points[2],
            ],
        )
    }
}

const FRONT_SHELL_COLOR: &str = "#f74c00";
const BACK_SHELL_COLOR: &str = "#a52b00";
const PUPIL_COLOR: &str = "#ffffff";
const MOUTH_COLOR: &str = "#000000";
const ORIGINAL_SIZE: Size = Size {
    width: 1200,
    height: 800,
};

fn adapt_point(point: impl Into<Point>, size: Size) -> Point {
    let point = point.into();
    Point {
        x: size.width as f64 * point.x / ORIGINAL_SIZE.width as f64,
        y: -(size.height as f64) * point.y / ORIGINAL_SIZE.height as f64,
    }
}

fn rel_points(abs_points: [Point; 4]) -> [Point; 3] {
    [
        abs_points[1] - abs_points[0],
        abs_points[2] - abs_points[0],
        abs_points[3] - abs_points[0],
    ]
}

fn adapt_distance(distance: Distance, angle: Angle, size: Size) -> Distance {
    ((distance * angle.cos() * size.width as f64 / ORIGINAL_SIZE.width as f64).powi(2)
        + (distance * angle.sin() * size.height as f64 / ORIGINAL_SIZE.height as f64).powi(2))
    .sqrt()
}

fn main() {
    let mut drawing = Drawing::new();
    let size = drawing.size();
    drawing.set_center([size.width as f64 / 2.0, -(size.height as f64) / 2.0]);

    let mut turtle = drawing.add_turtle();
    turtle.use_radians();
    turtle.pen_up();

    let rp = rel_points;
    let ap = |point| adapt_point(point, size);
    let ad = |distance, angle| adapt_distance(distance, angle, size);

    // turtle.bezier_rel(rp([
    //     ap([]),
    //     ap([]),
    //     ap([]),
    //     ap([]),
    // ]));
}
