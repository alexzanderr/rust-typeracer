use std::backtrace::Backtrace;
use std::sync::{
    MutexGuard,
    PoisonError
};

use thiserror::Error as ThisError;
use colored::*;
use lazy_static::lazy_static;

use crate::terminal_screen::{
    RectangleBuilderErrors,
    TerminalScreenBuilderErrors
};
use crate::{
    AppState,
    MusicPlayerErrors,
    MusicPlayerResult
};

// thread_local! {
//     pub static RED_ERROR: ColoredString = "[error]".red().bold();
// }

lazy_static! {
    pub static ref RED_ERROR: ColoredString = "[error]".red().bold();
}

#[derive(Debug, Clone)]
pub struct SpanError {
    file:   String,
    line:   u32,
    column: u32
}

impl SpanError {
    pub fn new(
        file: &str,
        line: u32,
        column: u32
    ) -> Self {
        let file = file.to_string();
        Self {
            file,
            line,
            column
        }
    }
}

impl core::fmt::Display for SpanError {
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter<'_>
    ) -> core::fmt::Result {
        let file = self.file.as_str();
        let line = self.line;
        let col = self.column;

        write!(f, "[{file}:{line}:{col}]")
    }
}

#[derive(Debug, Clone)]
pub struct IndexOutOfBoundsError {
    index: usize,
    text:  String,
    span:  SpanError
}

impl IndexOutOfBoundsError {
    pub fn new(
        index: usize,
        text: String,
        span: SpanError
    ) -> Self {
        Self {
            index,
            text,
            span,
        }
    }
}

use crate::ConfigErrors;

#[derive(Debug, ThisError)]
pub enum TyperacerErrors {
    #[error(
    r#"{}: IndexOutOfBounds
    text: "{}"
    index: {}
    text.len(): {}
    span: {}"#,
    RED_ERROR.to_string(),
    .0.text.yellow().bold().to_string(),
    .0.index.to_string().yellow().bold(),
    .0.text.len().to_string().yellow().bold(),
    .0.span.to_string()
    )]
    IndexOutOfBounds(IndexOutOfBoundsError),

    #[error("BuilderError")]
    BuilderError(#[from] TerminalScreenBuilderErrors),

    #[error(
    r#"{}: IoError
    message: {source}
"#,
    RED_ERROR.to_string()
    )]
    IoError {
        #[from]
        source: std::io::Error,
        backtrace: Backtrace
    },

    #[error("TerminalScreenError")]
    TerminalScreenError(
        #[from] crate::terminal_screen::TerminalScreenErrors
    ),

    #[error("RectangleBuilderError")]
    RectangleBuilderErrors(
        #[from] crate::terminal_screen::RectangleBuilderErrors
    ),

    #[error("MusicPlayerError: {:?}", mpe)]
    MusicPlayerError {
        #[from]
        mpe: MusicPlayerErrors
    },

    // TODO: actually implement from a real PoisonError with lifetimes (that was the hard part)
    #[error("PoisonError")]
    PoisonError,

    #[error("ConfigError: {}", ce)]
    ConfigError {
        #[from]
        ce: ConfigErrors
    }
}

pub type TyperacerResult<T> = core::result::Result<T, TyperacerErrors>;
