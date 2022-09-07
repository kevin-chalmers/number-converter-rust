use std::env;
use std::process::exit;

// Converts binary string to digital
fn binary_to_digital(input: &str) {
    // Check if input string too long
    if input.len() > 128 {
        println!("Input string too long - 128 bits maximum");
        return;
    }
    // Number created
    let mut num: u128 = 0;
    // Iterate through members of input string
    for c in input.chars() {
        num = num << 1;
        match c {
            '0' => (),
            '1' => num = num + 1,
            _ => { println!("Input string can only contain 0 and 1"); exit(-1) }
        }
    }
    println!("{}", num);
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
