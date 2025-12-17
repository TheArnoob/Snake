pub mod game;
pub mod game_with_menu;
mod internal;
pub mod menu;
pub mod snakelogic;
pub mod traits;

#[derive(Clone, Debug, Copy, PartialEq)]
/// This enum gives the direction.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
