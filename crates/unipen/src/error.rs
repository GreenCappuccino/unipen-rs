use std::{
    io,
    num::{ParseFloatError, ParseIntError},
};

use thiserror::Error;

use crate::statements::Rule;

#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum UniPenError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("Translation error: {0}\nThis is most likely a bug. Invalid input should be caught by the grammar.")]
    Translation(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error(transparent)]
    ParseFloat(#[from] ParseFloatError),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error(transparent)]
    PestRule(#[from] Box<pest::error::Error<Rule>>),
    #[error("Include path not provided, but file contains .INCLUDE")]
    MissingInclude,
}

macro_rules! translation_err {
    ($msg:expr) => {
        UniPenError::Translation(format!("{}:{}: {}", file!(), line!(), $msg))
    };
}
pub(crate) use translation_err;
