use turtle::{Distance, Drawing, Point, Size, Turtle};

const LEVELS: u8 = 7;
const SPEED: f64 = 25.0;
const BORDER_MARGIN: Distance = 15.0;

fn turn_and_go_to(turtle: &mut Turtle, dest: Point) {
    turtle.turn_towards(dest);
    turtle.go_to(dest);
}

fn sierpinski_triangle(turtle: &mut Turtle, level: u8, size: Distance) {
    if level == 0 {
        turtle.pen_down();

        for _ in 0..3 {
            turtle.forward(size);
            turtle.left(120.0);
        }

        turtle.pen_up();
    } else {
        let next_level = level - 1;
        let next_size = size / 2.0;

        sierpinski_triangle(turtle, next_level, next_size);

        turtle.forward(next_size);

        sierpinski_triangle(turtle, next_level, next_size);

        turtle.left(120.0);
        turtle.forward(next_size);
        turtle.right(120.0);

        sierpinski_triangle(turtle, next_level, next_size);

        turtle.right(120.0);
        turtle.forward(next_size);
        turtle.left(120.0);
    }
}

fn sierpinski_triangle_auto(turtle: &mut Turtle, level: u8, canvas_size: Size) {
    let auto_size = canvas_size.width.min(canvas_size.height) as f64 - 2.0 * BORDER_MARGIN;
    let start_point_coord = -auto_size / 2.0;

    turtle.pen_up();
    turn_and_go_to(turtle, [start_point_coord, start_point_coord].into());
    turtle.set_heading(0.0);

    sierpinski_triangle(turtle, level, auto_size);
}

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle = drawing.add_turtle();

    turtle.use_degrees();
    turtle.set_speed(SPEED);

    sierpinski_triangle_auto(&mut turtle, LEVELS, drawing.size());
}
