use pancurses::{
    initscr,
    endwin,
    Input,
    noecho,
};

use rand::{
    thread_rng,
    Rng,
};

fn main() {
    let window = initscr();
    window.printw("Type things, press delete to quit\n");
    window.refresh();
    window.keypad(true);
    noecho();
    loop {
        let rn = thread_rng().gen_range(1..1000);
        window.mv(1, 1);
        window.addstr(format!("{}-------------{}", rn, rn));
        window.mv(10, 1);
        match window.getch() {
            Some(Input::Character(c)) => {
                window.addch(c);
            },
            Some(Input::KeyDC) => break,
            Some(input) => {
                window.addstr(&format!("{:?}", input));
            },
            None => (),
        }
    }
    endwin();
}
