use rand::Rng;
use std::collections::VecDeque;
mod grid;
#[derive(Clone, Debug, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
#[derive(Debug, PartialEq)]
#[must_use]
enum GameResult {
    NoOp,
    GameOver,
}
impl GameResult {
    fn is_over(&self) -> bool {
        if *self == GameResult::GameOver {
            true
        } else {
            false
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
struct SnakeLogic {
    position_snake: VecDeque<(usize, usize)>,
    position_food: (usize, usize),
    direction: Direction,
    height: usize,
    width: usize,
}

impl SnakeLogic {
    pub const MIN_WIDTH: usize = 5;
    pub const MIN_HEIGHT: usize = 5;
    pub const MAX_WIDTH: usize = 150;
    pub const MAX_HEIGHT: usize = 150;
    pub fn new(width: usize, height: usize) -> Option<Self> {
        if width < Self::MIN_WIDTH
            || height < Self::MIN_HEIGHT
            || width > Self::MAX_WIDTH
            || height > Self::MAX_HEIGHT
        {
            return None;
        }

        Some(SnakeLogic {
            position_snake: vec![(0, 0)].into(),
            direction: Direction::None,
            position_food: (1, 1),
            height,
            width,
        })
    }

    pub fn next_step(&mut self, direction: Direction) -> GameResult {
        let head = *self.position_snake.back().unwrap();
        // if self.direction
        self.direction = direction;
        match direction {
            Direction::Right => {
                let new_head = (head.0 + 1, head.1);
                if new_head.0 >= self.width() || self.snake().contains(&new_head) {
                    return GameResult::GameOver;
                }

                self.position_snake.push_back(new_head);

                if new_head == self.food() {
                    self.position_food = self.generate_food();
                } else {
                    self.position_snake.pop_front();
                }

                GameResult::NoOp
            }
            Direction::Up => {
                if head.1 == 0 {
                    return GameResult::GameOver;
                }

                let new_head = (head.0, head.1 - 1);
                if self.snake().contains(&new_head) {
                    return GameResult::GameOver;
                }
                self.position_snake.push_back(new_head);

                if new_head == self.food() {
                    self.position_food = self.generate_food();
                } else {
                    self.position_snake.pop_front();
                }

                GameResult::NoOp
            }
            Direction::Down => {
                let new_head = (head.0, head.1 + 1);
                if new_head.1 >= self.width() || self.snake().contains(&new_head) {
                    return GameResult::GameOver;
                }

                self.position_snake.push_back(new_head);

                if new_head == self.food() {
                    self.position_food = self.generate_food();
                } else {
                    self.position_snake.pop_front();
                }

                GameResult::NoOp
            }
            Direction::Left => {
                if head.0 == 0 {
                    return GameResult::GameOver;
                }

                let new_head = (head.0 - 1, head.1);
                if self.snake().contains(&new_head) {
                    return GameResult::GameOver;
                }
                self.position_snake.push_back(new_head);

                if new_head == self.food() {
                    self.position_food = self.generate_food();
                } else {
                    self.position_snake.pop_front();
                }

                GameResult::NoOp
            }

            Direction::None => GameResult::NoOp,
        }
    }
    pub fn snake(&self) -> &VecDeque<(usize, usize)> {
        &self.position_snake
    }
    pub fn food(&self) -> (usize, usize) {
        self.position_food
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn direction(&self) -> &Direction {
        &self.direction
    }
    fn generate_food(&self) -> (usize, usize) {
        loop {
            let food_x = rand::rng().random_range(0..self.width) as usize;
            let food_y = rand::rng().random_range(0..self.height) as usize;

            if self.snake().contains(&(food_x, food_y)) {
                continue;
            } else {
                return (food_x, food_y);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_step_right() {
        {
            let mut logic = SnakeLogic::new(10, 10).unwrap();
            logic.position_snake = vec![(2, 0), (1, 0), (0, 0), (0, 1)].into();

            assert_eq!(logic.next_step(super::Direction::Right), GameResult::NoOp);
            assert_eq!(*logic.direction(), Direction::Right);
            logic.position_food = (1, 1);
            let x: VecDeque<(usize, usize)> = vec![(2, 0), (1, 0), (0, 0), (0, 1), (1, 1)].into();
            assert_eq!(*logic.snake(), x);
        }
        {
            let mut logic = SnakeLogic::new(10, 10).unwrap();
            logic.position_snake = vec![(2, 3), (2, 2), (2, 1), (2, 0)].into();

            assert_eq!(logic.height(), 10);
            assert_eq!(logic.width(), 10);
            assert_eq!(*logic.direction(), Direction::None);

            let next = logic.next_step(super::Direction::Right);
            assert!(!next.is_over());
            assert_eq!(
                *logic.snake(),
                vec![(2usize, 2usize), (2, 1), (2, 0), (3, 0)]
            );
        }
        {
            // Initial snake state.
            let mut logic = SnakeLogic::new(10, 10).unwrap();
            logic.position_snake = vec![(2, 2), (2, 1), (2, 0), (3, 0)].into();
            assert_eq!(*logic.snake(), vec![(2, 2), (2, 1), (2, 0), (3, 0)]);

            // Snake hits wall.
            logic.position_snake = vec![(1, 3), (1, 2), (1, 1), (10, 10)].into();
            assert!(logic.next_step(super::Direction::Right).is_over());

            // Snake hits self.
            logic.position_snake = vec![(2, 3), (2, 2), (1, 2), (1, 3)].into();
            assert!(logic.next_step(super::Direction::Right).is_over());

            // Normal.
            logic.position_snake = vec![(2, 0), (2, 1), (3, 1), (3, 0)].into();
            assert!(!logic.next_step(super::Direction::Right).is_over())
        }
    }

    #[test]
    fn next_step_up() {
        {
            let mut logic = SnakeLogic::new(10, 10).unwrap();
            logic.position_snake = vec![(0, 2), (1, 2), (2, 2), (2, 1)].into();
            logic.position_food = (2, 0);
            let next = logic.next_step(super::Direction::Up);
            assert_eq!(*logic.direction(), Direction::Up);
            assert!(!next.is_over());
            assert_eq!(*logic.snake(), vec![(0, 2), (1, 2), (2, 2), (2, 1), (2, 0)]);
        }
        {
            let mut logic = SnakeLogic::new(10, 10).unwrap();
            logic.position_snake = vec![(2, 3), (2, 2), (2, 1), (2, 0)].into();

            assert_eq!(logic.height(), 10);
            assert_eq!(logic.width(), 10);
            assert_eq!(*logic.direction(), Direction::None);
            let next = logic.next_step(super::Direction::Up);
            assert!(next.is_over());
        }
        {
            // Initial state.
            let mut logic = SnakeLogic::new(10, 10).unwrap();
            logic.position_snake = vec![(2, 2), (2, 1), (2, 0), (3, 0)].into();
            assert_eq!(*logic.snake(), vec![(2, 2), (2, 1), (2, 0), (3, 0)]);
            // snake hits wall
            logic.position_snake = vec![(1, 3), (1, 2), (1, 1), (1, 0)].into();
            assert!(logic.next_step(super::Direction::Up).is_over());
            // snake hits self.
            logic.position_snake = vec![(2, 3), (2, 2), (1, 2), (1, 3)].into();
            assert!(logic.next_step(super::Direction::Up).is_over());
            // Normal
            logic.position_snake = vec![(0, 2), (1, 2), (2, 2), (2, 1)].into();
            assert!(!logic.next_step(super::Direction::Up).is_over());
        }
    }
    #[test]
    fn next_step_down() {
        {
            let mut logic = SnakeLogic::new(5, 5).unwrap();
            logic.position_snake = vec![(2, 0), (1, 0), (0, 0), (0, 1)].into();

            assert_eq!(logic.next_step(super::Direction::Down), GameResult::NoOp);
            assert_eq!(*logic.direction(), Direction::Down);
            logic.position_food = (1, 1);
            let x: VecDeque<(usize, usize)> = vec![(1, 0), (0, 0), (0, 1), (0, 2)].into();
            assert_eq!(*logic.snake(), x);
            logic.position_food = (0, 3);
            assert!(!logic.next_step(Direction::Down).is_over());
            assert_eq!(*logic.snake(), vec![(1, 0), (0, 0), (0, 1), (0, 2), (0, 3)])
        }
        {
            let mut logic = SnakeLogic::new(5, 5).unwrap();
            logic.position_snake = vec![(2, 3), (2, 2), (2, 1), (2, 0)].into();

            assert_eq!(logic.height(), 5);
            assert_eq!(logic.width(), 5);
            assert_eq!(*logic.direction(), Direction::None);

            let next = logic.next_step(super::Direction::Down);
            assert!(next.is_over());
        }
        {
            // Initial snake state.
            let mut logic = SnakeLogic::new(5, 5).unwrap();
            logic.position_snake = vec![(2, 2), (2, 1), (2, 0), (3, 0)].into();
            assert_eq!(*logic.snake(), vec![(2, 2), (2, 1), (2, 0), (3, 0)]);

            // Snake hits wall.
            logic.position_snake = vec![(2, 5), (3, 5), (4, 5), (5, 5)].into();
            assert!(logic.next_step(super::Direction::Down).is_over());

            // Snake hits self.
            logic.position_snake = vec![(1, 4), (2, 4), (2, 3), (1, 3)].into();
            assert!(logic.next_step(super::Direction::Down).is_over());

            // Normal
            logic.position_snake = vec![(4, 0), (3, 0), (2, 0), (1, 0)].into();
            assert!(!logic.next_step(super::Direction::Down).is_over());
        }
    }

    #[test]
    fn next_step_left() {
        {
            let mut logic = SnakeLogic::new(5, 5).unwrap();
            logic.position_snake = vec![(3, 3), (3, 2), (3, 1), (2, 1)].into();

            assert_eq!(logic.next_step(super::Direction::Left), GameResult::NoOp);
            assert_eq!(*logic.direction(), Direction::Left);
            logic.position_food = (1, 1);
            let x: VecDeque<(usize, usize)> = vec![(3, 3), (3, 2), (3, 1), (2, 1), (1, 1)].into();
            assert_eq!(*logic.snake(), x);
            logic.position_food = (4, 3);
            assert!(!logic.next_step(Direction::Left).is_over());
            assert_eq!(*logic.snake(), vec![(3, 2), (3, 1), (2, 1), (1, 1), (0, 1)])
        }
        {
            let mut logic = SnakeLogic::new(5, 5).unwrap();
            logic.position_snake = vec![(3, 0), (2, 0), (1, 0), (0, 0)].into();

            assert_eq!(logic.height(), 5);
            assert_eq!(logic.width(), 5);
            assert_eq!(*logic.direction(), Direction::None);

            let next = logic.next_step(super::Direction::Left);
            assert!(next.is_over());
        }
        {
            // Initial snake state.
            let mut logic = SnakeLogic::new(5, 5).unwrap();
            logic.position_snake = vec![(2, 2), (2, 1), (2, 0), (3, 0)].into();
            assert_eq!(*logic.snake(), vec![(2, 2), (2, 1), (2, 0), (3, 0)]);

            // Snake hits self.
            logic.position_snake = vec![(2, 5), (3, 5), (4, 5), (5, 5)].into();
            assert!(logic.next_step(super::Direction::Left).is_over());

            // Snake hits wall.
            logic.position_snake = vec![(0, 3), (0, 2), (0, 1), (0, 0)].into();
            assert!(logic.next_step(super::Direction::Left).is_over());

            // Normal
            logic.position_snake = vec![(4, 1), (3, 1), (2, 1), (1, 1)].into();
            assert!(!logic.next_step(super::Direction::Left).is_over());
            assert_eq!(*logic.snake(), vec![(3, 1), (2, 1), (1, 1), (0, 1)])
        }
    }

    #[test]
    fn food_test() {
        let logic = SnakeLogic::new(10, 10).unwrap();
        for _i in 0..50 {
            let food = logic.generate_food();
            assert!(
                !logic.snake().contains(&food),
                "Food {:?} was found in snake: {:?}",
                food,
                logic.snake(),
            );
            assert!(
                food.0 < logic.width(),
                "Food found at row {} but max should be: {}",
                food.0,
                logic.width()
            );
            assert!(
                food.1 < logic.height(),
                "Food found at column {} but max should be: {}",
                food.1,
                logic.height()
            )
        }
    }
}
