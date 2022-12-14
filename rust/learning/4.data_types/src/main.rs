fn main() {
  
    // -- Integer --
    
    let x = 64;
    let x: u32 = 100; //    Length	Signed	Unsigned   <= Integer Types in Rust
                      //    8-bit	    i8  	u8
                      //    16-bit  	i16 	u16
                      //    32-bit	    i32 	u32
                      //    64-bit	    i64 	u64
                      //    128-bit 	i128	u128
                      //    arch	   isize	usize


    // -- Floats -- 
        
    let x = 2.0; // f64 
    let y: f32 = 3.0; // f32
                    
    // -- Numeric Operations --

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
    

    // -- Boolean --
    
    let t = true;

    let f: bool = false; // with explicit type annotation
    
    // -- Char --
    
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // -- Tuple --

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    // -- Array --

    let array = [3; 5]; // [3, 3, 3, 3, 3]
}

