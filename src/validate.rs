use semval::prelude::*;
use std::fs;
use std::path::PathBuf;

//$[code filename=foo.c]

// FileName validation

#[derive(Clone, Debug, Eq, PartialEq)]
struct FileName(String);

impl FileName {
    const fn min_len() -> usize {
        // a.b = 3 chars

        3
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FileNameInvalidity {
    FileNotFound,
    Format,
    MinLength,
}

impl Validate for FileName {
    type Invalidity = FileNameInvalidity;

    fn validate(&self) -> ValidationResult<Self::Invalidity> {
        ValidationContext::new()
            .invalidate_if(
                self.0.len() < Self::min_len(),
                FileNameInvalidity::MinLength,
            )
            .invalidate_if(
                self.0.find(".").is_none(),
                FileNameInvalidity::Format,
            )
            .invalidate_if(
                {
                    let paths = fs::read_dir(".").unwrap().map(|x| x.unwrap().path());

                    let names = paths.map(|x| x.file_name().unwrap().to_str().unwrap());

                    false
                },
                FileNameInvalidity::FileNotFound
            )
            .into()
    }
}

// Code validation

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

    fn validate(&self) -> ValidatedResult<Self::Invalidity> {
        ValidationContext::new()
        .validate_with(&self.name, CodeInvalidity::FileName)
        .invalidate_if(self.name.is_none() || self.lang.is_empty() , CodeInvalidity::Incomplete)
        .into()
    }
}