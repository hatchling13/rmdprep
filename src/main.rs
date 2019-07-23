mod file;

use std::env;
use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Command {
    token: Token,
    args: String,
}

#[derive(PartialEq, Debug)]
enum Token {
    Code,
    Youtube,
    Execute,
    Error
}

/*
enum StaticGen {
    Zola,
}
*/

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
        let result = file::read_file(file_name.as_str(), &mut text);
        
        match result {
            Ok(size) => println!("read_file succeeded, file size: {}", size),
            Err(e) => print!("read_file failed: {:?}", e.kind())
        }
    }
    
    if !text.is_empty() {
        create_post(text);
    }
}



fn create_post(s: String) {
    let v: Vec<&str> = s.split("+++").collect();

    // let front_matter = String::from(v[1].trim());
    let content = String::from(v[2].trim());

    let content_output = create_content(content);

    println!("{}", content_output);
}

fn create_content(c: String) -> String {
    let result: String = String::new();
 
    let mut commands: Vec<Command> = Vec::new();

    tokenise(&c, &mut commands);

    // preparation start

    let mut output: Vec<String> = Vec::new();
    
    for command in commands {
        match command.token {
            Token::Code => output.push(content_code(command)),
            Token::Execute => content_execute(),
            Token::Youtube => content_youtube(),

            Token::Error => {}
        }
    }
    
    // preparation end

    // substitution start
    // substitution end

    result
}

fn tokenise(c: &String, commands: &mut Vec<Command>) {
    let content = c.as_str();

    let re = Regex::new(r"(\$\[.+\])").unwrap();

    if re.is_match(content) {
        let matched: Vec<regex::Match> = re.find_iter(content).collect();

        for command in matched {
            let x: &[_] = &['[', ']'];

            let token_enum: Token;

            let command_trimmed = command.as_str().trim_start_matches("$").trim_matches(x);

            let command_tuple = command_trimmed.split_at(command_trimmed.find(" ").unwrap());

            match command_tuple.0 { 
                "code" => token_enum = Token::Code,
                "youtube" => token_enum = Token::Youtube,
                "exec" => token_enum = Token::Execute,
                _ => token_enum = Token::Error
            }

            if token_enum != Token::Error {
                let com: Command = Command { token: token_enum, args: String::from(command_tuple.1.trim_start()) };

                commands.push(com);
            }
        }
    }
}

fn content_code(c: Command) -> String {
    let result = String::new();

    let mut arg_map: HashMap<&str, &str> = HashMap::new();

    hashing_args(c, &mut arg_map);

    println!("{:?}", arg_map);

    result
}

fn content_execute() {
    
}

fn content_youtube() {
    
}

fn hashing_args(c: Command, arg_map: &mut HashMap<&str, &str>) {
    let args: Vec<&str> = c.args.split_whitespace().collect();

    for arg in args {
        let mut a: Vec<&str> = arg.split("=").collect();

        let value = a.pop().unwrap();
        let param = a.pop().unwrap();

        arg_map.insert(param, value);
    }
}