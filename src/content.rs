pub trait Content {
    fn produce(&self) -> &str;
    fn validate(&self) -> bool;
}