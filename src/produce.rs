mod content;
mod file;

impl content::Content for Code {
    fn produce(&self) -> &str {
        let src = file::read(name);
    }
}