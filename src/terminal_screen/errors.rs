use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum TerminalScreenErrors {
    #[error("failed to print on the screen")]
    IoError(#[from] std::io::Error),
    #[error("faied to load env variable:\n\t{source}")]
    EnvVarError {
        #[from]
        source: std::env::VarError
    }
}

pub type TerminalScreenResult<T> =
    core::result::Result<T, TerminalScreenErrors>;
