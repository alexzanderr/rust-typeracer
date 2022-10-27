use crate::imports::*;
use crate::LoopActions;

pub fn color_format_text(
    text: &str,
    index: usize,
    wrong_index: usize
) -> String {
    // index + wrong_index
    // {green}rust_best_asd{endc}\n{green}rust_best{endc}\n{green}sec{endc}{red}ond{endc} one long{endc}
    pub const GREEN: &'static str = "\x1b[0;32m";
    pub const RED: &'static str = "\x1b[0;31m";
    let ENDC = "\u{1b}[0m";

    // "\u{1b}[32mrust_best_asd\nrust_best\nsecond_\u{1b}[0m\u{1b}[31m\u{1b}[0mone long"
    let mut green = text[..index].green().to_string().replace(' ', '_');
    let green = if green.contains('\n') {
        let pat = format!("{ENDC}\n{GREEN}");
        green.replace('\n', &pat)
    } else {
        green
    };
    let red = text[index..index + wrong_index]
        .red()
        .to_string()
        .replace(' ', '_');
    let red = if red.contains('\n') {
        let pat = format!("{ENDC}\n{RED}");
        red.replace('\n', &pat)
    } else {
        red
    };
    let rest = &text[index + wrong_index..];
    format!("{green}{red}{rest}")
}

pub fn draw_ui(
    term: &mut TerminalScreen,
    app_state: &AppState
) -> TyperacerResult<()> {
    let stopwatch = app_state.stopwatch_ref();
    let mut keyboard_input = app_state.keyboard_input_ref_mut();
    let mut index = app_state.index_ref_mut();
    let mut wrong_index = app_state.wrong_index_ref_mut();
    let mut typeracer_text = app_state.typeracer_text_ref_mut();
    let mut typeracer_text_x = app_state.typeracer_text_x_ref_mut();
    let mut game_finished = app_state.game_finished_ref_mut();
    let mut user_input_prompt = app_state.user_input_prompt_ref_mut();
    let mut what_was_typed = app_state.what_was_typed_ref_mut();
    let mut user_input_prompt_x = app_state.user_input_prompt_x_ref_mut();

    let typeracer_text_colored =
        color_format_text(&typeracer_text, *index, *wrong_index);

    term.rectangle()
        .screens_width(true)
        .align_center(false)
        .xy(*typeracer_text_x, 0)
        .text(typeracer_text_colored)
        .build()?
        .draw()?;

    let y = (*index + *wrong_index) as u16 + 3;

    {
        let stats = Stats::new(
            typeracer_text.as_str(),
            &keyboard_input,
            *index,
            *wrong_index,
            typeracer_text.len()
        );

        let total_lines_of_stats = 9;
        let x = term.height() - total_lines_of_stats;

        let text = stats.to_string();

        term.rectangle()
            .screens_width(true)
            .align_center(false)
            .text(text)
            .xy(x, 0)
            .build()?
            .draw()?;
    }

    let x = *typeracer_text_x as u16 + 1;
    let move_to = cursor::MoveTo(y, x);
    let show_cursor = cursor::Show;
    let cursor_shape = cursor::SetCursorShape(cursor::CursorShape::Line);
    let cursor_blink_off = cursor::DisableBlinking;

    // if i show the cursor is blinking really fast
    // meaning the cursor is flickering
    execute!(
        term.buffer_ref_mut(),
        move_to,
        show_cursor,
        cursor_shape,
        cursor_blink_off
    )?;

    // write everything to the terminal after
    term.refresh()?;

    Ok(())
}
