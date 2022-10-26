use std::backtrace::Backtrace;

use thiserror::Error as ThisError;
use colored::*;

use crate::terminal_screen::TerminalScreenBuilderError;
use crate::terminal_screen::RectangleBuilderErrors;

#[derive(Debug)]
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

#[derive(Debug)]
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
            span
        }
    }
}

#[derive(Debug, ThisError)]
pub enum TyperacerErrors {
    #[error(
r#"{}: IndexOutOfBounds
    text: "{}"
    index: {}
    text.len(): {}
    span: {}"#,
    "[error]".red().bold(),
    .0.text.yellow().bold().to_string(),
    .0.index.to_string().yellow().bold(),
    .0.text.len().to_string().yellow().bold(),
    .0.span.to_string()
    )]
    IndexOutOfBounds(IndexOutOfBoundsError),

    #[error("BuilderError")]
    BuilderError(#[from] TerminalScreenBuilderError),

    #[error(
r#"{}: IoError
    message: {source}"#,
    "[error]".red().bold()
    )]
    IoError {
        #[from]
        source: std::io::Error
    },

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
