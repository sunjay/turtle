//3d Logarithmic Spiral Example
//This takes a little while so 
// turtle.set_speed("instant");

use turtle::{Drawing, ExportError};

fn main() -> Result<(), ExportError> { 
 
 let mut fval = 0.0;
 let tval = 1.0;
 let mut drawing = Drawing::new();
 let mut turtle = drawing.add_turtle();	
 turtle.set_speed("instant");
	for _j in 0..=3 {
     for _i in 0..=8192 {
         turtle.forward(fval);
         turtle.right(tval);
         
	//Adjust the tightness of the spiral
        fval -= 0.0009;
           }
	 turtle.right(90.0);
	 turtle.forward(0.1);
	 turtle.right(90.0);
	 
	 for _i in 0..=8192 {
         turtle.forward(fval);
         turtle.left(tval);
         
	//Adjust the tightness of the spiral
        fval += 0.0009;
           }
          turtle.left(90.0);
	 turtle.forward(0.1);
	 turtle.left(90.0); 
	 }
	turtle.hide();
    drawing.save_svg("three_d_logspiral.svg")?;
    Ok(())	
 }
