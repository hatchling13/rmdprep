use semval::prelude::*;
use std::fs;
use std::path::PathBuf;

// $[code filename=foo.c]

// FileName validation

#[derive(Clone, Debug, Eq, PartialEq)]

struct FileName(String);

impl FileName {
    const fn min_length() -> usize {
        // a.b
        3
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FileNameInvalidity {
    FileNotFound,
    MinLength,
}

impl Validate for FileName {
    type Invalidity = FileNameInvalidity;

    fn validate(&self) -> ValidationResult<Self::Invalidity> {
        ValidationContext::new()
        .invalidate_if({
            let paths: Vec<PathBuf> = fs::read_dir(".").unwrap().map(|x| x.unwrap().path()).collect();
            let mut names = paths.iter().map(|x| x.file_name().unwrap().to_str().unwrap()).filter(|x| x.contains(self.0.as_str())).peekable();
            
            names.next().is_none()

        }, FileNameInvalidity::FileNotFound)
        .invalidate_if(self.0.len() < Self::min_length(), FileNameInvalidity::MinLength)
        .into()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Code {
    name: Option<FileName>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum CodeInvalidity {
    FileName(FileNameInvalidity),
    Incomplete,
}

impl Validate for Code {
    type Invalidity = CodeInvalidity;

    fn validate(&self) -> ValidationResult<Self::Invalidity> {
        ValidationContext::new()
        .validate_with(&self.name, CodeInvalidity::FileName)
        .invalidate_if(self.name.is_none(), CodeInvalidity::Incomplete)
        .into()
    }
}