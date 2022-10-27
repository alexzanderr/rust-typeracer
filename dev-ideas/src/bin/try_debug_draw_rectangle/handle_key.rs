use crate::imports::*;
use crate::LoopActions;

pub fn handle_event(
    term: &mut TerminalScreen,
    event: Event,
    app_state: &AppState
) -> TyperacerResult<((), LoopActions)> {
    // pointers to AppState fields
    let mut keyboard_input = app_state.keyboard_input_ref_mut();
    let mut index = app_state.index_ref_mut();
    let mut wrong_index = app_state.wrong_index_ref_mut();
    let mut typeracer_text = app_state.typeracer_text_ref_mut();
    let mut game_finished = app_state.game_finished_ref_mut();
    let mut user_input_prompt = app_state.user_input_prompt_ref_mut();
    let mut what_was_typed = app_state.what_was_typed_ref_mut();
    let mut user_input_prompt_x = app_state.user_input_prompt_x_ref_mut();
    let term_width = term.width();

    match event {
        Event::FocusGained => {
            todo!("do something if terminal focus is gained")
        },
        Event::FocusLost => {
            todo!("do something if terminal focus is LOST")
        },
        Event::Paste(string_from_ctrl_v) => {},
        Event::Resize(y, x) => {},
        Event::Mouse(mevent) => {
            // dbg!(mevent);
            let mouse_kind = mevent.kind;
            match mouse_kind {
                event::MouseEventKind::Down(event::MouseButton::Right) => {
                },
                _ => {}
            }
        },
        Event::Key(kevent) => {
            let event_clone = format!("{:?}", kevent.code.clone());
            *keyboard_input = event_clone.yellow().to_string();

            match kevent {
                    // clear the entire user_input_bar
                    // and append the text to the text area
                    // enter or space into the user_input_prompt
                    KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {

                        let lines = app_state.typeracer_text_lines_ref_mut();
                        if let Some(lines) = lines {
                        if '\n' == typeracer_text.chars().nth(*index).unwrap()
                            && *wrong_index == 0
                            {
                                // return Ok((self, LoopActions::TimeToContinue))
                            }
                        }
                        // return Ok((self, LoopActions::TimeToContinue));
                        //typeracer logic

                        if '\n' == typeracer_text.chars().nth(*index).unwrap()
                            && *wrong_index == 0
                        {
                            *index += 1;
                            // cursor_x += 1;
                            // // let move_to = cursor::
                            // execute!(
                            //     self.ui.term_buffer_ref_mut(),
                            //     show_cursor,
                            //     cursor_shape,
                            //     cursor_blink_off
                            // )?;
                            if *index == typeracer_text.len() {
                                *game_finished = true;
                            }
                        } else if *index + *wrong_index < typeracer_text.len() {
                            *wrong_index += 1;
                        }

                        // // ui logic
                        let time_to_continue = handle_enter_key(
                            term,
                            &mut what_was_typed,
                            &mut user_input_prompt,
                            user_input_prompt_x.clone())?;

                        if time_to_continue {
                            return Ok(((), LoopActions::TimeToContinue))
                        }
                    },
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: event::KeyModifiers::CONTROL,
                        ..
                    }
                    | KeyEvent {
                        code: KeyCode::Char('d'),
                        modifiers: event::KeyModifiers::CONTROL,
                        ..
                    } => {
                        return Ok(((), LoopActions::QuitGame))
                    },
                    // backspace
                    // delete one char backward
                    KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {

                        // ui logic
                        let _ = user_input_prompt.pop();

                        // logic for the typeracer game
                        if *wrong_index > 0 {
                            *wrong_index -= 1
                        } else {
                            if *index > 0 {
                                *index -= 1;
                            }
                        }
                    },
                    // ctrl + backspace, doesnt work, cuz terminal stuff, i am guessing
                    // but ctrl + h works, cuz linux
                    //
                    // delete the entire word backwards
                    KeyEvent {
                        code: KeyCode::Char('h'),
                        modifiers: event::KeyModifiers::CONTROL,
                        ..
                    }
                    // and also for the same branch alt + backspace
                    | KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: KeyModifiers::ALT,
                        ..
                        // kind: KeyEventKind::Repeat | KeyEventKind::Release,
                        // state: KeyEventState::NONE
                    } => {
                        handle_ctrl_backspace(&mut user_input_prompt)
                    },
                    // user pressed a char key on keyboard
                    // append it to the prompt
                    KeyEvent {
                        code: KeyCode::Char(character),
                        modifiers: event::KeyModifiers::NONE | event::KeyModifiers::SHIFT,
                        ..
                    } => {

                        handle_any_character(term, &mut what_was_typed, &mut user_input_prompt, character);

                        if character == ' ' {
                            what_was_typed.push_str(&user_input_prompt);
                            // what_was_typed.push(' ');
                            if what_was_typed.len() >= term_width - 6 {
                                what_was_typed.clear();
                            }
                            user_input_prompt.clear();
                        }

                        if *index == typeracer_text.len() - 1 {
                            *index += 1;
                            *game_finished = true;
                            return Ok(((), LoopActions::GameFinished))
                        }

                        // typeracer game logic
                        if character == typeracer_text.chars().nth(*index).unwrap()
                            && *wrong_index == 0
                        {
                            *index += 1;
                            if *index == typeracer_text.len() {
                                *game_finished = true;
                            }
                        } else if *index + *wrong_index < typeracer_text.len() {
                            *wrong_index += 1;
                        }
                    },
                    _ => {},
                }
        },
        _ => {}
    }

    Ok(((), LoopActions::LoopDoesntQuit))
}

fn handle_enter_key(
    term: &mut TerminalScreen,
    what_was_typed: &mut String,
    user_input_prompt: &mut String,
    user_input_prompt_x: usize
) -> TyperacerResult<bool> {
    let term_height = term.height();
    let term_width = term.width();

    if user_input_prompt.is_empty() {
        return Ok(true);
    }

    what_was_typed.push_str(&user_input_prompt);
    what_was_typed.push(' ');

    if what_was_typed.len() >= term_width - 6 {
        what_was_typed.clear();
    }
    user_input_prompt.clear();

    Ok(false)
}

fn handle_any_character(
    term: &mut TerminalScreen,
    what_was_typed: &mut String,
    user_input_prompt: &mut String,
    character: char
) {
    user_input_prompt.push(character);
    // 2 from my red cursor _
    // 1 from char `_`
    // 1 from the ansi red color
    if user_input_prompt.len() == term.width() - 11 {
        user_input_prompt.clear();
    }
}

pub fn handle_ctrl_backspace(user_input_prompt: &mut String) {
    // clear the user input prompt
    if let Some(last_space_index) = user_input_prompt.rfind(' ') {
        if let Some(last_char) =
            user_input_prompt.chars().nth(user_input_prompt.len())
        {
            if last_char == ' ' {
                user_input_prompt.remove(user_input_prompt.len() - 1);
            }
        }
        user_input_prompt
            .replace_range(last_space_index..user_input_prompt.len(), '')
    } else {
        user_input_prompt.clear();
    }
}
