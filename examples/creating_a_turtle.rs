extern crate turtle;

use turtle::Turtle;

const SIZE: f64 = 3.0;

fn main() {
    let mut turtle = Turtle::new();
    turtle.set_speed(8);

    turtle.pen_up();
    turtle.set_x(-280.0);
    turtle.set_y(-90.0);

    // Panzer
    {
        turtle.set_fill_color([62.0, 114.0, 29.0, 1.0]);
        turtle.begin_fill();

        for _ in 0..180 {
            turtle.forward(SIZE);
            turtle.right(1.0);
        }

        for _ in 0..90 {
            turtle.forward(SIZE / 3.0);
            turtle.right(1.0);
        }

        turtle.set_speed(5);
        let d = SIZE * 360.0 / std::f64::consts::PI;
        turtle.forward(d - d / 3.0);
        turtle.set_speed(10);

        for _ in 0..90 {
            turtle.forward(SIZE / 3.0);
            turtle.right(1.0);
        }

        turtle.end_fill();
    }

    // Tail
    {
        turtle.set_fill_color([119.0, 178.0, 85.0, 1.0]);
        turtle.begin_fill();

        turtle.left(90.0);
        turtle.forward(SIZE);

        for _ in 0..45 {
            turtle.forward(SIZE / 3.0);
            turtle.left(1.0);
        }

        turtle.forward(SIZE * 3.0);

        for _ in 0..25 {
            turtle.forward(SIZE / 3.0);
            turtle.left(6.0);
        }

        turtle.forward(SIZE * 9.0);
        turtle.right(6.0);
        turtle.forward(SIZE * 8.0);

        turtle.end_fill();
    }

    turtle.pen_up();
    turtle.right(55.0);
    turtle.forward(SIZE * 10.0);
    turtle.right(55.0);

    // Hind leg
    draw_leg(&mut turtle);

    turtle.right(86.5);
    turtle.forward(SIZE * 55.0);
    turtle.right(86.0);

    // Front leg
    draw_leg(&mut turtle);

    turtle.right(54.5);
    turtle.forward(SIZE * 8.0);

    // Neck
    {
        turtle.begin_fill();

        for _ in 0..15 {
            turtle.forward(SIZE * 3.0);
            turtle.left(1.5);
        }

        turtle.end_fill();
        turtle.begin_fill();

        turtle.left(100.0);
        turtle.forward(SIZE * 20.0);
        turtle.left(80.0);

        for _ in 0..4 {
            turtle.forward(SIZE * 3.7);
            turtle.left(1.5);
        }

        turtle.left(30.0);

        for _ in 0..27 {
            turtle.forward(SIZE);
            turtle.right(1.0);
        }

        turtle.end_fill();
    }

    turtle.right(172.6);
    turtle.forward(SIZE * 40.0);
    turtle.right(110.0);

    // Head
    {
        turtle.begin_fill();

        for _ in 0..20 {
            turtle.forward(SIZE * 1.2);
            turtle.left(1.0);
        }

        for _ in 0..10 {
            turtle.forward(SIZE * 1.2);
            turtle.left(4.0);
        }

        for _ in 0..10 {
            turtle.forward(SIZE / 1.5);
            turtle.left(7.0);
        }

        for _ in 0..10 {
            turtle.forward(SIZE);
            turtle.left(2.0);
        }

        for _ in 0..50 {
            turtle.forward(SIZE / 2.5);
            turtle.left(1.0);
        }

        for _ in 0..30 {
            turtle.forward(SIZE / 3.0);
            turtle.left(1.8);
        }

        for _ in 0..10 {
            turtle.forward(SIZE / 1.5);
            turtle.left(1.8);
        }

        turtle.end_fill();
    }

    turtle.left(128.0);
    turtle.forward(SIZE * 15.0);

    // Eye
    {
        turtle.set_fill_color("black");
        turtle.begin_fill();
        for _ in 0..90 {
            turtle.forward(SIZE / 3.0);
            turtle.right(4.0);
        }
        turtle.end_fill();
    }

    turtle.set_fill_color([119.0, 178.0, 85.0, 1.0]);

    turtle.left(175.0);
    turtle.forward(SIZE * 43.0);

    // Glare
    {
        // Right glare
        {
            turtle.begin_fill();

            for _ in 0..39 {
                turtle.forward(SIZE / 5.0);
                turtle.left(2.0);
            }

            turtle.forward(SIZE * 36.0);

            for _ in 0..90 {
                turtle.forward(SIZE / 2.5);
                turtle.left(2.0);
            }

            for _ in 0..42 {
                turtle.forward(SIZE);
                turtle.left(0.9);
            }

            for _ in 0..26 {
                turtle.forward(SIZE / 5.0);
                turtle.left(2.0);
            }

            turtle.end_fill();
        }

        turtle.right(18.0);
        turtle.forward(SIZE * 37.0);
        turtle.set_heading(180.0);

        // Middle glare
        {
            turtle.begin_fill();

            for _ in 0..40 {
                turtle.forward(SIZE / 4.0);
                turtle.left(2.0);
            }

            turtle.forward(SIZE * 47.0);
            turtle.left(8.5);

            for _ in 0..90 {
                turtle.forward(SIZE / 2.0);
                turtle.left(2.0);
            }
            turtle.forward(SIZE / 2.0);

            turtle.left(8.5);
            turtle.forward(SIZE * 47.0);

            for _ in 0..40 {
                turtle.left(2.0);
                turtle.forward(SIZE / 4.0);
            }

            turtle.end_fill();
        }

        turtle.left(24.0);
        turtle.forward(SIZE * 36.0);
        turtle.set_heading(180.0);

        // Left glare
        {
            turtle.begin_fill();

            for _ in 0..26 {
                turtle.forward(SIZE / 5.0);
                turtle.left(2.0);
            }

            for _ in 0..42 {
                turtle.forward(SIZE);
                turtle.left(0.9);
            }

            for _ in 0..90 {
                turtle.forward(SIZE / 2.5);
                turtle.left(2.0);
            }

            turtle.forward(SIZE * 36.0);

            for _ in 0..39 {
                turtle.forward(SIZE / 5.0);
                turtle.left(2.0);
            }

            turtle.end_fill();
        }
    }

}

fn draw_leg(turtle: &mut Turtle) {
    turtle.begin_fill();

    for _ in 0..15 {
        turtle.forward(SIZE);
        turtle.left(0.5);
    }

    for _ in 0..90 {
        turtle.forward(SIZE / 6.0);
        turtle.left(1.0);
    }

    turtle.forward(SIZE * 3.0);

    for _ in 0..90 {
        turtle.forward(SIZE / 6.0);
        turtle.left(1.0);
    }

    turtle.forward(SIZE * 14.5);

    turtle.end_fill();
}
