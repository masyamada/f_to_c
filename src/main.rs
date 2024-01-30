use std::io ;

// Equation to convert Fahrenheit degrees to Celcius
fn f_to_c(x : f64) -> f64 {
    return ( x - 32.0 ) / 1.8 ;
}

fn main() {

// while keep_going is true, convert fahrenheit_in to celcius_out

    let mut keep_going : bool = true ;
    let mut fahrenheit_txt = String::new() ;

    while keep_going == true {

        println!("Hello, human! please tell me what the temperature is (deg F), or type 'quit'") ;

        // read fahrenheit
        fahrenheit_txt.clear() ;

        io::stdin()
            .read_line(&mut fahrenheit_txt)
            .expect("Failed to read line") ;

        // If the input string can be parsed into f64, convert to celcius ; or warn and continue, or exit program nicely.

// This works (but make sure to "let mut fahrenheit_in : f64 ;" and "let mut celcius_out : f64 ;" in the header.
//
//        fahrenheit_in = fahrenheit_txt.trim().parse().unwrap();
//        celcius_out = ( fahrenheit_in - 32.0 ) / 1.8 ;
//
// But I want to exit nicely (instead of panicking then "crashing"). So let's try to use "match" instead of "unwrap()"

//        let check_string = fahrenheit_txt.trim(); // nice (use check_string instead of fahrenheit_txt.trim()), but not necessary
        match fahrenheit_txt.trim().parse::<f64>() {
            Ok(fahrenheit_in) => {
                // println!("You entered a legitimate float: {}", fahrenheit_in);
                // Only convert from F_in to C_out if F_in is in a valid range: (-459.67, HUGE)
                if fahrenheit_in > -459.67 {
                    let celcius_out : f64 =  f_to_c(fahrenheit_in) ;
                    println!("{:.2} Fahrenheit is {:.2} Celcius", fahrenheit_in, celcius_out) ;
                }
                else {
                    println!("Input degrees Fahrenheit is too low: Try Again!") ;
                    continue ;
                }
            }
            Err(..) => {
                if fahrenheit_txt.trim() == "quit" {
                    keep_going = false ;
                    continue ;
                }
                else {
                    println!("{} can not be parsed into a float. Type 'quit' to stop program, or type a number", fahrenheit_txt.trim()) ;
                    continue ;
                }
            }
        } ;

    } // keep_going

}

