extern crate piston_window;

use std::thread;

use piston_window::*;

fn main() {
    let handle = thread::spawn(move || {
        let mut window: PistonWindow = WindowSettings::new(
            "Hello Piston!", [800, 600]
        ).exit_on_esc(true).build().unwrap();
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);

                let center = (200., 200.);
                let radius = 100.;
                let thickness = 1.;
                let mut prev = (center.0 + radius, center.0);
                for i in 0..361 {
                    let x = center.0 + radius * (i as f64).to_radians().cos();
                    let y = center.1 + radius * (i as f64).to_radians().sin();
                    line([0., 0., 0., 255.], thickness, [prev.0, prev.1, x, y], c.transform, g);
                    prev = (x, y);
                }
                line([0., 0., 0., 255.], 1., [250., 150., 400., 300.], c.transform, g);
                line([0., 0., 0., 255.], 1., [550., 150., 350., 500.], c.transform, g);
            });
        }
    });

    handle.join().unwrap();
}
