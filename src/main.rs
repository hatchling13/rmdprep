use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        print_help();
    }
    else {
        args.reverse();
        args.pop();
        args.reverse();

        println!("{:?}", args);
    }
}

fn print_help() {

}