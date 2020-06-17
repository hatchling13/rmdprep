mod args;

fn main() {
    let m = args::new();

    let mut input = String::new();

    if m.is_present("INPUT") {
        input = m.value_of("INPUT").unwrap().to_string();
    }

    let files = input.split(',');

    for file in files {
        println!("{}", file);
    }
}
