mod args;
mod code;
mod parsable;

fn main() {
    let m = args::new();

    let mut input = String::new();

    if m.is_present("INPUT") {
        input = m.value_of("INPUT").unwrap().to_string();
    }

    // Can receive multiple filename at once
    let names = input.split(',');

    for name in names {
        println!("{}", name);
    }

    
}
