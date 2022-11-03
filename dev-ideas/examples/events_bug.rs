use crossterm::{
    event::read,
    event::{
        Event,
        KeyCode,
        KeyEvent,
        KeyModifiers
    },
    execute,
    terminal::{
        self,
        Clear,
        ClearType
    }
};
// Describe the bug
// Ctrl + Enter = modifiers::NONE
// Shift + Enter = modifiers::NONE

// Alt + Enter = modifiers::ALT
fn main() {
    let mut stdout = std::io::stdout();
    loop {
        terminal::enable_raw_mode();
        let res = read().unwrap();
        if let Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) = res
        {
            break;
        }
        terminal::disable_raw_mode();
        println!("{:#?}", res);
    }
    terminal::disable_raw_mode();
}
