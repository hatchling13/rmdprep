use crate::parsable::Parsable;
use semval::prelude::*;

enum Type {
    Cpp,
}

struct FileName(String);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FileNameInvalidity {
    Format,
    NotFound
}

impl Validate for FileName {
    type Invalidity = FileNameInvalidity;

    fn validate(&self) -> ValidationResult<Self::Invalidity> {
        ValidationContext::new()
        .invalidate_if(false, FileNameInvalidity::Format) // TODO : If file name format is not correct
        .invalidate_if(false, FileNameInvalidity::NotFound) // TODO : If file is not found
        .into()
    }
}

// TODO : How to validate custom enum with semval?

struct Code {
    filename: Option<FileName>,
    filetype: Option<Type>
}

impl Parsable for Code {}

