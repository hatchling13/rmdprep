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
    Execute
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

    let mut content: Vec<(usize, String)> = Vec::new();

    let result = file::read_line(file_name.as_str(), &mut content);

    if result.is_ok() {
        listing(&content);
    } else {
        println!("read_line failed");
    }
}

/*
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

    listing_commands(&c);

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
*/

fn listing(content: &Vec<(usize, String)>) -> bool {
    let re = Regex::new(r"\$\[(\S+) (.+)\]").unwrap();

    let mut matched: Vec<(usize, String)> = Vec::new();
    
    // con.0 : line number, con.1 : line content
    for con in content {
        let line = con.1.as_str();

        if re.is_match(con.1.as_str()) {
            matched.push((con.0, String::from(line)));
        }
    }

    let tokens = ["code", "execute", "youtube"];

    for m in &matched {
        let caps = re.captures(m.1.as_str()).unwrap();

        if !tokens.contains(&caps.get(1).unwrap().as_str()) {
            println!("Invalid command in line {} --> {}", m.0, m.1);

            return false
        }
    }

    for m in &matched {
        // m.0 = line number, m.1 = line content
        let group = re.captures(m.1.as_str()).unwrap();

        // .get(1) = token, .get(2) = args
        let cmd_token = group.get(1).unwrap().as_str();
        let cmd_args = group.get(2).unwrap().as_str();

        let mut token_enum = Token::Code;

        match cmd_token { 
            "code" => token_enum = Token::Code,
            "youtube" => token_enum = Token::Youtube,
            "exec" => token_enum = Token::Execute,
            _ => {}
        }

        if !check_args(token_enum, cmd_args) {
            return false
        }
    }

    true
}

fn check_args(token: Token, args: &str) -> bool {
    let code = ["filename", "lang"];
    let youtube = ["id"];

    let args_splited: Vec<&str> = args.split_whitespace().collect();

    for s in args_splited {
        println!("{:?}", s);
    }

    // compare args with list

    false
}

fn tokenise(re: Regex, lines: &Vec<(usize, String)>) {
    // let commands: Vec<Command> = Vec::new();

    

    /*
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
    */
}

fn content_code(c: &Command) -> String {
    let mut result = String::new();

    let mut arg_map: HashMap<String, String> = HashMap::new();

    hashing_args(c, &mut arg_map);

    let mut codefile = String::new();

    let file_name = arg_map.get("filename").unwrap();

    match file::read_file(file_name, &mut codefile) {
        Ok(size) => println!("read_file for code content succeeded, file size : {}", size),
        Err(e) => println!("read_file for code content failed, {:?}", e.kind())
    }

    // Indicating file name
    result.push_str("> ");
    result.push_str(file_name);

    // language setting
    result.push_str("\n```");
    result.push_str(arg_map.get("lang").map_or("", |lang| lang.as_str()));
    result.push_str("\n");

    // code
    result.push_str(codefile.as_str());
    result.push_str("\n```");
    
    result
}

fn content_execute() {
    
}

fn content_youtube() {
    
}

fn listing_commands(text: &String) {
    let re = Regex::new(r"\$\[(\S+) (.+)\]").unwrap();

    let tokens = ["code", "execute", "youtube"];

    let mut coms: Vec<(&str, &str)> = Vec::new();

    'outer: for mat in re.find_iter(text) {
        let caps = re.captures(mat.as_str()).unwrap();

        let command = caps.get(1).unwrap().as_str();
        let args = caps.get(2).unwrap().as_str();

        for tok in tokens.iter() {
            if tok == &command {
                coms.push((command, args));

                continue 'outer;
            }
        }
    }

    println!("{:?}", coms);
}
/*
fn check_text(text: &String) -> bool {
    let mut result = true;

    let tokens = ["code", "youtube", "execute"];

    let joined = tokens.join("|");

    let mut expression = String::new();

    // set expression for regex
    expression.push_str(r"\$\[(");
    expression.push_str(joined.as_str());
    expression.push_str(r") (.+)\]");

    let re = Regex::new(r"\$\[(\S+) (.+)\]").unwrap();

    let mut line_number = 0;

    // what if $[invalid_command param1=code] ?

    for mat in re.find_iter(text) {
        let caps: Vec<regex::Captures> = re.captures_iter(mat.as_str()).collect();

        for cap in caps {
            println!("{:?}", cap);
        }
    }

    //let caps = re.captures(text).unwrap();

    /*

    let v: Vec<&str> = text.split("\n").collect();

    for line in v {
        line_number += 1;

        // if let? re.match?

        if line.is_empty() { continue; }
        else if !line.starts_with("$[") { continue; }
        else if !line.ends_with("]") { continue; }
        else {

        }
    }

    */

    /*

    'outer: for com in re.find(text) {
        for tok in tokens.iter() {
            if !com.as_str().contains(tok) {
                result = false;
                
                break 'outer;
            }
        }
    }

    */

    result
}
*/

fn hashing_args(c: &Command, arg_map: &mut HashMap<String, String>) {
    let args: Vec<&str> = c.args.split_whitespace().collect();

    for arg in args {
        let mut a: Vec<&str> = arg.split("=").collect();

        let value = String::from(a.pop().unwrap());
        let param = String::from(a.pop().unwrap());

        arg_map.insert(param, value);
    }
}