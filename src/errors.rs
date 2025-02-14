use std::{path::PathBuf, usize};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HakuError {
    #[error("Default recipe not found")]
    DefaultRecipeError,
    #[error("Internal Error{0}")]
    InternalError(String),
    #[error("Invalid feature name '{0}'{1}")]
    InvalidFeatureName(String, String),
    #[error("Failed to open file '{0}': {1}")]
    FileOpenFailure(String, String),
    #[error("Failed to read file '{0}'")]
    FileReadFailure(String),
    #[error("File '{0}' does not exist")]
    FileNotLoaded(usize),
    #[error("File '{0}' does not exist{1}")]
    IncludeNotFound(PathBuf, String),
    #[error("Fail to parse '{0}'{1}")]
    ParseError(String, String),
    #[error("FOR: sequence '{0}' must be integer: {1}")]
    SeqIntError(&'static str, String),
    #[error("FOR: invalid sequence from {0} to {1} step {2}")]
    SeqError(i64, i64, i64),
    #[error("FOR: variable name must not be empty: {0}")]
    SeqVarNameError(String),
    #[error("Include recursion detected: '{0}'")]
    IncludeRecursionError(String),
    #[error("Recipe recursive call detected: '{0}'{1}")]
    RecipeRecursionError(String, String),
    #[error("Recipe '{0}' not found")]
    RecipeNotFoundError(String),
    #[error("Recipe '{0}' is disabled")]
    RecipeDisabledError(String),
    #[error("Failed to execute '{0}': {1}{2}")]
    ExecFailureError(String, String, String),
    #[error("Function call error: '{0}'")]
    FunctionError(String),
    #[error("Include inside a recipe is not supported{0}")]
    IncludeInRecipeError(String),
    #[error("'END' without corresponding IF/WHILE/FOR{0}")]
    StrayEndError(String),
    #[error("Step is 0. For never ends{0}")]
    ForeverForError(String),
    #[error("{0} without matching END{1}")]
    NoMatchingEndError(String, String),
    #[error("No matching FOR or WHILE for END{0}")]
    NoMatchingForWhileError(String),
    #[error("'ELSE' without corresponding IF{0}")]
    StrayElseError(String),
    #[error("'ELSEIF' without corresponding IF{0}")]
    StrayElseIfError(String),
    #[error("Only the last recipe argument can be a list: '{0}'")]
    RecipeListArgError(String),
    #[error("Assertion failed: {0}{1}")]
    AssertError(String, String),
    #[error("Error raised: {0}{1}")]
    UserError(String, String),
    #[error("Invalid directory {0}: {1}")]
    CdError(String, String),
}

impl HakuError {
    /// Generates detailed information about a place where the error happenned.
    pub(crate) fn error_extra(filename: &str, line_no: usize) -> String {
        if !filename.is_empty() {
            format!("\n    at {}:{}", filename, line_no)
        } else {
            format!("\n    at :{}", line_no)
        }
    }
}
