//Faux-bonacci Spiral Example
//Maybe ot quite a true Fibonacci spiral but pretty close
use turtle::{Drawing, ExportError};


 fn main() -> Result<(), ExportError> {
 
 let mut fval = 0.0;
 let mut drawing = Drawing::new();
 let mut turtle = drawing.add_turtle();
	
     for i in 0..=960 {
         turtle.forward(fval);
         turtle.right(1.0);
         
	//Adjust the tightness of the spiral ex. 0.00001-0.00003
	//Change sign to change direction.
	
        fval += 0.0000168 * i as f64;
        
           }
	
	turtle.hide();
    drawing.save_svg("fibspiral.svg")?;
    Ok(())
 }
