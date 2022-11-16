mod terminal_screen;
pub use terminal_screen::{
    // ScreenPrint,
    TerminalScreen,
};

mod errors;
pub use errors::{
    TerminalScreenErrors,
    TerminalScreenResult
};

mod term_lines;
pub use term_lines::TermLines;

mod rectangle;

pub use rectangle::{
    Rectangle,
    RectangleBuilder,
    RectangleBuilderErrors,
    RectangleBuilderResult,
};

mod builder;

pub use builder::{
    TerminalScreenBuilder,
    TerminalScreenBuilderResult,
    TerminalScreenBuilderErrors,
};
