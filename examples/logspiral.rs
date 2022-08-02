//Logarithmic Spiral Example

use turtle::{Drawing, ExportError};

fn main() -> Result<(), ExportError> { 
 let mut fval = 0.0;
 let mut drawing = Drawing::new();
 let mut turtle = drawing.add_turtle();	
  turtle.set_speed("instant");

 
     for _i in 0..=8192 {
         turtle.forward(fval);
         turtle.right(1.0);
         
	//Adjust the tightness of the spiral ex. 0.0011 - 0.0009
	//Change sign to change direction
        fval -= 0.001;
           }
	
	turtle.hide();
    drawing.save_svg("logspiral.svg")?;
    Ok(())	
 }
