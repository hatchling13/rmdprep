use clap::{App, load_yaml};

fn main() {

    let _locale = "en";

    // add macro to choose yml filename decided by locale

    let yaml = load_yaml!("cli.yml");
    let m = App::from(yaml).get_matches();

    if m.is_present("INPUT") {
        println!("{}", m.value_of("INPUT").unwrap());
    }
}
