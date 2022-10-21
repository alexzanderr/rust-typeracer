use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum TerminalScreenErrors {
    #[error("failed to print on the screen")]
    IoError(#[from] std::io::Error)
}

pub type TerminalScreenResult<T> =
    core::result::Result<T, TerminalScreenErrors>;
