use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use regex::Regex;

#[allow(dead_code)]
enum StaticGen {
    Zola,
    Hugo,
}

fn main() {
    let usage = "Usage: rmdprep <file> [options]\n
    Options:\n
    -h --help\tShow this screen.
    -v --version\tShow version.\n";

    let args: Vec<String> = env::args().collect();

    let mut file_name = String::new();

    if let 1 = args.len() {
        println!("{}", usage);
    } else {
        file_name.push_str(&args[1]);
    }

    let mut text = String::new();

    if !file_name.is_empty(){
        let result = read_file(file_name.as_str(), &mut text);
        
        match result {
            Ok(size) => println!("read_file succeeded, file size: {}", size),
            Err(e) => print!("read_file failed: {:?}", e.kind())
        }
    }
    
    if !text.is_empty() {
        create_post(text);
    }
}

fn read_file(file_name: &str, contents: &mut String) -> std::result::Result<usize, std::io::Error> {
    let file = File::open(file_name);
    
    match file {
        Ok(_) => {
            let mut file_buffer = BufReader::new(file.unwrap());
            let result = file_buffer.read_to_string(contents);
        
            result
        }
        Err(e) => {
            return Err(e);
        }
    }
}

fn create_post(s: String) {
    let v: Vec<&str> = s.split("+++").collect();

    // let front_matter = String::from(v[1].trim());
    let content = String::from(v[2].trim());

    let content_output = create_content(content);

    println!("{}", content_output);
}

fn create_content(c: String) -> String
{
    let result: String = String::from("");
 
    // tokenisation start

    let mut commands: Vec<(&str, &str)> = Vec::new();

    let content = c.as_str();

    let re = Regex::new(r"(\$\[.+\])").unwrap();

    if re.is_match(content) {
        let matched: Vec<regex::Match> = re.find_iter(content).collect();

        for command in matched {
            let x: &[_] = &['[', ']'];

            let a = command.as_str().trim_start_matches("$").trim_matches(x);

            commands.push(a.split_at(a.find(" ").unwrap()));
        }

        println!("{:?}", commands);
    }

    // tokenisation end

    // preparation start
    
    for command in commands {
        
    }
    
    // preparation end

    // substitution start
    // substitution end

    result
}