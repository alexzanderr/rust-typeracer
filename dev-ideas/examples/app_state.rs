#[allow(unused, dead_code)]
use std::time::Instant;
// use getset::{
//     CopyGetters,
//     Getters,
//     MutGetters,
//     Setters
// };
use std::rc::Rc;
use std::cell::{
    RefCell,
    RefMut
};

#[derive(Debug)]
// #[derive(Getters, MutGetters, Debug)]
// #[getset(get = "pub", get_mut = "pub")]
pub struct AppState {
    text:  RefCell<String>,
    index: RefCell<usize>,
}

impl AppState {
    pub fn text_ref_mut(&self) -> RefMut<'_, String> {
        self.text.borrow_mut()
    }

    pub fn index_ref_mut(&self) -> RefMut<'_, usize> {
        self.index.borrow_mut()
    }

    pub fn init() -> Self {
        let text = RefCell::new(String::from("rust"));
        let index = RefCell::new(0);
        Self {
            text,
            index
        }
    }
}

fn second_func(app_state: &AppState) {
    let mut text = app_state.text_ref_mut();
    let mut index = app_state.index_ref_mut();

    {
        *index = 123;
    }
    {
        *text = String::from("asd");
    }
    dbg!(&app_state);
}

fn applogic(app_state: &AppState) {
    dbg!(&app_state);
    second_func(&app_state);
}

fn main() {
    let mut app_state = AppState::init();
    dbg!(&app_state);
    applogic(&app_state);
    dbg!(&app_state);
}
