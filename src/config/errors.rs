use thiserror::Error as ThisError;
use colored::*;

#[derive(Debug, ThisError)]
pub enum ConfigErrors {
    #[error("SleepMSError: invalid sleep in milliseconds value: {}, must be between [10; 1000]", .0.to_string().yellow().bold())]
    SleepMSError(u16),

    #[error("failed to print on the screen")]
    IoError(#[from] std::io::Error),

    #[error("faied to load env variable:\n\t{source}")]
    EnvVarError {
        #[from]
        source: std::env::VarError
    },

    #[error("TomlError: failed to parse:\n\t{}", .source.to_string().yellow().bold())]
    TomlError {
        #[from]
        source: toml::de::Error
    }
}

pub type ConfigResult<T> = core::result::Result<T, ConfigErrors>;
