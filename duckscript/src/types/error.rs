//! # error
//!
//! The error structure and types.
//!

use crate::types::instruction::InstructionMetaInfo;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[cfg(test)]
#[path = "./error_test.rs"]
mod error_test;

fn format_error_message(
    formatter: &mut fmt::Formatter,
    meta_info: &InstructionMetaInfo,
    message: &str,
) -> Result<(), fmt::Error> {
    let source = match meta_info.source {
        Some(ref value) => value.to_string(),
        None => "Unknown".to_string(),
    };
    let line = match meta_info.line {
        Some(value) => value.to_string(),
        None => "Unknown".to_string(),
    };

    write!(formatter, "Source: {} Line: {} - {}", source, line, message)
}

#[derive(Debug)]
/// Holds the error information
pub enum ScriptError {
    /// Error Info Type
    Initialization(String),
    /// Error Info Type
    Runtime(String, Option<InstructionMetaInfo>),
    /// Error Info Type
    PreProcessNoCommandFound(InstructionMetaInfo),
    /// Error Info Type
    ControlWithoutValidValue(InstructionMetaInfo),
    /// Error Info Type
    InvalidControlLocation(InstructionMetaInfo),
    /// Error Info Type
    MissingEndQuotes(InstructionMetaInfo),
    /// Error Info Type
    MissingOutputVariableName(InstructionMetaInfo),
    /// Error Info Type
    InvalidEqualsLocation(InstructionMetaInfo),
    /// Error Info Type
    InvalidQuotesLocation(InstructionMetaInfo),
    /// Error Info Type
    EmptyLabel(InstructionMetaInfo),
    /// IO Error
    IoError(std::io::Error),
}

impl Display for ScriptError {
    /// Formats the script error using the given formatter.
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Initialization(ref message) => write!(formatter, "{}", message),
            Self::Runtime(ref message, ref meta_info) => {
                let empty_meta_data = InstructionMetaInfo::new();
                let meta_info_value = meta_info.as_ref().unwrap_or(&empty_meta_data);
                format_error_message(formatter, &meta_info_value, message)
            }
            Self::PreProcessNoCommandFound(ref meta_info) => {
                format_error_message(formatter, &meta_info, "preprocessor is missing a command")
            }
            Self::ControlWithoutValidValue(ref meta_info) => format_error_message(
                formatter,
                &meta_info,
                "control character found without a valid value",
            ),
            Self::InvalidControlLocation(ref meta_info) => {
                format_error_message(formatter, &meta_info, "invalid control character location")
            }
            Self::MissingEndQuotes(ref meta_info) => {
                format_error_message(formatter, &meta_info, "missing end quotes")
            }
            Self::MissingOutputVariableName(ref meta_info) => {
                format_error_message(formatter, &meta_info, "missing variable name")
            }
            Self::InvalidEqualsLocation(ref meta_info) => {
                format_error_message(formatter, &meta_info, "invalid equals sign location")
            }
            Self::InvalidQuotesLocation(ref meta_info) => {
                format_error_message(formatter, &meta_info, "invalid quotes location")
            }
            Self::EmptyLabel(ref meta_info) => {
                format_error_message(formatter, &meta_info, "empty label found")
            }
            Self::IoError(ref error) => write!(formatter, "IO Error: {error}"),
        }
    }
}
impl From<std::io::Error> for ScriptError {
    fn from(value: std::io::Error) -> Self {
        ScriptError::IoError(value)
    }
}

impl Error for ScriptError {}
