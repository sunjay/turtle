use turtle::Turtle;

struct MyCoord {
    x: u8,
}

impl From<MyCoord> for f64 {
    fn from(source: MyCoord) -> Self {
        f64::from(source.x)
    }
}

fn main() {
    let mut turtle = Turtle::new();

    turtle.forward(200);
    turtle.right(90);
    turtle.forward(200u8);
    turtle.right(90f32);
    turtle.forward(200.0);
    turtle.right(90.0);
    turtle.forward(MyCoord { x: 200 });
    turtle.right(MyCoord { x: 90 });
}
