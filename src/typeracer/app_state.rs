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

use crate::MusicState;
use crate::GameState;

type rusize = RefCell<usize>;

#[derive(Debug)]
// #[derive(Getters, MutGetters, Debug)]
// #[getset(get = "pub", get_mut = "pub")]
pub struct AppState {
    stopwatch:            RefCell<Instant>,
    typeracer_text:       RefCell<String>,
    typeracer_text_lines: Option<RefCell<Vec<String>>>,

    // #[getset(skip)]
    typeracer_text_x: RefCell<usize>,
    what_was_typed:   RefCell<String>,

    // #[getset(skip)]
    what_was_typed_x:  RefCell<usize>,
    user_input_prompt: RefCell<String>,

    index:               RefCell<usize>,
    // #[getset(skip)]
    user_input_prompt_x: RefCell<usize>,
    keyboard_input:      RefCell<String>,

    // #[getset(skip)]
    // #[getset(skip)]
    wrong_index:   RefCell<usize>,
    // #[getset(skip)]
    game_finished: RefCell<bool>,

    cursor_x: RefCell<usize>,
    cursor_y: RefCell<usize>,

    index_shadow:       RefCell<usize>,
    wrong_index_shadow: RefCell<usize>,
    current_line:       RefCell<usize>,

    music_state: RefCell<MusicState>,

    elapsed_time: RefCell<usize>,
    game_state:   RefCell<GameState>
}

impl AppState {
    pub fn elapsed_time_ref_mut(&self) -> RefMut<'_, usize> {
        self.elapsed_time.borrow_mut()
    }

    pub fn game_state_ref_mut(&self) -> RefMut<'_, GameState> {
        self.game_state.borrow_mut()
    }

    pub fn music_state_ref_mut(&self) -> RefMut<'_, MusicState> {
        self.music_state.borrow_mut()
    }

    pub fn current_line_ref_mut(&self) -> RefMut<'_, usize> {
        self.current_line.borrow_mut()
    }

    pub fn cursor_x_ref_mut(&self) -> RefMut<'_, usize> {
        self.cursor_x.borrow_mut()
    }

    pub fn cursor_y_ref_mut(&self) -> RefMut<'_, usize> {
        self.cursor_y.borrow_mut()
    }

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

    pub fn index_shadow_ref_mut(&self) -> RefMut<'_, usize> {
        self.index_shadow.borrow_mut()
    }

    pub fn wrong_index_shadow_ref_mut(&self) -> RefMut<'_, usize> {
        self.wrong_index_shadow.borrow_mut()
    }

    pub fn typeracer_text_lines_ref_mut(
        &self
    ) -> Option<RefMut<'_, Vec<String>>> {
        // https://rust-lang.github.io/rust-clippy/master/index.html#manual_map
        self.typeracer_text_lines
            .as_ref()
            .map(|typeracer_text_lines| typeracer_text_lines.borrow_mut())
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
        let typeracer_text = "\tRust's is blazingly fast and memory-efficient:
with no runtime or garbage collector,
it can power performance-critical services,
run on embedded devices,
and easily integrate with other languages.

Rust’s rich type system and ownership model
guarantee memory-safety and thread-safety
- enabling you to eliminate
many classes of bugs at compile-time.";

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

        let mut index_shadow = index;
        let mut wrong_index_shadow = wrong_index;
        let mut cursor_y = index_shadow + wrong_index_shadow;

        let typeracer_text_lines = if typeracer_text.contains('\n') {
            let lines = typeracer_text
                .split('\n')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            Some(RefCell::new(lines))
        } else {
            None
        };

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

        let index_shadow = RefCell::new(0usize);
        let wrong_index_shadow = RefCell::new(0usize);
        let cursor_y = RefCell::new(0usize);
        let current_line = RefCell::new(0usize);

        let music_state = RefCell::new(MusicState::new_stopped());

        let elapsed_time = RefCell::new(0usize);
        let game_state = RefCell::new(GameState::Paused);

        Self {
            stopwatch,
            typeracer_text,
            typeracer_text_x,
            typeracer_text_lines,
            what_was_typed,
            what_was_typed_x,
            user_input_prompt,
            user_input_prompt_x,
            keyboard_input,
            index,
            wrong_index,
            cursor_x,
            cursor_y,
            game_finished,
            index_shadow,
            wrong_index_shadow,
            current_line,
            music_state,
            elapsed_time,
            game_state
        }
    }
}
