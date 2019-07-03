use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[allow(dead_code)]
enum StaticGen {
    Zola,
    Hugo,
}

fn main() {
    /*
    let usage = "
                Usage:\n
                rmdprep <file> [options]
              Options\n
                -h --help\tShow this screen.\n
                -v --version\tShow version.\n
              ";
    */
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    
    let mut text = String::new();
    
    {
        let result = read_file(file_name, &mut text);
        
        match result {
            Ok(size) => println!("read_file succeeded, file size: {}", size),
            Err(e) => print!("read_file failed: {:?}", e.kind()),
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
            println!("cannot open file: {:?}", e.kind());
            return Err(e);
        }
    }
}

fn create_post(s: String) {
    // let mut content = String::new();

    // TODO : Implement
    // Divide text into front-matter and content

    let v: Vec<&str> = s.split("+++").collect();

    println!("Vec size : {}", v.len());

    let front_matter = String::from(v[1].trim());
    let content = String::from(v[2].trim());

    println!("{}", front_matter);
    println!("----------------");
    println!("{}", content);
}