use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();

    // Rotate the whole image by 30 degrees
    turtle.left(30.0);

    // Draw three triangles
    for _ in 0..3 {
        // Draw one triangle
        for _ in 0..3 {
            turtle.forward(100.0);
            turtle.right(120.0);
        }

        // Rotate the turtle so that the triangles
        // aren't drawn over each other
        turtle.left(120.0);
    }
}
