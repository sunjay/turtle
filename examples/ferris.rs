use turtle::{Angle, Color, Distance, Drawing, Point, Size, Turtle};

trait Rotate {
    fn rot(&self, angle: Angle) -> Self;
}

impl Rotate for Point {
    fn rot(&self, angle: Angle) -> Self {
        let (sin_angle, cos_angle) = angle.sin_cos();
        [
            self.x * cos_angle - self.y * sin_angle,
            self.x * sin_angle + self.y * cos_angle,
        ]
        .into()
    }
}

trait CubicBezier {
    const DEFAULT_SAMPLES: usize = 100;

    fn bezier_abs_pr(&mut self, samples: usize, points: [Point; 4]);
    fn bezier_rel_pr(&mut self, samples: usize, rel_points: [Point; 3]);
    fn bezier_rel_head_pr(&mut self, samples: usize, rel_points: [Point; 3]);

    fn bezier_abs(&mut self, points: [Point; 4]) {
        self.bezier_abs_pr(Self::DEFAULT_SAMPLES, points)
    }

    fn bezier_rel(&mut self, rel_points: [Point; 3]) {
        self.bezier_rel_pr(Self::DEFAULT_SAMPLES, rel_points)
    }

    fn bezier_rel_head(&mut self, rel_points: [Point; 3]) {
        self.bezier_rel_head_pr(Self::DEFAULT_SAMPLES, rel_points)
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

    fn bezier_rel_head_pr(&mut self, samples: usize, rel_points: [Point; 3]) {
        let pos = self.position();
        let rot_angle = self.heading() - rel_points[0].atan2();
        self.bezier_abs_pr(
            samples,
            [
                pos,
                pos + rel_points[0].rot(rot_angle),
                pos + rel_points[1].rot(rot_angle),
                pos + rel_points[2].rot(rot_angle),
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

fn adapted_head_forward(turtle: &mut Turtle, distance: Distance, head: Angle, size: Size) {
    turtle.set_heading(head);
    turtle.forward(adapt_distance(distance, head, size));
}

fn turn_and_go_to(turtle: &mut Turtle, point: Point) {
    turtle.turn_towards(point);
    turtle.go_to(point);
}

fn next_fill(turtle: &mut Turtle, color: impl Into<Color>) {
    next_fill_at(turtle, color, turtle.position());
}

fn next_fill_at(turtle: &mut Turtle, color: impl Into<Color>, pos: Point) {
    turtle.end_fill();
    turtle.set_fill_color(color.into());
    turn_and_go_to(turtle, pos);
    turtle.begin_fill();
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
    let ahf =
        |turtle: &mut Turtle, distance, head| adapted_head_forward(turtle, distance, head, size);

    turtle.set_fill_color(BACK_SHELL_COLOR);
    turtle.set_speed("instant");
    turn_and_go_to(&mut turtle, ap([240.0, 504.0]));
    turtle.set_speed("faster");
    turtle.begin_fill();

    ahf(&mut turtle, 18.9, (180.0 + 32.01f64).to_radians());
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

    next_fill(&mut turtle, FRONT_SHELL_COLOR);

    turtle.bezier_rel(rp([
        ap([239.45, 504.36]),
        ap([233.00, 508.18]),
        ap([213.45, 519.82]),
        ap([212.50, 524.44]),
    ]));
    turtle.bezier_rel(rp([
        ap([212.25, 524.44]),
        ap([215.00, 570.50]),
        ap([300.00, 657.50]),
        ap([304.52, 700.83]),
    ]));
    turtle.bezier_rel(rp([
        ap([303.56, 702.56]),
        ap([206.50, 599.75]),
        ap([133.25, 541.75]),
        ap([138.00, 522.82]),
    ]));
    turtle.bezier_rel(rp([
        ap([138.00, 523.18]),
        ap([137.82, 511.73]),
        ap([145.55, 510.64]),
        ap([176.00, 476.00]),
    ]));
    turtle.bezier_rel(rp([
        ap([175.64, 475.55]),
        ap([138.73, 455.36]),
        ap([111.09, 430.55]),
        ap([103.73, 408.91]),
    ]));
    turtle.bezier_rel(rp([
        ap([103.91, 409.55]),
        ap([-21.00, 349.50]),
        ap([5.00, 282.50]),
        ap([52.00, 244.12]),
    ]));
    turtle.bezier_rel(rp([
        ap([52.69, 244.19]),
        ap([59.00, 289.00]),
        ap([83.12, 323.00]),
        ap([109.56, 344.59]),
    ]));
    ahf(&mut turtle, 124.2, (180.0 - 82.13f64).to_radians());
    turtle.bezier_rel(rp([
        ap([93.13, 221.13]),
        ap([163.25, 250.75]),
        ap([216.25, 316.75]),
        ap([148.35, 405.39]),
    ]));
    turtle.bezier_rel(rp([
        ap([148.52, 405.43]),
        ap([160.27, 422.64]),
        ap([185.00, 435.45]),
        ap([207.04, 443.65]),
    ]));
    ahf(&mut turtle, 12.7, 45f64.to_radians());

    let spike_start_turn = 52.57f64.to_radians();
    let spike_start_length = 49.2;
    let spike_top_curve = rp([
        ap([509.52, 106.52]),
        ap([512.17, 99.17]),
        ap([524.30, 96.83]),
        ap([529.52, 102.48]),
    ]);
    let spike_end_turn = 58.28f64.to_radians();
    let spike_end_length = 47.49;
    let spike_between_length = 12.0;
    turtle.left(81.32f64.to_radians() - spike_start_turn);

    for _ in 0..13 {
        turtle.left(spike_start_turn);
        turtle.forward(ad(spike_start_length, turtle.heading()));
        turtle.bezier_rel_head(spike_top_curve);
        turtle.forward(ad(spike_end_length, turtle.heading()));
        turtle.left(spike_end_turn);
        turtle.forward(ad(spike_between_length, turtle.heading()));
    }

    turtle.bezier_rel(rp([
        ap([1000.36, 439.64]),
        ap([1008.36, 434.45]),
        ap([1035.27, 410.91]),
        ap([1041.87, 402.91]),
    ]));
    ahf(&mut turtle, 22.8, 66.8f64.to_radians());
    turtle.bezier_rel(rp([
        ap([1050.65, 381.52]),
        ap([980.00, 298.25]),
        ap([1027.75, 236.75]),
        ap([1109.50, 200.62]),
    ]));
    turtle.bezier_rel(rp([
        ap([1109.57, 200.26]),
        ap([1096.75, 232.50]),
        ap([1100.12, 304.25]),
        ap([1100.13, 316.87]),
    ]));
    turtle.bezier_rel(rp([
        ap([1101.50, 316.91]),
        ap([1139.00, 298.00]),
        ap([1156.62, 260.62]),
        ap([1166.34, 219.34]),
    ]));
    turtle.bezier_rel(rp([
        ap([1166.53, 218.62]),
        ap([1216.75, 298.00]),
        ap([1148.50, 372.00]),
        ap([1085.75, 405.78]),
    ]));
    turtle.bezier_rel(rp([
        ap([1085.78, 405.72]),
        ap([1077.36, 420.27]),
        ap([1048.64, 453.45]),
        ap([1033.47, 465.31]),
    ]));
    ahf(&mut turtle, 60.8, -36.3f64.to_radians());
    turtle.bezier_rel(rp([
        ap([1081.88, 502.06]),
        ap([1084.65, 506.52]),
        ap([1088.26, 511.17]),
        ap([1074.84, 534.82]),
    ]));
    ahf(&mut turtle, 127.3, (180.0 + 55.56f64).to_radians());
    turtle.bezier_rel(rp([
        ap([1002.84, 639.87]),
        ap([975.18, 675.00]),
        ap([965.82, 698.00]),
        ap([959.53, 702.47]),
    ]));
    turtle.bezier_rel(rp([
        ap([959.13, 701.74]),
        ap([958.25, 656.00]),
        ap([1012.12, 586.75]),
        ap([1016.50, 523.19]),
    ]));
    ahf(&mut turtle, 11.4, (180.0 - 41.87f64).to_radians());
    turtle.bezier_rel(middle_curve_part1);
    turtle.bezier_rel(middle_curve_part2);

    next_fill_at(&mut turtle, MOUTH_COLOR, ap([539.88, 510.78]));

    turtle.bezier_rel(rp([
        ap([539.88, 510.78]),
        ap([498.12, 499.00]),
        ap([470.12, 463.28]),
        ap([512.38, 399.28]),
    ]));
    turtle.bezier_rel(rp([
        ap([512.44, 399.38]),
        ap([605.64, 358.55]),
        ap([627.09, 510.55]),
        ap([540.57, 510.48]),
    ]));

    next_fill_at(&mut turtle, PUPIL_COLOR, ap([522.57, 455.57]));

    turtle.bezier_rel(rp([
        ap([522.57, 455.57]),
        ap([495.45, 455.73]),
        ap([493.64, 397.55]),
        ap([522.53, 397.50]),
    ]));
    turtle.bezier_rel(rp([
        ap([522.18, 397.55]),
        ap([551.45, 397.55]),
        ap([549.73, 455.55]),
        ap([522.45, 455.55]),
    ]));

    next_fill_at(&mut turtle, MOUTH_COLOR, ap([710.44, 507.56]));

    turtle.bezier_rel(rp([
        ap([710.44, 507.56]),
        ap([675.09, 512.36]),
        ap([644.91, 454.00]),
        ap([677.57, 410.30]),
    ]));
    turtle.bezier_rel(rp([
        ap([677.52, 410.52]),
        ap([693.44, 389.69]),
        ap([730.06, 384.00]),
        ap([751.44, 411.53]),
    ]));
    turtle.bezier_rel(rp([
        ap([751.47, 411.56]),
        ap([782.36, 480.00]),
        ap([749.45, 500.91]),
        ap([711.50, 507.50]),
    ]));

    next_fill_at(&mut turtle, PUPIL_COLOR, ap([699.12, 453.53]));

    turtle.bezier_rel(rp([
        ap([699.12, 453.53]),
        ap([672.91, 453.55]),
        ap([671.73, 397.45]),
        ap([699.11, 395.44]),
    ]));
    turtle.bezier_rel(rp([
        ap([699.03, 395.53]),
        ap([727.55, 397.55]),
        ap([725.07, 453.56]),
        ap([699.06, 453.50]),
    ]));

    next_fill_at(&mut turtle, MOUTH_COLOR, ap([673.44, 530.4]));

    turtle.bezier_rel(rp([
        ap([673.44, 530.4]),
        ap([663.27, 570.64]),
        ap([611.91, 579.64]),
        ap([593.57, 536.61]),
    ]));
    ahf(&mut turtle, 80.2, 4.24f64.to_radians());

    turtle.end_fill();
    turtle.hide();
}
