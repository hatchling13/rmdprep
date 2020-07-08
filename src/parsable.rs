pub trait Parsable {
    fn parse(&self, line: &str) -> Vec<String> {
        line.trim_start_matches("!").split_whitespace().map(String::from).collect()
    }
}