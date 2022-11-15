mod config;

pub use config::{
    TyperacerConfig,
    UIConfig,
};

mod errors;

pub use errors::{
    ConfigErrors,
    ConfigResult,
};

mod getters;
