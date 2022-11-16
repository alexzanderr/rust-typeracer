/// represents game state
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameState {
    /// if `GameState::Paused` then everything is paused
    /// including `music thread` and the `stopwatch thread`
    Paused,
    Playing
}
