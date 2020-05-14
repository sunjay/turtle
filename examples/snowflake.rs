// Source: http://www.algorithm.co.il/blogs/computer-science/fractals-in-10-minutes-no-6-turtle-snowflake/
// Modified to create a full snowflake with the fractals

use turtle::{Drawing, Turtle};

fn main() {
    let mut drawing = Drawing::new();
    let mut turtle = drawing.add_turtle();

    drawing.set_background_color("#29B6F6");
    turtle.set_pen_color("#B2EBF2");

    turtle.pen_up();
    turtle.set_speed("instant");
    turtle.backward(200.0);
    turtle.right(30.0);
    turtle.pen_down();

    turtle.set_speed("fast");
    fractal(&mut turtle, 350.0, 3);
    turtle.left(120.0);
    fractal(&mut turtle, 350.0, 3);
    turtle.left(120.0);
    fractal(&mut turtle, 350.0, 3);

    turtle.hide();
}

fn fractal(turtle: &mut Turtle, length: f64, depth: usize) {
    if depth == 0 {
        turtle.forward(length);
    } else {
        fractal(turtle, length / 3.0, depth - 1);
        turtle.right(60.0);
        fractal(turtle, length / 3.0, depth - 1);
        turtle.left(120.0);
        fractal(turtle, length / 3.0, depth - 1);
        turtle.right(60.0);
        fractal(turtle, length / 3.0, depth - 1);
    }
}
