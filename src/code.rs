mod content;

struct Code {
    name: &str,
    lang: &str
}

impl content::Content for Code {
    fn validate(args: Vec<&str>) {
        for arg in args {
            let splited: Vec<&str> = arg.split("=").collect();
            
            
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_arg() {

    }

    fn produce_code {

    }
}