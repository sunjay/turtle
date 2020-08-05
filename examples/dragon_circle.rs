extern crate turtle;
use turtle::Turtle;

enum Turn {
    Left,
    Right,
}

// Shameless copy from https://gist.github.com/fogleman/006e4069348fa163b8ae

fn turn(i: i64) -> Turn {
    let left = (((i & -i) << 1) & i) != 0;
    if left {
        return Turn::Left;
    } else {
        return Turn::Right;
    }
}

fn main() {
    let mut turtle = Turtle::new();
    turtle.set_speed("instant");
    for i in 1..100000 {
        match turn(i) {
            Turn::Left => turtle.arc(-4.0, Some(90.0), Some(36)),
            Turn::Right => turtle.arc(4.0, Some(90.0), Some(36)),
        }
    }
}
