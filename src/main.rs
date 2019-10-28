mod file;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("error: argument missing");
    }

    else {
        
    }
}

fn _print_help() {
    
}