use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    // Rotate the whole image
    turtle.left(20.0);

    // Draw three isosceles triangles
    for _ in 0..3 {
        // Angles and side lengths found using:
        // https://www.triangle-calculator.com/?what=iso
        turtle.forward(120.0);
        turtle.right(110.0);
        turtle.forward(82.085);
        turtle.right(110.0);
        turtle.forward(120.0);
        turtle.right(140.0);

        // Rotate the turtle so that the triangles
        // aren't drawn over each other
        turtle.left(120.0);
    }
}
