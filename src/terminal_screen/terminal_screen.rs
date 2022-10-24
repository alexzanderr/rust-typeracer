// ╭───────────────────────────╮
// │                           │
// │                           │
// ╰───────────────────────────╯

// ┌───────────────────────────────────────────────────────────────────┐
// │                                                                   │
// │  string = "andrew is here and he wants to tell you something"     │
// │  --->--->--->x = 3↵                                               │
// │  --->--->y = 2↵                                                   │
// │  --->print()↵                                                     │
// │  from core.json__ import *↵                                       │
// │  --->print("hello world")↵                                        │
// │  ↵                                                                │
// │  ↵                                                                │
// │  for i in range(100):↵                                            │
// │  --->x = 3↵                                                       │
// │  --->y = 2↵                                                       │
// │  ↵                                                                │
// │  print("something22222222")↵                                      │
// └───────────────────────────────────────────────────────────────────┘

use std::io::Write;

use ansi_parser::{
    AnsiParser,
    Output
};
use ansi_parser::AnsiSequence;
use thiserror::Error as ThisError;
use pad::{
    Alignment,
    PadStr
};
use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent
};
use crossterm::{
    cursor,
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture
    },
    execute,
    style,
    terminal::{
        self,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
    tty,
    Result as CrosstermResult
};

use super::TerminalScreenResult;
use super::TermLines;
use super::RectangleBuilder;
use super::Rectangle;

#[derive(Debug, derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct TerminalScreen {
    /// Precede your struct (or field) with #[builder(pattern = "owned")] to opt into this pattern. Builders generated with this pattern do not automatically derive Clone, which allows builders to be generated for structs with fields that do not derive Clone.
    #[builder(default = "std::io::stdout()")]
    stdout:        std::io::Stdout,
    #[builder(default = "vec![]")]
    buffer:        Vec<u8>,
    alternate:     bool,
    capture_mouse: bool,
    #[builder(default = "crossterm::terminal::size().unwrap().0")]
    width:         u16,
    #[builder(default = "crossterm::terminal::size().unwrap().1")]
    height:        u16
}

impl<'a> TerminalScreen {
    pub fn new(
        alternate: bool,
        capture_mouse: bool
    ) -> Self {
        let mut stdout = std::io::stdout();
        let (width, height) = crossterm::terminal::size().unwrap();
        let buffer = Vec::<u8>::new();

        Self {
            stdout,
            buffer,
            alternate,
            capture_mouse,
            width,
            height
        }
    }

    /// its working :)
    pub fn set_panic_hook(&self) {
        let alternate = self.alternate;
        let capture_mouse = self.capture_mouse;
        std::panic::set_hook(Box::new(move |panic_info| {
            // if i dont manually bring back the cursor here,
            // the cursor wont come back
            let mut stdout = std::io::stdout();
            let _ = write!(stdout, "{}", crossterm::cursor::Show);

            // ? operator is converting from an error to another
            if alternate {
                let _ = execute!(stdout, LeaveAlternateScreen);
            }
            if capture_mouse {
                let _ = execute!(stdout, DisableMouseCapture);
            }
            let _ = terminal::disable_raw_mode();

            eprintln!();
            eprintln!("program panicked with this:");
            eprintln!("{panic_info:#?}");
        }));
    }

    pub fn builder() -> TerminalScreenBuilder {
        TerminalScreenBuilder::default()
    }

    pub fn enter_raw_terminal(&mut self) -> TerminalScreenResult<()> {
        terminal::enable_raw_mode()?;

        if self.alternate {
            execute!(self.stdout, EnterAlternateScreen)?;
        }
        if self.capture_mouse {
            execute!(self.stdout, EnableMouseCapture)?;
        }
        Ok(())
    }

    pub fn leave_raw_terminal(&mut self) -> TerminalScreenResult<()> {
        // if i dont manually bring back the cursor here,
        // the cursor wont come back
        write!(self.stdout, "{}", crossterm::cursor::Show)?;

        // ? operator is converting from an error to another
        if self.alternate {
            execute!(self.stdout, LeaveAlternateScreen)?;
        }
        if self.capture_mouse {
            execute!(self.stdout, DisableMouseCapture)?;
        }
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn rectangle<L>(&'a mut self) -> RectangleBuilder<'a, L>
    where
        L: TermLines<'a, IteratorItem = &'a str> + 'a
    {
        RectangleBuilder::<'a, L>::new(self)
    }

    pub fn width(&self) -> usize {
        self.width as usize
    }

    pub fn height(&self) -> usize {
        self.height as usize
    }

    pub fn draw_rectangle<
        L: TermLines<'a, IteratorItem = &'a str> + 'a
    >(
        &'a mut self,
        // if you are not putting &'a L
        // error[E0597]: `lines` does not live long enough
        // cuz its moved and then deallocated at the end of the function
        // and im returning data from this function which was part of lines
        lines: &'a L,
        x: usize,
        y: usize,
        screens_width: bool,
        align_center: bool
    ) -> TerminalScreenResult<&'a mut Self> {
        let mut current_x = x;
        let lines = lines.term_lines();
        let mut lines = Vec::from_iter(lines);

        let max_length = if screens_width {
            let (w, h) = crossterm::terminal::size()?;
            w as usize - 6
        } else {
            let mut max_length = 0usize;
            for line in lines.iter() {
                if line.len() > max_length {
                    max_length = line.len();
                }
            }
            max_length
        };

        let inside_length = max_length + 4;

        let header = format!("┌{}┐", "─".repeat(inside_length));

        self.print_buffer(&header, current_x, y)?;
        current_x += 1;

        for line in lines.iter() {
            let aligned_line = if align_center {
                line.pad(inside_length - 4, ' ', Alignment::Middle, true)
            } else {
                // line contains ansi, maybe
                let mut contains_ansi = false;
                let mut total_ansi_length = 0usize;
                let parsed_ansi_string = line.ansi_parse();
                for ansi in parsed_ansi_string {
                    if let Output::Escape(ansi) = ansi {
                        contains_ansi = true;
                        let s = ansi.to_string();
                        total_ansi_length += s.len() - 1;
                    }
                }

                // length is affected is ansi is present in the string
                // the fix is to add total length of just the ansi string to the pad fn
                let pad_length = if contains_ansi {
                    inside_length - 4 + total_ansi_length
                } else {
                    inside_length - 4
                };
                let line =
                    line.pad(pad_length, ' ', Alignment::Left, true);
                line
            };
            let line = format!("│  {aligned_line}  │");
            self.print_buffer(&line, current_x, y)?;
            current_x += 1;

            // rectangle_lines.push(&line.to_owned());
        }

        let footer = format!("└{}┘", "─".repeat(inside_length));

        self.print_buffer(&footer, current_x, y)?;
        current_x += 1;

        // │
        // ─
        // ┌
        // └
        // ┐
        // ┘

        Ok(self)
    }

    pub fn buffer_ref_mut(&mut self) -> &mut Vec<u8> {
        &mut self.buffer
    }

    pub fn clear(&mut self) -> TerminalScreenResult<&mut Self> {
        let clear_screen = crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All
        );
        execute!(self.stdout, clear_screen)?;
        Ok(self)
    }

    pub fn stdout_ref_mut(&mut self) -> &mut std::io::Stdout {
        &mut self.stdout
    }

    pub fn print_buffer(
        &mut self,
        text: &str,
        x: usize,
        y: usize
    ) -> TerminalScreenResult<&mut Self> {
        let x = x as u16;
        let y = y as u16;

        let (move_to_y_x, clear_current_line, hide_cursor) = (
            crossterm::cursor::MoveTo(y, x),
            // crossterm::cursor::MoveTo(cursor::Moveto),
            crossterm::terminal::Clear(
                crossterm::terminal::ClearType::CurrentLine
            ),
            crossterm::cursor::Hide
        );
        // doesnt work with text, it must be a crossterm Command
        // execute!(&mut stdout, move_to, clear, text).unwrap();
        // note that execute! does flush the stdout

        // write! doesnt flush the stdout automatically, except
        // when it contains '\n'
        write!(
            self.buffer,
            "{}{}{}",
            // "{}{}{}",
            move_to_y_x,
            // clear_current_line,
            text,
            // you need to hide the cursor
            // otherwise it will appear alone without any invocations
            hide_cursor
        )?;

        Ok(self)
    }

    #[deprecated = "use term.print_buffer(&mut self) instead"]
    pub fn print(
        &mut self,
        text: &str,
        x: usize,
        y: usize
    ) -> TerminalScreenResult<&mut Self> {
        let x = x as u16;
        let y = y as u16;

        let (move_to_y_x, clear_current_line, hide_cursor) = (
            crossterm::cursor::MoveTo(y, x),
            // crossterm::cursor::MoveTo(cursor::Moveto),
            crossterm::terminal::Clear(
                crossterm::terminal::ClearType::CurrentLine
            ),
            crossterm::cursor::Hide
        );
        // doesnt work with text, it must be a crossterm Command
        // execute!(&mut stdout, move_to, clear, text).unwrap();
        // note that execute! does flush the stdout
        write!(
            self.stdout,
            "{}{}{}",
            // "{}{}{}",
            move_to_y_x,
            clear_current_line,
            text,
            // you need to hide the cursor
            // otherwise it will appear alone without any invocations
            // hide_cursor
        )?;

        // so you can call refresh after print on the same line
        Ok(self)
    }

    #[deprecated = "use term.flush(&mut self) instead"]
    pub fn refresh(&mut self) -> TerminalScreenResult<&mut Self> {
        self.stdout.flush()?;
        Ok(self)
    }

    pub fn flush(&mut self) -> TerminalScreenResult<&mut Self> {
        self.stdout.write_all(self.buffer.as_slice())?;
        self.buffer.clear();
        Ok(self)
    }
}

// #[derive(Debug, derive_builder::Builder)]
// pub struct ScreenPrinter<C: crossterm::cursor::CursorShape> {
//     x:      u16,
//     y:      u16,
//     clear:  crossterm::terminal::Clear,
//     cursor: C
// }

// impl<C> ScreenPrinter<C> {
//     pub fn builder() -> ScreenPrinterBuilder<C> {
//         ScreenPrinterBuilder::default()
//     }

//     pub fn print(
//         &mut self,
//         stdout: &mut std::io::Stdout
//     ) {
//     }
// }
