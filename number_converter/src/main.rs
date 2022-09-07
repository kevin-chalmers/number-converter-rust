use std::env;
use std::process::exit;

// Converts binary string to digital
fn binary_to_digital(input: &str) {
    println!("In binary to digital");
}


fn main() {
    // Collect the command line arguments
    let args: Vec<String> = env::args().collect();
    // Check right number of arguments. Should have:
    // -- the conversion type
    // -- the input
    if args.len() != 3 {
        // Just print and exit for now
        println!("Application takes two arguments");
        exit(-1);
    }
    // Check second parameter for conversion type.
    // For now, just binary to digital.
    match args[1].as_str() {
        "-b2d" => binary_to_digital(args[2].as_str()),
        &_ => todo!()
    };
}
