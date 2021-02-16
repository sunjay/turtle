use turtle::Turtle;
use chrono::{Local, Timelike};
use std::{thread, time};

fn main() {
    let hours = 12;
    let minutes = 60.0;
    let seconds = 60.0;
    let full_circle = 360.0;
    let second = time::Duration::from_millis(1000);
    
    let mut turtle = Turtle::new();
    turtle.set_speed("instant");
    turtle.hide();

    //A clock runs forever
    loop{
        //clean everything
        turtle.clear();
        turtle.home();

        //Get local time
        let now = Local::now();

        //Draw the clock
        for i in 1..=hours{
            turtle.pen_up();
            turtle.forward(205.0);
            if (i - 1) % 3 == 0 {
                turtle.set_pen_size(5.0);
            }else{
                turtle.set_pen_size(1.0);
            }
            turtle.pen_down();
            turtle.forward(10.0);
            turtle.home();
            turtle.right(full_circle / hours as f64 * i as f64);
        }

        //Draw the hour hand
        turtle.home();
        turtle.right(full_circle / hours as f64 * now.hour() as f64);
        turtle.set_pen_size(5.0);
        turtle.forward(150.0);

        //Draw the minute hand
        turtle.home();
        turtle.right(full_circle / minutes * now.minute() as f64);
        turtle.set_pen_size(3.0);
        turtle.forward(160.0);

        //Draw the second hand
        turtle.home();
        turtle.right(full_circle / seconds * now.second() as f64);
        turtle.set_pen_size(1.0);
        turtle.forward(175.0);

        //And update every second
        thread::sleep(second);
    }
}