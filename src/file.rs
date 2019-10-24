use std::fs::File;
use std::io::{Read, Result};

pub fn read(file_name: &str) -> Result<String> {
    let mut file = File::open(file_name)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    println!("{}", content);

    Ok(content)
}