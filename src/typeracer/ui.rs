use std::io::Write;

use colored::*;
use core_dev::datetime::datetime::get_current_datetime;
use crossterm::event::{
    Event,
    KeyCode,
    KeyEvent,
    KeyEventKind,
    KeyEventState,
    KeyModifiers
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

use super::AppState;
use crate::{
    TerminalScreen,
    TerminalScreenResult
};
use super::TyperacerResult;
use super::Stats;
use crate::statics::{
    ENDC,
    GREEN,
    PROMPT_ARROW,
    RED,
    TERMINAL_CURSOR
};

#[derive(Debug)]
pub struct TyperacerUI<'a> {
    term: &'a mut TerminalScreen
}

impl<'a> TyperacerUI<'a> {
    pub fn from_term(term: &'a mut TerminalScreen) -> Self {
        Self {
            term
        }
    }

    pub fn draw_stats(
        &mut self,
        game_stats: Stats
    ) -> TyperacerResult<()> {
        // 6 + 2 (the borders)
        let total_lines_of_stats = 9;
        let x = self.term.height() - total_lines_of_stats;

        let text = game_stats.to_string();

        self.term
            .rectangle()
            .screens_width(true)
            .align_center(false)
            .text(text)
            .xy(x, 0)
            .build()?
            .draw()?;

        Ok(())
    }

    pub fn term_ref_mut(&'a mut self) -> &'a mut TerminalScreen {
        &mut self.term
    }

    pub fn term_buffer_ref_mut(&mut self) -> &mut Vec<u8> {
        self.term.buffer_ref_mut()
    }

    pub fn draw_user_input_prompt(
        &mut self,
        user_input: &str,
        x: usize,
        y: usize
    ) -> TyperacerResult<&mut Self> {
        // let red_cursor = "_".red().bold();
        // let red_cursor = "│".red().bold();
        let prompt_arrow = PROMPT_ARROW.yellow().bold();
        let red_cursor = TERMINAL_CURSOR.red().bold();
        let text = format!("{prompt_arrow}  {user_input}{red_cursor}");
        let text = text.as_str();
        self.term
            .rectangle()
            .text(text)
            .xy(x, y)
            .screens_width(true)
            .align_center(false)
            .build()?
            .draw()?;

        Ok(self)
    }

    pub fn color_format_text(
        &self,
        text: &str,
        index: usize,
        wrong_index: usize
    ) -> String {
        // "\u{1b}[32mrust_best_asd\nrust_best\nsecond_\u{1b}[0m\u{1b}[31m\u{1b}[0mone long"
        let mut green =
            text[..index].green().to_string().replace(" ", "_");
        let green = if green.contains("\n") {
            let pat = format!("{ENDC}\n{GREEN}");
            green.replace("\n", &pat)
        } else {
            green
        };
        let red = text[index..index + wrong_index]
            .red()
            .to_string()
            .replace(" ", "_");
        let red = if red.contains("\n") {
            let pat = format!("{ENDC}\n{RED}");
            red.replace("\n", &pat)
        } else {
            red
        };
        let rest = &text[index + wrong_index..];
        format!("{green}{red}{rest}")
    }

    pub fn term_height(&self) -> usize {
        self.term.height()
    }

    pub fn term_width(&self) -> usize {
        self.term.width()
    }

    /// uses `self.term.print(text, x, y)?`
    pub fn print(
        &mut self,
        text: &str,
        x: usize,
        y: usize
    ) -> TerminalScreenResult<&mut Self> {
        self.term.print(text, x, y)?;
        Ok(self)
    }

    pub fn flush_stdout(&mut self) -> TerminalScreenResult<&mut Self> {
        self.term.flush_stdout()?;
        Ok(self)
    }

    pub fn draw(
        &mut self,
        app_state: &AppState
    ) -> TyperacerResult<&mut Self> {
        let stopwatch = app_state.stopwatch_ref();
        let mut keyboard_input = app_state.keyboard_input_ref_mut();
        let mut index = app_state.index_ref_mut();
        let mut wrong_index = app_state.wrong_index_ref_mut();
        let mut typeracer_text = app_state.typeracer_text_ref_mut();
        let mut typeracer_text_x = app_state.typeracer_text_x_ref_mut();
        let mut game_finished = app_state.game_finished_ref_mut();
        let mut user_input_prompt = app_state.user_input_prompt_ref_mut();
        let mut what_was_typed = app_state.what_was_typed_ref_mut();
        let mut user_input_prompt_x =
            app_state.user_input_prompt_x_ref_mut();

        let yellow_left_bracket = "[".yellow();
        let yellow_right_bracket = "]".yellow();
        let lb = yellow_left_bracket;
        let rb = yellow_right_bracket;

        let elapsed_time = stopwatch.elapsed();

        let header_x = 0usize;
        // let elapsed_repr = format!("{:.2?}", elapsed_time);
        let current_date_time = get_current_datetime();
        let header = format!(
                "{lb}Date-time: {current_date_time}{rb} {lb}Elapsed-time: {elapsed_time:.2?}{rb}",
            );

        self.term
            .rectangle()
            .screens_width(true)
            // TODO: ansi parser algo doesnt work in align_center == true
            .align_center(false)
            .text(header)
            .xy(header_x, 0)
            .build()?
            .draw()?;

        let typeracer_text_colored =
            self.color_format_text(&typeracer_text, *index, *wrong_index);

        self.term
            .rectangle()
            .screens_width(true)
            .align_center(false)
            .xy(*typeracer_text_x, 0)
            .text(typeracer_text_colored)
            .build()?
            .draw()?;

        self.draw_user_input_prompt(
            &user_input_prompt,
            *user_input_prompt_x,
            0
        )?;

        {
            let stats = Stats::new(
                typeracer_text.as_str(),
                &keyboard_input,
                *index,
                *wrong_index,
                typeracer_text.len()
            );
            self.draw_stats(stats)?;
        }

        let cursor_x = app_state.cursor_x_ref_mut();
        let cursor_y = app_state.cursor_y_ref_mut();

        // let y = (*index_shadow + *wrong_index_shadow) as u16 + 3;
        // let x = *typeracer_text_x as u16 + 1;

        let x = *cursor_x as u16;
        
        // 3 is the gap diff between text and ui margins
        let y = *cursor_y as u16 + 3;

        let move_to = cursor::MoveTo(y, x);
        let show_cursor = cursor::Show;
        let cursor_shape =
            cursor::SetCursorShape(cursor::CursorShape::Line);
        let cursor_blink_off = cursor::DisableBlinking;

        // if i show the cursor is blinking really fast
        // meaning the cursor is flickering
        execute!(
            self.term.buffer_ref_mut(),
            move_to,
            show_cursor,
            cursor_shape,
            cursor_blink_off
        )?;

        // write everything to the terminal after
        self.term.refresh()?;

        Ok(self)
    }
}
