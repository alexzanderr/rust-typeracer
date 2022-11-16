use thiserror::Error as ThisError;


#[derive(Debug, ThisError)]
pub enum TerminalScreenBuilderErrors {
    /// Uninitialized field
    #[error("UninitializedField: {}", .0)]
    UninitializedFieldError(&'static str),
    /// Custom validation error
    #[error("ValidationError: {}", .0)]
    ValidationError(&'static str),
    // #[error("From<&str>: {}", *_str)]
    // Message {
    //     #[from]
    //     _str: &'static str
    // },
    #[error("IoError: {}", io)]
    IoError {
        #[from]
        io: std::io::Error
    },
}

pub type TerminalScreenBuilderResult<T> = std::result::Result<T, TerminalScreenBuilderErrors>;
