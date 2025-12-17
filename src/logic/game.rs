use crate::Direction;
use crate::logic::internal::GameResult;
use crate::logic::snakelogic::SnakeLogic;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
#[derive(Debug)]
pub struct SnakeGame {
    snake_logic: SnakeLogic,
    now: Instant,
    paused: bool,
    last_logic_update: Instant,
    last_game_result: GameResult,
}

impl SnakeGame {
    pub const TIMESTEP: Duration = Duration::from_millis(100);

    pub fn new(width: usize, height: usize) -> SnakeGame {
        let snake_logic = SnakeLogic::new(width, height).expect("Cannot fail");
        let now = Instant::now();

        SnakeGame {
            snake_logic,
            now,
            paused: false,
            last_logic_update: Instant::now(),
            last_game_result: GameResult::NoOp,
        }
    }

    pub fn height(&self) -> usize {
        self.snake_logic.height()
    }

    pub fn width(&self) -> usize {
        self.snake_logic.width()
    }

    pub fn food(&self) -> (usize, usize) {
        self.snake_logic.food()
    }

    pub fn snake(&self) -> &VecDeque<(usize, usize)> {
        self.snake_logic.snake()
    }

    pub fn direction(&self) -> Direction {
        self.snake_logic.direction()
    }
    pub fn change_direction(&mut self, direction: Direction) {
        self.snake_logic.change_direction(direction)
    }

    pub fn set_paused(&mut self, new_paused: bool) {
        self.paused = new_paused;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn score(&self) -> usize {
        self.snake().len()
    }

    pub fn is_over(&self) -> bool {
        self.last_game_result == GameResult::GameOver
    }
    pub fn update(&mut self, now: std::time::Instant) {
        if self.last_game_result.is_over() {
            return;
        }
        self.now = now;
        if now - self.last_logic_update > Self::TIMESTEP {
            if !self.is_paused() {
                self.last_game_result = self.snake_logic.next_step();
            }
            self.last_logic_update = now;
        }

        if self.snake().len() == (self.height() * self.width()) {
            println!("You Win! Great job!")
        }
    }
}
