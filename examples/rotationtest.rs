#[macro_use]
extern crate turtle;

/// This program demonstrates rotations at multiple speeds
run_turtle!(|mut turtle| {

    turtle.pen_up();
    turtle.set_speed("instant");
    turtle.left(90.0);
    turtle.forward(300.0);
    turtle.right(180.0);
    turtle.pen_down();

    for i in 1..12 {
        turtle.set_speed(i);
        turtle.right(2.0 * 360.0);

        turtle.pen_up();
        turtle.set_speed("instant");
        turtle.forward(60.0);
        turtle.pen_down();
    }

    turtle.pen_up();
    turtle.set_speed("instant");
    turtle.backward(720.0);
    turtle.pen_down();

    for i in 1..12 {
        turtle.set_speed(i);
        turtle.left(2.0 * 360.0);

        turtle.pen_up();
        turtle.set_speed("instant");
        turtle.forward(60.0);
        turtle.pen_down();
    }
});
