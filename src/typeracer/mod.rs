mod typeracer;
// self is here because when `cargo test`
// error[E0659]: `typeracer` is ambiguous
// --> /home/alexzander/Alexzander__/programming/rust/projects/typeracer/src/typeracer/mod.rs:2:9
// |
// 2 | pub use typeracer::Typeracer;
// |         ^^^^^^^^^ ambiguous name
// |
// = note: ambiguous because of multiple potential import sources
// = note: `typeracer` could refer to a crate passed with `--extern`
// = help: use `::typeracer` to refer to this crate unambiguously
// note: `typeracer` could also refer to the module defined here
// --> /home/alexzander/Alexzander__/programming/rust/projects/typeracer/src/typeracer/mod.rs:1:1
// |
// 1 | mod typeracer;
// | ^^^^^^^^^^^^^^
// = help: use `self::typeracer` to refer to this module unambiguously
pub use self::typeracer::Typeracer;

mod ui;
pub use ui::TyperacerUI;

mod stats;
pub use stats::Stats;

mod errors;
pub use errors::{
    SpanError,
    TyperacerErrors,
    TyperacerResult
};

mod app_state;
pub use app_state::AppState;

mod game_state;
pub use game_state::GameState;
