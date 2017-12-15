#[macro_use]
extern crate turtle;

use turtle::Turtle;

run_turtle!(|mut turtle| {

    for _ in 0..36 {
        square(&mut turtle);
        turtle.right(10.0);
    }
});

fn square(turtle: &mut Turtle) {
    for _ in 0..4 {
        turtle.forward(200.0);
        turtle.right(90.0);
    }
}
