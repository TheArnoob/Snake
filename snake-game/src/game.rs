use crate::Direction;
use crate::internal::GameResult;
use crate::snakelogic::SnakeLogic;
use std::collections::VecDeque;

use web_time::{Duration, Instant};

#[derive(Debug)]
pub struct SnakeGame {
    snake_logic: SnakeLogic,
    now: Instant,
    paused: bool,
    last_logic_update: Instant,
    last_game_result: GameResult,
    timestep: Duration,
}

impl Default for SnakeGame {
    fn default() -> Self {
        Self {
            snake_logic: SnakeLogic::new(25, 25).expect("Cannot fail"),
            now: Instant::now(),
            paused: Default::default(),
            last_logic_update: Instant::now(),
            last_game_result: Default::default(),
            timestep: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum GameDifficulty {
    Hard,
    Normal,
    Easy,
    Basic,
    VeryHard,
    Extreme,
    Intermediate,
    Expert,
    Insane,
    VeryEasy,
}

impl GameDifficulty {
    pub const TIMESTEP_NORMAL: Duration = Duration::from_millis(100);
    pub const TIMESTEP_EASY: Duration = Duration::from_millis(200);
    pub const TIMESTEP_HARD: Duration = Duration::from_millis(75);
    pub const TIMESTEP_EXTREME: Duration = Duration::from_millis(35);
    pub const TIMESTEP_INSANE: Duration = Duration::from_millis(20);
    pub const TIMESTEP_VERYEASY: Duration = Duration::from_millis(400);
    pub const TIMESTEP_EXPERT: Duration = Duration::from_millis(10);
    pub const TIMESTEP_INTERMEDIATE: Duration = Duration::from_millis(60);
    pub const TIMESTEP_BASIC: Duration = Duration::from_millis(150);
    pub const TIMESTEP_VERYHARD: Duration = Duration::from_millis(30);

    pub fn timestep(&self) -> Duration {
        match self {
            GameDifficulty::Hard => Self::TIMESTEP_HARD,
            GameDifficulty::Normal => Self::TIMESTEP_NORMAL,
            GameDifficulty::Easy => Self::TIMESTEP_EASY,
            GameDifficulty::Extreme => Self::TIMESTEP_EXTREME,
            GameDifficulty::Insane => Self::TIMESTEP_INSANE,
            GameDifficulty::VeryEasy => Self::TIMESTEP_VERYEASY,
            GameDifficulty::Intermediate => Self::TIMESTEP_INTERMEDIATE,
            GameDifficulty::Expert => Self::TIMESTEP_EXPERT,
            GameDifficulty::Basic => Self::TIMESTEP_BASIC,
            GameDifficulty::VeryHard => Self::TIMESTEP_VERYHARD,
        }
    }

    pub fn game_size(&self) -> (usize, usize) {
        match self {
            GameDifficulty::Hard => (35, 35),
            GameDifficulty::Normal => (25, 25),
            GameDifficulty::Easy => (15, 15),
            GameDifficulty::Extreme => (70, 70),
            GameDifficulty::Insane => (85, 85),
            GameDifficulty::VeryEasy => (8, 8),
            GameDifficulty::Intermediate => (45, 45),
            GameDifficulty::Expert => (100, 100),
            GameDifficulty::Basic => (11, 11),
            GameDifficulty::VeryHard => (55, 55),
        }
    }
}

impl SnakeGame {
    pub fn new(difficulty: GameDifficulty) -> SnakeGame {
        let (width, height) = difficulty.game_size();
        let snake_logic = SnakeLogic::new(width, height).expect("Cannot fail");
        let now = Instant::now();

        SnakeGame {
            snake_logic,
            now,
            paused: false,
            last_logic_update: Instant::now(),
            last_game_result: GameResult::NoOp,
            timestep: difficulty.timestep(),
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

    #[cfg(test)]
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
    pub fn update(&mut self, now: web_time::Instant) {
        if self.last_game_result.is_over() {
            return;
        }
        self.now = now;
        if now - self.last_logic_update > self.timestep {
            if !self.is_paused() {
                self.last_game_result = self.snake_logic.next_step();
            }
            self.last_logic_update = now;
        }
    }
}
