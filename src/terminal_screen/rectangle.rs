use thiserror::Error as ThisError;

use super::TerminalScreen;
use super::TerminalScreenResult;
use super::TermLines;

#[derive(Debug)]
pub struct Rectangle<'a, T>
where
    T: TermLines<'a, IteratorItem = &'a str> + 'a
{
    terminal_screen: &'a mut TerminalScreen,
    screens_width:   bool,
    align_center:    bool,
    text:            T,
    x:               u16,
    y:               u16
}

impl<'a, T> Rectangle<'a, T>
where
    T: TermLines<'a, IteratorItem = &'a str> + 'a
{
    pub fn builder(
        terminal_screen: &'a mut TerminalScreen
    ) -> RectangleBuilder<'a, T> {
        RectangleBuilder::new(terminal_screen)
    }

    pub fn draw(&'a mut self) -> TerminalScreenResult<()> {
        self.terminal_screen.draw_rectangle(
            &self.text,
            self.x as usize,
            self.y as usize,
            self.screens_width,
            self.align_center
        )?;

        // error[E0499]: cannot borrow `*self.terminal_screen` as mutable more than once at a time
        //   --> src/terminal_screen/rectangle.rs:44:12
        //    |
        // 23 |   impl<'a, 'b, T> Rectangle<'a, 'b, T>
        //    |            -- lifetime `'b` defined here
        // ...
        // 36 | /         self.terminal_screen.draw_rectangle(
        // 37 | |             &self.text,
        // 38 | |             5,
        // 39 | |             0,
        // 40 | |             self.screens_width,
        // 41 | |             self.align_center
        // 42 | |         )?;
        //    | |_________- first mutable borrow occurs here
        // 43 |
        // 44 |           Ok(self.terminal_screen)
        //    |           ---^^^^^^^^^^^^^^^^^^^^-
        //    |           |  |
        //    |           |  second mutable borrow occurs here
        //    |           returning this value requires that `*self.terminal_screen` is borrowed for `'b`

        // cannot do this
        // Ok(self.terminal_screen)
        Ok(())
    }
}

#[derive(Debug, ThisError)]
pub enum RectangleBuilderErrors {
    #[error("one of the fields is None")]
    FailedToBuildFields
}

pub type RectangleBuilderResult<T> =
    core::result::Result<T, RectangleBuilderErrors>;

#[derive(Debug)]
pub struct RectangleBuilder<'a, T: TermLines<'a> + 'a> {
    terminal_screen: Option<&'a mut TerminalScreen>,
    screens_width:   Option<bool>,
    align_center:    Option<bool>,
    text:            Option<T>,
    x:               Option<u16>,
    y:               Option<u16>
}

impl<'a, T> RectangleBuilder<'a, T>
where
    T: TermLines<'a, IteratorItem = &'a str> + 'a
{
    pub fn new(terminal_screen: &'a mut TerminalScreen) -> Self {
        let terminal_screen = Some(terminal_screen);
        Self {
            terminal_screen,
            screens_width: None,
            align_center: None,
            text: None,
            x: None,
            y: None
        }
    }

    pub fn screens_width(
        mut self,
        screens_width: bool
    ) -> Self {
        self.screens_width = Some(screens_width);
        self
    }

    pub fn align_center(
        mut self,
        align_center: bool
    ) -> Self {
        self.align_center = Some(align_center);
        self
    }

    pub fn text(
        mut self,
        text: T
    ) -> Self {
        self.text = Some(text);
        self
    }

    pub fn xy(
        mut self,
        x: usize,
        y: usize
    ) -> Self {
        self.x = Some(x as u16);
        self.y = Some(y as u16);
        self
    }

    pub fn build(
        &'a mut self
    ) -> RectangleBuilderResult<Rectangle<'a, T>> {
        let terminal_screen = self.terminal_screen.as_mut().unwrap();
        let screens_width = self
            .screens_width
            .clone()
            .ok_or(RectangleBuilderErrors::FailedToBuildFields)?;

        let align_center = self
            .align_center
            .clone()
            .ok_or(RectangleBuilderErrors::FailedToBuildFields)?;

        let text = self
            .text
            .clone()
            .ok_or(RectangleBuilderErrors::FailedToBuildFields)?;

        let x = self
            .x
            .clone()
            .ok_or(RectangleBuilderErrors::FailedToBuildFields)?;

        let y = self
            .y
            .clone()
            .ok_or(RectangleBuilderErrors::FailedToBuildFields)?;

        let rectangle = Rectangle {
            terminal_screen,
            screens_width,
            align_center,
            text,
            x,
            y
        };

        Ok(rectangle)
    }
}
