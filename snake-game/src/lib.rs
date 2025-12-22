mod game;
pub mod game_with_menu;
mod internal;
mod menu;
mod snakelogic;
pub mod traits;

#[derive(Clone, Debug, Copy, PartialEq)]
/// This enum gives the direction.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    /// This is when you aren't moving
    None,
}
