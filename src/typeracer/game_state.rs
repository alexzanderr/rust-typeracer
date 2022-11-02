/// represents game state
#[derive(Debug, Clone, Copy)]
pub enum GameState {
    /// if `GameState::Paused` then everything is paused
    /// including `music thread` and the `stopwatch thread`
    Paused,
    Playing
}
