mod grid;
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

struct SnakeLogic {
    position_snake: Vec<(usize, usize)>,
    position_food: (usize, usize),
    direction: Direction,
    height: usize,
    width: usize,
}

impl SnakeLogic {
    pub const MIN_WIDTH: usize = 5;
    pub const MIN_HEIGHT: usize = 5;

    pub fn new(width: usize, height: usize) -> Option<Self> {
        if width < Self::MIN_WIDTH || height < Self::MIN_HEIGHT {
            return None;
        }

        Some(SnakeLogic {
            position_snake: vec![(0, 0)],
            direction: Direction::None,
            position_food: (1, 1),
            height,
            width,
        })
    }
}
