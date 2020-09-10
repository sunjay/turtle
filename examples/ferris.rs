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

    let start_point = ap([240.0, 504.0]);
    turtle.set_fill_color(BACK_SHELL_COLOR);
    turtle.set_speed("instant");
    turtle.turn_towards(start_point);
    turtle.go_to(start_point);
    turtle.set_speed("faster");
    turtle.begin_fill();

    let start_heading = (180.0 + 32.01f64).to_radians();
    turtle.set_heading(start_heading);
    turtle.forward(ad(18.9, start_heading));
    turtle.bezier_rel(rp([
        ap([223.64, 514.18]),
        ap([222.18, 525.09]),
        ap([288.00, 597.09]),
        ap([323.94, 645.06]),
    ]));
    turtle.bezier_rel(rp([
        ap([323.91, 645.18]),
        ap([324.00, 623.09]),
        ap([283.27, 565.27]),
        ap([270.69, 529.94]),
    ]));
    turtle.bezier_rel(rp([
        ap([270.75, 529.94]),
        ap([444.75, 621.50]),
        ap([826.25, 622.50]),
        ap([973.09, 515.22]),
    ]));
    turtle.bezier_rel(rp([
        ap([973.12, 515.25]),
        ap([965.45, 552.18]),
        ap([923.09, 619.45]),
        ap([922.09, 646.52]),
    ]));
    turtle.bezier_rel(rp([
        ap([922.38, 647.00]),
        ap([931.09, 631.09]),
        ap([988.91, 554.55]),
        ap([1008.00, 517.12]),
    ]));
    let middle_curve_part1 = rp([
        ap([1008.12, 517.06]),
        ap([979.25, 500.88]),
        ap([960.12, 507.19]),
        ap([946.69, 510.31]),
    ]);
    turtle.bezier_rel(middle_curve_part1);
    let middle_curve_part2 = rp([
        ap([946.66, 510.41]),
        ap([750.73, 606.91]),
        ap([472.55, 603.45]),
        ap([239.57, 505.39]),
    ]);
    turtle.bezier_rel(middle_curve_part2);

    turtle.end_fill();
}
