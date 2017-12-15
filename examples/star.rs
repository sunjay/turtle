#[macro_use]
extern crate turtle;

run_turtle!(|mut turtle| {

    turtle.right(90.0);
    turtle.set_pen_size(2.0);
    turtle.set_pen_color("yellow");

    for _ in 0..5 {
        turtle.forward(300.0);
        turtle.right(180.0 - (180.0 / 5.0));
    }
});
