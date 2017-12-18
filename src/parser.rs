use turtle::Turtle;

/// A parser from LOGO commands to turtle commands
pub struct Parser;

impl Parser {
    /// Parse the string and execut the commands.  
    /// 
    /// The turtle will execute the commands from the string.  
    /// # Example
    /// ```rust
    /// extern crate turtle;
    /// 
    /// use turtle::{Turtle,Parser};
    /// fn main() {
    ///     let mut turtle = Turtle::new();
    ///     turtle.set_speed(4);
    ///     // Parser::parse_string("fd 20 rt 90 fd 100 lt 90 bk 40", &mut turtle);
    /// }
    /// ```
    /// 
    pub fn parse_string(source: &str, turtle: &mut Turtle) {
        let ops: Vec<&str> = source.split(' ').collect();

        let mut ins_pointer = 0;

        while let Some(op) = ops.get(ins_pointer) {
            match *op {
                "fd" | "forward" => if let Some(dist) = ops.get(ins_pointer + 1) {
                    let dist: f64 = dist.parse().unwrap();
                    turtle.forward(dist as f64);
                    ins_pointer += 1;
                },
                "bk" | "back" => if let Some(dist) = ops.get(ins_pointer + 1) {
                    let dist: f64 = dist.parse().unwrap();
                    turtle.backward(dist as f64);
                    ins_pointer += 1;
                },
                "lt" | "left" => if let Some(dist) = ops.get(ins_pointer + 1) {
                    let dist: f64 = dist.parse().unwrap();
                    turtle.left(dist as f64);
                    ins_pointer += 1;
                },
                "rt" | "right" => if let Some(dist) = ops.get(ins_pointer + 1) {
                    let dist: f64 = dist.parse().unwrap();
                    turtle.right(dist as f64);
                    ins_pointer += 1;
                },
                _ => println!("command: {}", op),
            }
            ins_pointer += 1;
        }
    }
}
