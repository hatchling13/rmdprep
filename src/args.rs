use clap::{App, Arg, ArgMatches};

pub fn new() -> ArgMatches {
    App::new("rmdprep")
    .version("0.1.0")
    .author("Jeong Wook Park <hatchling3713@gmail.com>")
    .arg(Arg::new("INPUT").short('i').long("input").value_name("FILE").takes_value(true).required(true).about("Files to input"))
    .get_matches()
}