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

    let front_matter = String::from(v[1].trim());
    let content = String::from(v[2].trim());

    let content_output = create_content(content);

    // concat front matter with content

    let mut front_output = String::new();

    front_output.push_str("+++\n\n");
    front_output.push_str(front_matter.as_str());
    front_output.push_str("\n+++\n\n");

    let mut final_output = String::new();

    final_output.push_str(front_output.as_str());
    final_output.push_str(content_output.as_str());

    println!("{}", final_output);

    // write file
}

fn create_content(c: String) -> String {
    let mut result: String = String::new();
 
    let mut commands: Vec<Command> = Vec::new();

    tokenise(&c, &mut commands);

    // preparation start

    let mut output: Vec<String> = Vec::new();
    
    for command in commands {
        match command.token {
            Token::Code => output.push(content_code(&command)),
            Token::Execute => content_execute(),
            Token::Youtube => content_youtube(),

            Token::Error => {}
        }
    }
    
    // preparation end

    // substitution start

    result.push_str(c.as_str());

    for out in output {
        let tmp = result.clone();

        let mat = Regex::new(r"(\$\[.+\])").unwrap().find(&tmp).unwrap();

        result.replace_range(mat.start()..mat.end(), out.as_str());
    }

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

fn content_code(c: &Command) -> String {
    let mut result = String::new();

    let mut arg_map: HashMap<String, String> = HashMap::new();

    hashing_args(c, &mut arg_map);

    let mut codefile = String::new();

    match file::read_file(arg_map.get("filename").unwrap(), &mut codefile) {
        Ok(size) => println!("read_file for code content succeeded, file size : {}", size),
        Err(e) => println!("read_file for code content failed, {:?}", e.kind())
    }

    result.push_str("```");
    result.push_str(arg_map.get("lang").unwrap()); // what if "lang" is invalid?
    result.push_str("\n");
    result.push_str(codefile.as_str());
    result.push_str("\n```");

    result
}

fn content_execute() {
    
}

fn content_youtube() {
    
}

fn hashing_args(c: &Command, arg_map: &mut HashMap<String, String>) {
    let args: Vec<&str> = c.args.split_whitespace().collect();

    for arg in args {
        let mut a: Vec<&str> = arg.split("=").collect();

        let value = String::from(a.pop().unwrap());
        let param = String::from(a.pop().unwrap());

        arg_map.insert(param, value);
    }
}