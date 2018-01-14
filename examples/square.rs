#[macro_use]
extern crate turtle;

run_turtle!(|mut turtle| {

    for _ in 0..4 {
        turtle.forward(200.0);
        turtle.right(90.0);
    }
});
