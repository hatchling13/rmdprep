mod file;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("error: argument missing");
    }

    else {
        let contents = file::read(args[1].as_str());

        match contents {
            Ok(content) => println!("{}", content),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn _print_help() {
    
}