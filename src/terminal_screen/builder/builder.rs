use std::io::{
    stdout,
    Stdout,
    Write,
};

use super::super::TerminalScreen;
// use crate::TerminalScreen;
use super::errors::{
    TerminalScreenBuilderErrors,
    TerminalScreenBuilderResult,
};

#[derive(Debug)]
pub struct TerminalScreenBuilder {
    /// standard output where everything is written to
    stdout: Option<std::io::Stdout>,
    /// this is the buffer where we write the entire UI
    /// from which all the data will be flushed into stdout
    /// then stdout will be flushed
    buffer: Option<Vec<u8>>,

    /// enter in alternate screen or dont enter
    alternate: Option<bool>,
    /// capture mouse or dont capture mouse
    capture_mouse: Option<bool>,

    /// terminal height aka y from math graph
    height: Option<u16>,
    /// terminal width aka x from math graph
    width: Option<u16>,
}

impl Default for TerminalScreenBuilder {
    fn default() -> Self {
        let stdout = Some(stdout());
        let buffer = Some(Vec::<u8>::new());
        let (width, height) = {
            let (w, h) = crossterm::terminal::size().unwrap();
            let (w, h) = (Some(w), Some(h));
            (w, h)
        };

        TerminalScreenBuilder {
            stdout,
            buffer,
            alternate: None,
            capture_mouse: None,
            width,
            height,
        }
    }
}

impl TerminalScreenBuilder {
    /// Default::default but with `Result<TerminalScreen>` as return type
    /// and the default values which i consider default for TerminalScreen
    pub fn build_default() -> TerminalScreenBuilderResult<TerminalScreen> {
        let stdout = stdout();
        let buffer = Vec::<u8>::new();
        let alternate = true;
        let capture_mouse = false;
        let (width, height) = crossterm::terminal::size()?;

        let terminal_screen = TerminalScreen {
            stdout,
            buffer,
            alternate,
            capture_mouse,
            width,
            height,
        };

        Ok(terminal_screen)
    }

    /// alternate screen or not
    pub fn alternate(
        mut self,
        value: bool,
    ) -> Self {
        self.alternate = Some(value);
        self
    }

    /// capture mouse or dont capture mouse
    pub fn capture_mouse(
        mut self,
        value: bool,
    ) -> Self {
        self.capture_mouse = Some(value);
        self
    }

    // pub fn width(
    //     self,
    //     value: u16
    // ) -> Self {
    //     self.width = Some(value);
    //     self
    // }
    //
    // pub fn height(
    //     mut self,
    //     value: u16
    // ) -> Self {
    //     self.height = Some(value);
    //     self
    // }

    /// Builds a new `TerminalScreen`.
    ///
    /// # Errors
    ///
    /// If a required field has not been initialized.
    pub fn build(
        &mut self
    ) -> TerminalScreenBuilderResult<TerminalScreen> {
        let stdout = match self.stdout {
            Some(_) => {
                stdout()
            },
            None => {
                return Err(TerminalScreenBuilderErrors::UninitializedFieldError("hello world"));
            }
        };


        let buffer = self.buffer.clone().ok_or(TerminalScreenBuilderErrors::UninitializedFieldError("hello world"))?;

        // i dont need builders for these
        // these are automatically detected
        let (width, height) = {
            let w = self.width.clone().ok_or(TerminalScreenBuilderErrors::UninitializedFieldError("hello world"))?;
            let h = self.height.clone().ok_or(TerminalScreenBuilderErrors::UninitializedFieldError("hello world"))?;
            (w, h)
        };

        let alternate = self.alternate.clone().ok_or(TerminalScreenBuilderErrors::UninitializedFieldError("hello world"))?;
        let capture_mouse = self.capture_mouse.clone().ok_or(TerminalScreenBuilderErrors::UninitializedFieldError("hello world"))?;

        let term_screen = TerminalScreen {
            stdout,
            buffer,
            alternate,
            capture_mouse,
            width,
            height,
        };

        Ok(term_screen)
    }

    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn new() -> Self {
        Self {
            stdout: None,
            buffer: None,
            alternate: None,
            capture_mouse: None,
            width: None,
            height: None,
        }
    }
}
