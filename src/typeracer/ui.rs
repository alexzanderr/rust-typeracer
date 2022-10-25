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
    PROMPT_ARROW,
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
        let green = text[..index].green().to_string().replace(" ", "_");
        let red = text[index..index + wrong_index]
            .red()
            .to_string()
            .replace(" ", "_");
        let rest = &text[index + wrong_index..];
        // let cursor = "r𑗅ust".red();
        // format!("'𑗅asd'")
        // the cursor only looks like this inside sublime text
        format!("{green}{red}{rest}")
    }

    pub fn term_height(&self) -> usize {
        self.term.height()
    }

    pub fn term_width(&self) -> usize {
        self.term.width()
    }

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

    pub fn draw_from_app_state(
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

        let y = (*index + *wrong_index) as u16 + 3;

        let x = *typeracer_text_x as u16 + 1;
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

    pub fn draw(
        &mut self,
        current_time: std::time::Instant,
        typeracer_text: &str,
        typeracer_text_x: usize,
        what_was_typed: &str,
        what_was_typed_x: usize,
        user_input_prompt: &mut String,
        user_input_prompt_x: usize,
        keyboard_input: &mut String,
        index: usize,
        wrong_index: usize,
        cursor_x: usize
    ) -> TyperacerResult<&mut Self> {
        let yellow_left_bracket = "[".yellow();
        let yellow_right_bracket = "]".yellow();
        let lb = yellow_left_bracket;
        let rb = yellow_right_bracket;

        let elapsed_time = current_time.elapsed();

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
            self.color_format_text(typeracer_text, index, wrong_index);
        // let typeracer_text_colored = format!(
        //     "{} {} {}",
        //     "hello".green(),
        //     "wrong".red(),
        //     "nortmrl teast"
        // );
        // let indexes = (0..typeracer_text.len())
        //     .into_iter()
        //     .map(|index| index.to_string())
        //     .collect::<String>();

        // let typeracer_text_colored =
        // format!("{typeracer_text_colored}\n{indexes}");
        self.term
            .rectangle()
            .screens_width(true)
            .align_center(false)
            .xy(typeracer_text_x, 0)
            .text(typeracer_text_colored)
            .build()?
            .draw()?;

        // self.term
        //     .rectangle()
        //     .screens_width(true)
        //     .align_center(false)
        //     .xy(what_was_typed_x, 0)
        //     .text(what_was_typed)
        //     .build()?
        //     .draw()?;

        // if !text_area.is_empty() {
        //     self.term
        //         .rectangle()
        //         .screens_width(true)
        //         .align_center(false)
        //         .xy(text_area_x, 0)
        //         .text(text_area.as_slice())
        //         .build()?
        //         .draw()?;
        //     self.term.refresh()?;
        // }

        self.draw_user_input_prompt(
            &user_input_prompt,
            user_input_prompt_x,
            0
        )?;

        {
            let stats = Stats::new(
                typeracer_text,
                &keyboard_input,
                index,
                wrong_index,
                typeracer_text.len()
            );
            self.draw_stats(stats)?;
        }

        let y = (index + wrong_index) as u16 + 3;

        let index_shadow = index;
        let wrong_index_shadow = wrong_index;
        // let cursor_x = false;
        // let cursor_y = false;

        let move_to = cursor::MoveTo(y, cursor_x as u16);
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
