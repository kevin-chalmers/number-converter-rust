use std::env;

fn main() {
    // Collect the command line arguments
    let args: Vec<String> = env::args().collect();
    // Iterate through the command line arguments
    for arg in args.iter() {
        println!("{}", arg);
    }
}
