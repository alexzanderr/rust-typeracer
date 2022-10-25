use std::time::Instant;
use std::rc::Rc;
use std::cell::{
    Ref,
    RefCell,
    RefMut
};

use getset::{
    CopyGetters,
    Getters,
    MutGetters,
    Setters
};
#[derive(Debug)]
// #[derive(Getters, MutGetters, Debug)]
// #[getset(get = "pub", get_mut = "pub")]
pub struct AppState {
    stopwatch:      RefCell<Instant>,
    typeracer_text: RefCell<String>,

    // #[getset(skip)]
    pub typeracer_text_x: RefCell<usize>,
    what_was_typed:       RefCell<String>,

    // #[getset(skip)]
    pub what_was_typed_x: RefCell<usize>,
    user_input_prompt:    RefCell<String>,

    // #[getset(skip)]
    pub user_input_prompt_x: RefCell<usize>,
    keyboard_input:          RefCell<String>,

    // #[getset(skip)]
    pub index:         RefCell<usize>,
    // #[getset(skip)]
    pub wrong_index:   RefCell<usize>,
    // #[getset(skip)]
    pub game_finished: RefCell<bool>,

    pub cursor_x: RefCell<usize> // #[getset(skip)]
}

impl AppState {
    pub fn stopwatch_ref(&self) -> Ref<'_, Instant> {
        self.stopwatch.borrow()
    }

    pub fn typeracer_text_ref_mut(&self) -> RefMut<'_, String> {
        self.typeracer_text.borrow_mut()
    }

    pub fn typeracer_text_x_ref_mut(&self) -> RefMut<'_, usize> {
        self.typeracer_text_x.borrow_mut()
    }

    pub fn what_was_typed_ref_mut(&self) -> RefMut<'_, String> {
        self.what_was_typed.borrow_mut()
    }

    pub fn keyboard_input_ref_mut(&self) -> RefMut<'_, String> {
        self.keyboard_input.borrow_mut()
    }

    pub fn user_input_prompt_ref_mut(&self) -> RefMut<'_, String> {
        self.user_input_prompt.borrow_mut()
    }

    pub fn index_ref_mut(&self) -> RefMut<'_, usize> {
        self.index.borrow_mut()
    }

    pub fn wrong_index_ref_mut(&self) -> RefMut<'_, usize> {
        self.wrong_index.borrow_mut()
    }

    pub fn game_finished_ref_mut(&self) -> RefMut<'_, bool> {
        self.game_finished.borrow_mut()
    }

    pub fn user_input_prompt_x_ref_mut(&self) -> RefMut<'_, usize> {
        self.user_input_prompt_x.borrow_mut()
    }

    pub fn init() -> Self {
        let typeracer_text_lines = "rust best asd\n\
             rust best\n\
             second one long";
        // let typeracer_text = "asd|";
        // let typeracer_text = "what | is this ?|";
        // let typeracer_text = "what|";
        let stopwatch = std::time::Instant::now();

        // for pair in typeracer_lines.into_iter().enumerate() {
        // let (row_index, line) = pair;
        // game

        // let typeracer_text = line;
        let typeracer_text_x = 6;
        let typeracer_text =
            "rust is the best language ever and the hardest";

        let mut what_was_typed = String::from("");
        let mut what_was_typed_x = 9;

        let user_input_prompt_x = 3;

        let mut user_input_prompt = String::from("");
        let mut total_spaces = 0usize;

        let mut keyboard_input = String::from("");
        // let term_height = self.ui.term_height();
        // let term_width = self.ui.term_width();

        let mut index = 0;
        let mut wrong_index = 0;
        let mut time_to_break = false;
        let mut game_finished = false;
        let mut cursor_x = typeracer_text_x + 1;

        let typeracer_text = typeracer_text.to_string();

        let stopwatch = RefCell::new(stopwatch);
        let typeracer_text = RefCell::new(typeracer_text);
        let typeracer_text_x = RefCell::new(typeracer_text_x);
        let what_was_typed = RefCell::new(what_was_typed);
        let what_was_typed_x = RefCell::new(what_was_typed_x);
        let user_input_prompt = RefCell::new(user_input_prompt);
        let user_input_prompt_x = RefCell::new(user_input_prompt_x);
        let keyboard_input = RefCell::new(keyboard_input);
        let index = RefCell::new(index);
        let wrong_index = RefCell::new(wrong_index);
        let cursor_x = RefCell::new(cursor_x);
        let game_finished = RefCell::new(game_finished);

        Self {
            stopwatch,
            typeracer_text,
            typeracer_text_x,
            what_was_typed,
            what_was_typed_x,
            user_input_prompt,
            user_input_prompt_x,
            keyboard_input,
            index,
            wrong_index,
            cursor_x,
            game_finished
        }
    }
}
