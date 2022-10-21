use thiserror::Error as ThisError;

use crate::terminal_screen::TerminalScreenBuilderError;
use crate::terminal_screen::RectangleBuilderErrors;

#[derive(Debug, ThisError)]
pub enum TyperacerErrors {
    #[error("BuilderError")]
    BuilderError(#[from] TerminalScreenBuilderError),
    #[error("IoError")]
    IoError(#[from] std::io::Error),
    #[error("TerminalScreenError")]
    TerminalScreenError(
        #[from] crate::terminal_screen::TerminalScreenErrors
    ),
    #[error("RectangleBuilderError")]
    RectangleBuilderErrors(
        #[from] crate::terminal_screen::RectangleBuilderErrors
    )
}

pub type TyperacerResult<T> = core::result::Result<T, TyperacerErrors>;
