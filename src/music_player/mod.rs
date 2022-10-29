mod music_player;
pub use music_player::MusicPlayer;

mod errors;
pub use errors::{
    MusicPlayerErrors,
    MusicPlayerResult
};

mod music_state;
pub use music_state::MusicState;
