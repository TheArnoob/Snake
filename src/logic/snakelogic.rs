use std::collections::VecDeque;

use rand::Rng;

use crate::logic::{Direction, internal::GameResult};

const DEFAULT_GROWTH: usize = 2;

#[derive(Debug, PartialEq, Clone)]
/// The overall Snake Game State.
pub struct SnakeLogic {
    /// **This is a vector showing all the squares where the snake is.**
    position_snake: VecDeque<(usize, usize)>,
    position_food: (usize, usize),
    direction: Direction,
    height: usize,
    width: usize,
    /// The amount of growth squares missing to be covered before you walk normally.
    ///
    amount_of_growth: usize,
    growth_per_food: usize,
    /// **A boolean that indicates wether you can change direction or not.
    /// After you change direction, it is false. Default is true**
    can_change_direction: bool,
}

impl SnakeLogic {
    pub const MIN_WIDTH: usize = 5;
    pub const MIN_HEIGHT: usize = 5;
    pub const MAX_WIDTH: usize = 150;
    pub const MAX_HEIGHT: usize = 150;
    /// **This function creates a new instance with the given height and width.
    /// It will return [`None`] if the width or height are out of a specific boundary (MIN or MAX height or width).**
    pub fn new(width: usize, height: usize) -> Option<Self> {
        if width < Self::MIN_WIDTH
            || height < Self::MIN_HEIGHT
            || width > Self::MAX_WIDTH
            || height > Self::MAX_HEIGHT
        {
            return None;
        }
        let position_snake = vec![Self::generate_initial_snake(width, height)].into();
        let position_food = Self::generate_food_inner(width, height, &position_snake);
        Some(SnakeLogic {
            position_snake,
            direction: Direction::None,
            position_food,
            height,
            width,
            amount_of_growth: 0,
            growth_per_food: DEFAULT_GROWTH,
            can_change_direction: true,
        })
    }
    /// **This function changes [`self`] s direction.
    /// It won't change the direction if you let it change to the opposite direction or if you let it change to the same direction it already has.**
    pub fn change_direction(&mut self, direction: Direction) {
        if !self.can_change_direction {
            return;
        }

        match self.direction {
            Direction::Up => match direction {
                Direction::Up => return,
                Direction::Down => return,
                Direction::Left => self.direction = Direction::Left,

                Direction::Right => self.direction = Direction::Right,

                Direction::None => return,
            },
            Direction::Down => match direction {
                Direction::Up => return,
                Direction::Down => return,
                Direction::Left => self.direction = Direction::Left,

                Direction::Right => self.direction = Direction::Right,

                Direction::None => return,
            },
            Direction::Left => match direction {
                Direction::Up => self.direction = Direction::Up,

                Direction::Down => self.direction = Direction::Down,

                Direction::Left => return,
                Direction::Right => return,
                Direction::None => return,
            },
            Direction::Right => match direction {
                Direction::Up => self.direction = Direction::Up,

                Direction::Down => self.direction = Direction::Down,

                Direction::Left => return,
                Direction::Right => return,
                Direction::None => return,
            },
            Direction::None => match direction {
                Direction::Up => self.direction = Direction::Up,

                Direction::Down => self.direction = Direction::Down,

                Direction::Left => self.direction = Direction::Left,

                Direction::Right => self.direction = Direction::Right,

                Direction::None => return,
            },
        }
        self.can_change_direction = false;
    }

    /// **This function moves the snake by the number of steps in the args based on the direction of ** [`self`] ** and returns wether the game is over or not.
    /// It also alters **[`self`] **s snake position and may alter food position.**
    pub fn next_step(&mut self) -> GameResult {
        let head = *self.position_snake.back().unwrap();

        match self.direction {
            Direction::Right => {
                let new_head = (head.0 + 1, head.1);
                if new_head.0 >= self.width() || self.snake().contains(&new_head) {
                    return GameResult::GameOver;
                }
                self.position_snake.push_back(new_head);
                if self.amount_of_growth == 0 {
                    self.position_snake.pop_front();
                } else {
                    self.amount_of_growth -= 1;
                }

                if new_head == self.food() {
                    self.amount_of_growth += DEFAULT_GROWTH;
                    self.position_food = self.generate_food();
                }
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
                if self.amount_of_growth == 0 {
                    self.position_snake.pop_front();
                } else {
                    self.amount_of_growth -= 1;
                }

                if new_head == self.food() {
                    self.amount_of_growth += DEFAULT_GROWTH;
                    self.position_food = self.generate_food();
                }
            }
            Direction::Down => {
                let new_head = (head.0, head.1 + 1);
                if new_head.1 >= self.width() || self.snake().contains(&new_head) {
                    return GameResult::GameOver;
                }
                self.position_snake.push_back(new_head);
                if self.amount_of_growth == 0 {
                    self.position_snake.pop_front();
                } else {
                    self.amount_of_growth -= 1;
                }

                if new_head == self.food() {
                    self.amount_of_growth += DEFAULT_GROWTH;
                    self.position_food = self.generate_food();
                }
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
                if self.amount_of_growth == 0 {
                    self.position_snake.pop_front();
                } else {
                    self.amount_of_growth -= 1;
                }

                if new_head == self.food() {
                    self.amount_of_growth += DEFAULT_GROWTH;
                    self.position_food = self.generate_food();
                }
            }
            Direction::None => (),
        }
        self.can_change_direction = true;
        GameResult::NoOp
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

    #[cfg(test)]
    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    fn random_square(width: usize, height: usize) -> (usize, usize) {
        let square_x = rand::rng().random_range(0..width) as usize;
        let square_y = rand::rng().random_range(0..height) as usize;
        (square_x, square_y)
    }

    /// **This generates the snake in a random place.**
    fn generate_initial_snake(width: usize, height: usize) -> (usize, usize) {
        Self::random_square(width, height)
    }

    /// **This generates the food in a random place.**
    fn generate_food(&self) -> (usize, usize) {
        Self::generate_food_inner(self.width, self.height, self.snake())
    }

    /// **This generates the food in a random place.**
    fn generate_food_inner(
        width: usize,
        height: usize,
        snake: &VecDeque<(usize, usize)>,
    ) -> (usize, usize) {
        loop {
            let (food_x, food_y) = Self::random_square(width, height);
            if snake.contains(&(food_x, food_y)) {
                continue;
            } else {
                return (food_x, food_y);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::Direction;

    use super::*;

    #[test]
    fn next_step_many_steps() {
        {
            let mut logic = SnakeLogic::new(25, 25).unwrap();
            logic.growth_per_food = 2;
            logic.direction = Direction::Right;
            logic.position_snake = vec![(0, 0)].into();
            logic.position_food = (24, 24);
            assert!(!logic.next_step().is_over());
            assert_eq!(*logic.snake(), [(1usize, 0usize)]);
            logic.position_snake = vec![(0, 0)].into();
            logic.position_food = (1, 0);
            assert!(!logic.next_step().is_over());
            assert_eq!(*logic.snake(), [(1usize, 0usize)]);
            assert!(!logic.next_step().is_over());
            assert_eq!(*logic.snake(), [(1, 0), (2, 0)]);
            assert!(!logic.next_step().is_over());
            assert_eq!(*logic.snake(), [(1, 0), (2, 0), (3, 0)]);
            logic.position_food = (24, 24);
            logic.position_snake = [(1, 0), (2, 0), (3, 0)].into();
            let mut logic = SnakeLogic::new(25, 25).unwrap();
            logic.growth_per_food = 2;
            logic.direction = Direction::Right;
            logic.position_snake = [(1, 0), (2, 0), (3, 0)].into();
            assert!(!logic.next_step().is_over());
            assert_eq!(*logic.snake(), [(2, 0), (3, 0), (4, 0)])
        }
        {
            let mut logic = SnakeLogic::new(25, 25).unwrap();
            logic.growth_per_food = 2;

            logic.direction = Direction::Left;
            logic.position_snake = vec![(25, 25)].into();
            assert!(!logic.next_step().is_over());
            assert_eq!(*logic.snake(), [(24, 25)]);
            logic.position_snake = vec![(25, 25)].into();
            logic.position_food = (24, 25);
            assert!(!logic.next_step().is_over());
            assert_eq!(*logic.snake(), [(24, 25)]);
            assert!(!logic.next_step().is_over());
            assert_eq!(*logic.snake(), [(24, 25), (23, 25)]);
            assert!(!logic.next_step().is_over());
            assert_eq!(*logic.snake(), [(24, 25), (23, 25), (22, 25)]);
            assert!(!logic.next_step().is_over());
            assert_eq!(*logic.snake(), [(23, 25), (22, 25), (21, 25)])
        }
        {
            {
                let mut logic = SnakeLogic::new(25, 25).unwrap();
                logic.growth_per_food = 2;

                logic.direction = Direction::Down;
                logic.position_snake = vec![(0, 0)].into();
                assert!(!logic.next_step().is_over());
                assert_eq!(*logic.snake(), [(0, 1)]);
            }
            let mut logic = SnakeLogic::new(25, 25).unwrap();
            logic.position_food = (24, 24);
            logic.direction = Direction::Down;
            logic.position_snake = vec![(0, 0)].into();
            assert!(!logic.next_step().is_over());
            assert_eq!(*logic.snake(), [(0usize, 1usize)]);
            assert!(!logic.next_step().is_over());

            assert_eq!(*logic.snake(), [(0, 2)]);
            logic.position_food = (0, 3);
            assert!(!logic.next_step().is_over());
            assert_eq!(*logic.snake(), [(0, 3)]);

            assert!(!logic.next_step().is_over());

            assert_eq!(*logic.snake(), [(0, 3), (0, 4)]);
            assert!(!logic.next_step().is_over());
            assert_eq!(*logic.snake(), [(0, 3), (0, 4), (0, 5)])
        }

        let mut logic = SnakeLogic::new(25, 25).unwrap();
        logic.growth_per_food = 2;

        logic.direction = Direction::Up;
        logic.position_snake = vec![(25, 25)].into();
        assert!(!logic.next_step().is_over());
        assert_eq!(*logic.snake(), [(25usize, 24usize)]);
        logic.position_snake = vec![(25, 25)].into();
        logic.position_food = (25, 24);
        assert!(!logic.next_step().is_over());
        assert_eq!(*logic.snake(), [(25, 24)]);
        assert!(!logic.next_step().is_over());
        assert_eq!(*logic.snake(), [(25, 24), (25, 23)]);
        assert!(!logic.next_step().is_over());
        assert_eq!(*logic.snake(), [(25, 24), (25, 23), (25, 22)]);
        assert!(!logic.next_step().is_over());
        assert_eq!(*logic.snake(), [(25, 23), (25, 22), (25, 21)])
    }

    #[test]
    fn next_step_right() {
        {
            let mut logic = SnakeLogic::new(10, 10).unwrap();
            logic.growth_per_food = 1;
            logic.position_snake = vec![(2, 0), (1, 0), (0, 0), (0, 1)].into();
            logic.position_food = (1, 1);
            logic.direction = Direction::Right;
            assert_eq!(logic.next_step(), GameResult::NoOp);
            assert_eq!(*logic.direction(), Direction::Right);
            let x: VecDeque<(usize, usize)> = vec![(1, 0), (0, 0), (0, 1), (1, 1)].into();
            assert_eq!(*logic.snake(), x);
        }
        {
            let mut logic = SnakeLogic::new(10, 10).unwrap();
            logic.position_snake = vec![(2, 3), (2, 2), (2, 1), (2, 0)].into();
            logic.growth_per_food = 1;

            assert_eq!(logic.height(), 10);
            assert_eq!(logic.width(), 10);
            assert_eq!(*logic.direction(), Direction::None);
            logic.direction = Direction::Right;

            let next = logic.next_step();
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
            logic.growth_per_food = 1;

            assert_eq!(*logic.snake(), vec![(2, 2), (2, 1), (2, 0), (3, 0)]);

            // Snake hits wall.
            logic.position_snake = vec![(1, 3), (1, 2), (1, 1), (10, 10)].into();
            logic.direction = Direction::Right;

            assert!(logic.next_step().is_over());

            // Snake hits self.
            logic.position_snake = vec![(2, 3), (2, 2), (1, 2), (1, 3)].into();
            logic.direction = Direction::Right;

            assert!(logic.next_step().is_over());

            // Normal.
            logic.position_snake = vec![(2, 0), (2, 1), (3, 1), (3, 0)].into();
            logic.direction = Direction::Right;
            assert!(!logic.next_step().is_over())
        }
    }

    #[test]
    fn next_step_up() {
        {
            let mut logic = SnakeLogic::new(10, 10).unwrap();
            logic.position_snake = vec![(0, 2), (1, 2), (2, 2), (2, 1)].into();
            logic.position_food = (2, 0);
            logic.growth_per_food = 1;

            logic.direction = Direction::Up;
            let next = logic.next_step();
            assert_eq!(*logic.direction(), Direction::Up);
            assert!(!next.is_over());
            assert_eq!(*logic.snake(), vec![(1, 2), (2, 2), (2, 1), (2, 0)]);
        }
        {
            let mut logic = SnakeLogic::new(10, 10).unwrap();
            logic.position_snake = vec![(2, 3), (2, 2), (2, 1), (2, 0)].into();
            logic.growth_per_food = 1;

            assert_eq!(logic.height(), 10);
            assert_eq!(logic.width(), 10);
            assert_eq!(*logic.direction(), Direction::None);
            logic.direction = Direction::Up;
            let next = logic.next_step();
            assert!(next.is_over());
        }
        {
            // Initial state.
            let mut logic = SnakeLogic::new(10, 10).unwrap();
            logic.position_snake = vec![(2, 2), (2, 1), (2, 0), (3, 0)].into();
            logic.growth_per_food = 1;

            assert_eq!(*logic.snake(), vec![(2, 2), (2, 1), (2, 0), (3, 0)]);
            // snake hits wall
            logic.position_snake = vec![(1, 3), (1, 2), (1, 1), (1, 0)].into();
            logic.direction = Direction::Up;

            assert!(logic.next_step().is_over());
            // snake hits self.
            logic.position_snake = vec![(2, 3), (2, 2), (1, 2), (1, 3)].into();
            logic.direction = Direction::Up;

            assert!(logic.next_step().is_over());
            // Normal
            logic.position_snake = vec![(0, 2), (1, 2), (2, 2), (2, 1)].into();
            logic.direction = Direction::Up;

            assert!(!logic.next_step().is_over());
        }
    }

    #[test]
    fn next_step_down() {
        {
            let mut logic = SnakeLogic::new(25, 25).unwrap();
            logic.position_snake = vec![(2, 0), (1, 0), (0, 0), (0, 1)].into();
            logic.growth_per_food = 1;

            logic.direction = Direction::Down;
            logic.position_food = (4, 1);
            assert_eq!(logic.next_step(), GameResult::NoOp);
            assert_eq!(*logic.direction(), Direction::Down);
            assert_eq!(*logic.snake(), [(1, 0), (0, 0), (0, 1), (0, 2)]);
            logic.position_food = (4, 4);
            logic.position_snake = [(0, 0), (0, 1), (0, 2), (0, 3)].into();
            assert!(!logic.next_step().is_over());
            assert_eq!(*logic.snake(), [(0, 1), (0, 2), (0, 3), (0, 4)])
        }
        {
            let mut logic = SnakeLogic::new(5, 5).unwrap();
            logic.growth_per_food = 1;

            logic.position_snake = vec![(2, 3), (2, 2), (2, 1), (2, 0)].into();

            assert_eq!(logic.height(), 5);
            assert_eq!(logic.width(), 5);
            assert_eq!(*logic.direction(), Direction::None);
            logic.direction = Direction::Down;

            let next = logic.next_step();
            assert!(next.is_over());
        }
        {
            // Initial snake state.
            let mut logic = SnakeLogic::new(5, 5).unwrap();
            logic.growth_per_food = 1;

            logic.position_snake = vec![(2, 2), (2, 1), (2, 0), (3, 0)].into();
            assert_eq!(*logic.snake(), vec![(2, 2), (2, 1), (2, 0), (3, 0)]);

            // Snake hits wall.
            logic.position_snake = vec![(2, 5), (3, 5), (4, 5), (5, 5)].into();
            logic.direction = Direction::Down;

            assert!(logic.next_step().is_over());

            // Snake hits self.
            logic.position_snake = vec![(1, 4), (2, 4), (2, 3), (1, 3)].into();
            logic.direction = Direction::Down;

            assert!(logic.next_step().is_over());

            // Normal
            logic.position_snake = vec![(4, 0), (3, 0), (2, 0), (1, 0)].into();
            logic.direction = Direction::Down;

            assert!(!logic.next_step().is_over());
        }
    }

    #[test]
    fn next_step_left() {
        {
            let mut logic = SnakeLogic::new(5, 5).unwrap();
            logic.growth_per_food = 1;

            logic.position_snake = vec![(3, 3), (3, 2), (3, 1), (2, 1)].into();
            logic.direction = Direction::Left;
            logic.position_food = (1, 1);
            assert_eq!(logic.next_step(), GameResult::NoOp);
            assert_eq!(*logic.direction(), Direction::Left);
            let x: VecDeque<(usize, usize)> = vec![(3, 2), (3, 1), (2, 1), (1, 1)].into();
            assert_eq!(*logic.snake(), x);
            logic.position_food = (4, 3);
            logic.direction = Direction::Left;
            assert!(!logic.next_step().is_over());
            assert_eq!(*logic.snake(), vec![(3, 2), (3, 1), (2, 1), (1, 1), (0, 1)])
        }
        {
            let mut logic = SnakeLogic::new(5, 5).unwrap();
            logic.growth_per_food = 1;

            logic.position_snake = vec![(3, 0), (2, 0), (1, 0), (0, 0)].into();

            assert_eq!(logic.height(), 5);
            assert_eq!(logic.width(), 5);
            assert_eq!(*logic.direction(), Direction::None);
            logic.direction = Direction::Left;

            let next = logic.next_step();
            assert!(next.is_over());
        }
        {
            // Initial snake state.
            let mut logic = SnakeLogic::new(5, 5).unwrap();
            logic.growth_per_food = 1;
            logic.position_snake = vec![(2, 2), (2, 1), (2, 0), (3, 0)].into();
            assert_eq!(*logic.snake(), vec![(2, 2), (2, 1), (2, 0), (3, 0)]);

            // Snake hits self.
            logic.position_snake = vec![(2, 5), (3, 5), (4, 5), (5, 5)].into();
            logic.direction = Direction::Left;

            assert!(logic.next_step().is_over());

            // Snake hits wall.
            logic.position_snake = vec![(0, 3), (0, 2), (0, 1), (0, 0)].into();
            logic.direction = Direction::Left;

            assert!(logic.next_step().is_over());

            // Normal
            logic.position_snake = vec![(4, 1), (3, 1), (2, 1), (1, 1)].into();
            logic.direction = Direction::Left;

            assert!(!logic.next_step().is_over());
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

    #[test]
    fn change_direction() {
        // Initializing the logic state.
        let mut logic = SnakeLogic::new(25, 25).unwrap();
        logic.growth_per_food = 1;

        // Checking for initial stop.
        assert_eq!(*logic.direction(), Direction::None);
        // Changing first direction
        logic.change_direction(Direction::Right);
        // Checking this is valid
        assert_eq!(*logic.direction(), Direction::Right);
        // Checking for lock
        assert!(!logic.can_change_direction);
        // Changing in lock (should not work)
        logic.change_direction(Direction::Up);
        // Checking that lock had prevented change
        assert_eq!(*logic.direction(), Direction::Right);
        // Using the GameResult (must_use) and freeing the lock
        logic.position_snake = [(0, 0)].into();
        assert!(!logic.next_step().is_over());
        // Checking that lock is removed
        assert!(logic.can_change_direction);
        // Doing an invalid move
        logic.change_direction(Direction::Left);
        // Checking you cannot go front then back
        assert_eq!(*logic.direction(), Direction::Right);
        // Trying to stop moving
        logic.change_direction(Direction::None);
        // Checking you cannot stop moving
        assert_eq!(*logic.direction(), Direction::Right)
    }
}
