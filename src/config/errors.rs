use thiserror::Error as ThisError;


#[derive(Debug, ThisError)]
pub enum ConfigErrors {
    #[error("failed to print on the screen")]
    IoError(#[from] std::io::Error),

    #[error("faied to load env variable:\n\t{source}")]
    EnvVarError {
        #[from]
        source: std::env::VarError
    },

    #[error("failed to parse something:\n\t{source}")]
    TomlError {
        #[from]
        source: toml::de::Error,
    },
}

pub type ConfigResult<T> = core::result::Result<T, ConfigErrors>;
