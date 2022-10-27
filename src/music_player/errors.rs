use thiserror::Error as ThisError;
use soloud::SoloudError;

#[derive(Debug, ThisError)]
pub enum MusicPlayerErrors {
    #[error("failed to print on the screen")]
    IoError(#[from] std::io::Error),

    #[error("faied to load env variable:\n\t{source}")]
    EnvVarError {
        #[from]
        source: std::env::VarError
    },

    #[error("")]
    SoloudError {
        #[from]
        source: SoloudError
    },

    #[error("music player already initialized error")]
    MusicPLayerAlreadyInitializedError
}

pub type MusicPlayerResult<T> = core::result::Result<T, MusicPlayerErrors>;
