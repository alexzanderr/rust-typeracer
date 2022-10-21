mod terminal_screen;
pub use terminal_screen::{
    // ScreenPrint,
    TerminalScreen,
    TerminalScreenBuilderError
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
    RectangleBuilderResult
};
