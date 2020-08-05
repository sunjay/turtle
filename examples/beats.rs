extern crate turtle;
use turtle::{Point, Turtle};

fn main() {
    let mut turtle = Turtle::new();
    let init_pos: Point = turtle.position();

    let _radius_outer = 250.0;
    turtle.set_pen_color("#fe0000");
    turtle.pen_up();
    turtle.go_to((init_pos.x - (_radius_outer), init_pos.y));
    turtle.pen_down();
    turtle.set_fill_color("#fe0000");
    turtle.begin_fill();
    turtle.arc(_radius_outer, None, None);
    turtle.end_fill();

    let _radius_mid = 115.0;
    turtle.pen_up();
    turtle.go_to((init_pos.x.round() - (_radius_mid), init_pos.y));
    turtle.pen_down();
    turtle.set_fill_color("white");
    turtle.set_pen_color("white");
    turtle.begin_fill();
    turtle.arc(_radius_mid, None, None);
    turtle.end_fill();

    let _radius_inner = 60.0;
    turtle.pen_up();
    turtle.go_to((init_pos.x - (_radius_inner), init_pos.y));
    turtle.pen_down();
    turtle.set_fill_color("#fe0000");
    turtle.begin_fill();
    turtle.arc(_radius_inner, None, None);
    turtle.end_fill();

    let _size = 25.0;
    turtle.pen_up();
    turtle.go_to((init_pos.x - (_radius_mid) + _size - 2.0, init_pos.y));
    turtle.pen_down();
    turtle.set_pen_size(_size);
    turtle.forward(250.0);
}
