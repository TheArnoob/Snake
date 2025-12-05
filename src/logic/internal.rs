#[derive(Debug, PartialEq, Default, Clone)]
#[must_use]
/// This enum tells wether the game is over or not
pub enum GameResult {
    #[default]
    NoOp,
    GameOver,
}

impl GameResult {
    /// Gives a [`bool`] .
    /// If the game is over, it gives [`true`].
    /// If the game is not over, it gives [`false`].
    pub fn is_over(&self) -> bool {
        *self == GameResult::GameOver
    }
}
