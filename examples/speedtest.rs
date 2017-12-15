#[macro_use]
extern crate turtle;

/// This program draws several parallel lines to demonstrate each of the
/// different possible movement speeds
run_turtle!(|mut turtle| {

    turtle.pen_up();
    turtle.set_speed("instant");
    turtle.left(90.0);
    turtle.forward(300.0);
    turtle.right(90.0);
    turtle.pen_down();

    let length = 200.0;

    for i in 1..12 {
        turtle.set_speed(i);
        turtle.forward(length);

        turtle.pen_up();
        turtle.set_speed("instant");
        turtle.backward(length);
        turtle.right(90.0);
        turtle.forward(60.0);
        turtle.left(90.0);
        turtle.pen_down();
    }

    turtle.hide();
});
