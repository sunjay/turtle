//Ulam Spiral
//A plot of primes along a square spiral as such
//where each side length is repeated once then 1
//space is added to grow.
//
//55555555555555555
//5
//5  3.3.3.3.3.4
//5  3         4
//5  3  111    4
//5  3  1 2    4
//5  3    2    4
//5  322222    4
//5            4
//44444444444444
use turtle::{Drawing, ExportError};

 fn main() -> Result<(), ExportError> {
 //Each ring half grows by 1.0 per 'next' which is
 //described in the above image. Each half is 2 sides.
 
 //step is the ring spacing size.
 
 //count is the total steps drawn and what
 //is determined to be prime or not.
 
	let mut drawing = Drawing::new();
	let mut turtle = drawing.add_turtle();	
	turtle.set_speed("instant");
	let mut count: f64 = 1.0;
	let step = 4.0;
	let mut next = 1;
	
	//size of spiral in lines x2
     for _i in 0..=192 {		//Adjust to size
		let mut cvr = next.clone();		//a clean copy
		while cvr != 0 {		//decrementor for count against the side len
			let mut result = true;		//holds prime check
			if count >= 2.0 {		//min check value
				let sqrt = count.sqrt();		//Find square root of count
				let mut trsqrt = sqrt.clone().trunc() + 1.0; //truncate, round up
				while trsqrt >= 2.0 {
				//Check all the modulus of the real values from the 
				//rounded up square root down to 2.
				//If any == 0.0 then it is composite, else prime.
					if count % trsqrt == 0.0 {
						result = false;
						}
					trsqrt -= 1.0;	//down to 2
					}
				if count == 2.0 {	// 2 is prime
					result = true;
				}
			}
		if count <= 1.0 { // 1 is not prime
			result = false;
		}
	
	//If prime then draw a larger space then for the other spaces.
	//Then restore the size for the next draw.
	//If composite then just draw normally.
		if result == true {
		turtle.set_pen_size(2.0);
		turtle.forward(step);
		turtle.set_pen_size(1.0);
		} else { 
         turtle.forward(step);
         }
         cvr -= 1;	//decrementor to zer0
         count += 1.0;	//increment the total count
         }
         
	//When done with the side above then turn and repeat.
		turtle.right(90.0);
		cvr = next.clone();
		while cvr != 0 {
		let mut result2 = true;
		if count >= 2.0 {
			let sqrt = count.sqrt();
			let mut trsqrt = sqrt.clone().trunc() + 1.0; 
			while trsqrt >= 2.0 {				
				if count % trsqrt == 0.0 {
					result2 = false;
				}
				trsqrt -= 1.0;
			}
			if count == 2.0 {
				result2 = true;
			}
		}
		if count <= 1.0 {
				result2 = false;
			}
		if result2 == true {
		turtle.set_pen_size(2.0);
		turtle.forward(step);
		turtle.set_pen_size(1.0);
		} else { 
		turtle.forward(step);
		}
		cvr -= 1;
		count += 1.0;
		}
		turtle.right(90.0);
        next += 1;//Increment the side counter
	}
//Save the picture.
	turtle.hide();
    drawing.save_svg("ulamspiral.svg")?;
    Ok(())
 }
