use std::io::Write;
use std::sync::{
    Arc,
    Mutex,
    MutexGuard,
    RwLock,
};
use std::marker::PhantomData;
use std::borrow::Cow;

use itertools::join as iter_join;
use unicode_segmentation::UnicodeSegmentation;
use colored::*;
use core_dev::datetime::datetime::{
    get_current_datetime,
    get_current_time
};
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

use super::GameState;
use crate::MusicState;
use super::AppState;
use crate::{
    TerminalScreen,
    TerminalScreenResult,
    TyperacerErrors
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
    // TODO: mut ref of something its too stupid
    // use Rc<RefCell<TerminalScreen>
    term: &'a mut TerminalScreen,
    // this practice is very useful when dropping stuff
    // mostly for the compiler to know
    _marker: PhantomData<TerminalScreen>,
}

impl<'a> TyperacerUI<'a> {
    pub fn from_term(term: &'a mut TerminalScreen) -> Self {
        let _marker = PhantomData;
        Self {
            term,
            _marker
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
        self.term
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

    // inline is here to get rid of the double-call
    #[inline(always)]
    pub fn color_format_text(
        &self,
        text: &str,
        index: usize,
        wrong_index: usize
    ) -> String {
        // this function is private and i want to test it individually with private tests
        // im doing this because i dont want to instantiante
        // a TyperacerUI every time i want to test this function
        color_format_text(text, index, wrong_index)
    }

    #[inline(always)]
    pub fn set_term_height(
        &mut self,
        height: u16
    ) {
        self.term.set_height(height);
    }

    #[inline(always)]
    pub fn set_term_width(
        &mut self,
        width: u16
    ) {
        self.term.set_width(width);
    }

    #[inline(always)]
    pub fn term_height(&self) -> usize {
        self.term.height()
    }

    #[inline(always)]
    pub fn term_width(&self) -> usize {
        self.term.width()
    }

    /// uses `self.term.print(text, x, y)?`
    pub fn print<T: AsRef<str>>(
        &mut self,
        text: T,
        x: usize,
        y: usize
    ) -> TerminalScreenResult<&mut Self> {
        self.term.print(text.as_ref(), x, y)?;
        Ok(self)
    }

    pub fn flush_stdout(&mut self) -> TerminalScreenResult<&mut Self> {
        self.term.flush_stdout()?;
        Ok(self)
    }

    pub fn draw_progress_bar(
        &mut self,
        index: usize,
        text: &str,
    ) -> TyperacerResult<()> {
        let progress_bar_width = self.term.width() - 2;
        let text_length = text.len();
        let text_length_f32 = text_length as f32;

        let index_f32 = index as f32;

        let progress_percent = 100f32 * (index_f32 / text_length_f32);
        let fill_len = progress_bar_width * index / text_length;

        let progress_bar = format!(
            "{}{}",
            "━".repeat(fill_len).green(),
            "━"
                .repeat(progress_bar_width - fill_len)
                .truecolor(62, 62, 62)
        );

        self.term
            .print(&progress_bar, 9, 1)?;

        //     [Progress]: |███████████████████████_________________| [58.00] [Complete]
        //     [Progress]: |━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━| [58.00] [Complete]

        //     progress_percent = 100 * (index / text.len())
        //
        //
        //     fill_len = int(progress_bar_width * index // text.len())
        //
        //     completed = fill_symbol.repeat(fill_len) + "{remaining_char}" * (progress_bar_width - 4 - fill_len)
        //
        //     if iteration == length:
        //         progressbar = f"[{title}]: |{completed}| [{progress_percent}] [Completed]"
        //     progressbar = ConsoleColored(progressbar, "green")
        //     else:
        //     progressbar = f"[{title}]: |{completed}| [{progress_percent}] [Complete]"
        //     progressbar = ConsoleColored(progressbar, color)
        //
        //     return progressbar
        Ok(())
    }

    pub fn draw(
        &mut self,
        app_state_arc: Arc<Mutex<AppState>>,
    ) -> TyperacerResult<&mut Self> {
        let app_state = match app_state_arc.lock() {
            Ok(app_state) => app_state,
            Err(error) => return Err(TyperacerErrors::PoisonError)
        };

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

        let mut game_state = app_state.game_state_ref_mut();
        let app_state_elapsed = app_state.elapsed_time_ref_mut();
        let mut wpm = app_state.wpm_ref_mut();
        let mut last_wpm = app_state.last_wpm_ref_mut();

        // self.print(format!("{:#?}", wpm), 23, 10)?;

        let wpm = match *wpm {
            Some(wpm) => wpm.to_string().yellow().bold(),
            None => "None".red().bold()
        };

        let last_wpm = match *last_wpm {
            Some(last_wpm) => last_wpm.to_string().yellow().bold(),
            None => "None".red().bold()
        };

        let yellow_left_bracket = "[".yellow();
        let yellow_right_bracket = "]".yellow();
        // let lb = yellow_left_bracket;
        // let rb = yellow_right_bracket;

        let elapsed_time = stopwatch.elapsed().as_secs_f32();

        let game_state_string = match *game_state {
            GameState::Playing => "Playing".green().bold().to_string(),
            GameState::Paused => "Paused".red().bold().to_string()
        };

        let header_x = 0usize;
        // let elapsed_repr = format!("{:.2?}", elapsed_time);
        let current_time = get_current_time();

        #[cfg(feature = "music")]
            let header = {
            let music_state = app_state.music_state_ref_mut();
            let music_state_string = match &*music_state {
                MusicState::Playing => {
                    "Playing".green().bold().to_string()
                }
                MusicState::Paused => "Paused".yellow().bold().to_string(),
                MusicState::Stopped => "Stopped".red().bold().to_string(),
                MusicState::PlaySongNowByAlias(alias) => {
                    format!("Playing: {alias}").green().bold().to_string()
                }
                MusicState::Muted => {
                    "Muted".truecolor(62, 62, 62).bold().to_string()
                }
            };
            format!(
                "{lb}Time: {current_time}{rb} \
                {lb}Elapsed: {elapsed_time:.2?}s{rb} \
                {lb}Game: {game_state_string}{rb} \
                {lb}EAS: {app_state_elapsed:.2}s{rb}\
                \n\
                {lb}WPM: {wpm}{rb} \
                {lb}WPM(last): {last_wpm}{rb} \
                {lb}Music: {music_state_string}{rb}",
                lb = yellow_left_bracket,
                rb = yellow_right_bracket,
                app_state_elapsed = *app_state_elapsed as f32
            )
        };

        #[cfg(not(feature = "music"))]
            let header = format!(
            "{lb}Time: {current_time}{rb} \
                {lb}Elapsed: {elapsed_time:.2?}s{rb} \
                {lb}Game: {game_state_string}{rb} \
                {lb}EAS: {app_state_elapsed:.2}s{rb}\
                \n\
                {lb}WPM: {wpm}{rb}",
            lb = yellow_left_bracket,
            rb = yellow_right_bracket,
            app_state_elapsed = *app_state_elapsed as f32
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

        self.draw_user_input_prompt(
            &user_input_prompt,
            *user_input_prompt_x,
            0,
        )?;

        self.draw_progress_bar(*index, &*typeracer_text)?;

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

        let mut typed_keys = app_state.typed_keys_ref_mut();
        let typed_keys_string = iter_join(
            (*typed_keys).iter(),
            "｜".truecolor(62, 62, 62).bold().to_string().as_str(),
        );

        let typed_keys_string = if typed_keys.len() > 0 {
            format!("... {typed_keys_string}")
        } else {
            typed_keys_string
        };
        let typed_keys_string =
            format!("{}{}", typed_keys_string, " ".repeat(10));
        self.term.print(&typed_keys_string, 22, 0)?;
        // self.term
        //     .rectangle()
        //     .screens_width(true)
        //     .align_center(false)
        //     .xy(22, 0)
        //     .text(typed_keys_string)
        //     .build()?
        //     .draw()?;

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

// this will be the better version
fn _color_format_text(
    text: &str,
    index: usize,
    wrong_index: usize
) -> Cow<'_, str> {
    if index == 0 && wrong_index == 0 {
        Cow::Borrowed(text)
    } else {
        Cow::Owned(text.to_string())
    }
}

// TODO: make this function return Cow<&'a, str>
// if index == 0 && and wrong_index == 0 then just pass through
// else return a new String
fn color_format_text(
    text: &str,
    index: usize,
    wrong_index: usize
) -> String {
    // dont do anything because nothing changed
    if index == 0 && wrong_index == 0 {
        return text.to_string();
    }

    let text =
        UnicodeSegmentation::graphemes(text, true).collect::<Vec<&str>>();

    // "\u{1b}[32mrust_best_asd\nrust_best\nsecond_\u{1b}[0m\u{1b}[31m\u{1b}[0mone long"
    let mut green =
        text[..index].join("").green().to_string().replace(' ', "_");

    let green = if green.contains('\n') {
        let pat = format!("{ENDC}\n{GREEN}");
        green.replace('\n', &pat)
    } else {
        green
    };

    let red = text[index..index + wrong_index]
        .join("")
        .red()
        .to_string()
        .replace(' ', "_");
    let red = if red.contains('\n') {
        let pat = format!("{ENDC}\n{RED}");
        red.replace('\n', &pat)
    } else {
        red
    };

    let rest = text[index + wrong_index..].join("");
    format!("{green}{red}{rest}")
}

#[cfg(test)]
mod tests {
    use assert2::assert;
    use rstest::rstest;

    use super::color_format_text;
    use super::{
        ENDC,
        GREEN,
        RED
    };

    #[rstest]
    #[case("hello world", 0, 0, "hello world")]
    // ERROR: this doesnt work
    // it puts an empty green at the beginning that is not visible
    // because its an ENDC just next to it to delete it
    //   \x1b[32m\x1b[0m\u{1b}[31mh\u{1b}[0mello world
    #[case("hello world", 0, 1, "\x1b[0;31mh\u{1b}[0mello world")]
    fn test_color_format_text(
        #[case] text: &str,
        #[case] index: usize,
        #[case] wrong_index: usize,
        #[case] expected_text: &str
    ) {
        // green "\x1b[0;32m";
        // red "\x1b[0;31m";
        // endc  "\u{1b}[0m";
        let colored_formatted_text =
            color_format_text(text, index, wrong_index);
        assert!(colored_formatted_text == expected_text)
    }
}
