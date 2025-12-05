pub mod game;
mod internal;
pub mod snakelogic;

#[derive(Clone, Debug, Copy, PartialEq)]
/// This enum gives the direction.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
