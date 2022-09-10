use std::env;
use std::process::exit;

// Converts binary string to digital
fn binary_to_decimal(input: &str) -> Result<u128, &'static str> {
    // Check if input string too long
    if input.len() > 128 {
        return Err("Input string too long - 128 bits maximum");
    }
    // Number created
    let mut num: u128 = 0;
    // Iterate through members of input string
    for c in input.chars() {
        // Bitshift num -- increases power of two.
        // At start has no effect -- 0 << 1 is still 0.
        num = num << 1;
        // Check if character is 0, 1, or not allowed
        match c {
            '0' => (),
            '1' => num = num + 1,
            _ => return Err("Input string can only contain 0 and 1")
        };
    }
    Ok(num)
}

// Converts binary string to hexadecimal
fn binary_to_hex(input: &str) -> Result<&str, &'static str> {
    // Convert binary to decimal
    let decimal = binary_to_decimal(input);
    // Check result
    match decimal {
        Ok(n) => {
            todo!();
        },
        Err(e) => {
            return Err(e);
        },
    };
    // Convert binary to deci
    // String to build
    let mut String = "";
    return Ok("");
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
    match args[1].as_str() {
        // Binary to decimal
        "-b2d" => { 
            let result = binary_to_decimal(args[2].as_str());
            match result {
                Ok(x) => println!("{}", x),
                Err(e) => println!("{}", e)
            };
        },
        // Binary to hex
        "-b2h" => {
            let result = binary_to_hex(args[2].as_str());
            match result {
                Ok(x) => println!("{}", x),
                Err(e) => println!("{}", e)
            }
        },
        &_ => todo!()
    };
}
