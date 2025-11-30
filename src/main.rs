pub mod logic;

use iced::widget::canvas::{Frame, Geometry};
use iced::window;
use iced::{Element, Fill, Rectangle, Renderer, Size, Subscription, Theme};

use std::time::Instant;

use crate::logic::{GameResult, SnakeLogic};

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();
    iced::application(
        "Snaaaaaaaaaaaaaake! ============================================================================================<",
        SolarSystem::update,
        SolarSystem::view,
    )
    .subscription(SolarSystem::subscription)
    .theme(SolarSystem::theme)
    .run()
}

#[derive(Default)]
struct SolarSystem {
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(Instant),
}

impl SolarSystem {
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick(instant) => {
                self.state.update(instant);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        iced::widget::canvas(&self.state)
            .width(Fill)
            .height(Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Moonfly
    }

    fn subscription(&self) -> Subscription<Message> {
        window::frames().map(Message::Tick)
    }
}

#[derive(Debug)]
struct State {
    system_cache: iced::widget::canvas::Cache,
    now: Instant,
    snake_logic: logic::SnakeLogic,
    last_logic_update: Instant,
    last_game_result: GameResult,
}

impl State {
    pub fn new() -> State {
        let snake_logic = SnakeLogic::new(5, 5).expect("Cannot fail");
        State {
            system_cache: iced::widget::canvas::Cache::default(),
            now: Instant::now(),
            snake_logic,
            last_logic_update: Instant::now(),
            last_game_result: GameResult::NoOp,
        }
    }

    pub fn update(&mut self, now: Instant) {
        self.now = now;
        {
            // TODO: change to something dynamic
            const TIMESTEP: std::time::Duration = std::time::Duration::from_secs(1);
            if now - self.last_logic_update > TIMESTEP {
                self.last_game_result = self.snake_logic.next_step();
                self.last_logic_update = now;
            }
        }
        self.system_cache.clear();
    }
}
/// This function does a transformation from the logic system to the graphics and draws the square.
/// Can draw a square in any color
pub fn draw_snake_square(
    frame: &mut Frame<Renderer>,
    color: iced::Color,
    (square_x, square_y): (usize, usize),
    (game_square_width, game_square_height): (usize, usize),
) {
    let h_s = frame.height() as usize / game_square_height;
    let w_s = frame.width() as usize / game_square_width;
    let ix = square_x * w_s;
    let iy = square_y * h_s;
    let top_left = iced::Point {
        x: ix as f32,
        y: iy as f32,
    };

    frame.fill_rectangle(top_left, Size::new(w_s as f32, h_s as f32), color);
}

impl<Message> iced::widget::canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let my_snake = self.system_cache.draw(renderer, bounds.size(), |frame| {
            for (snake_x, snake_y) in self.snake_logic.snake() {
                draw_snake_square(
                    frame,
                    iced::Color::from_rgb8(0, u8::MAX, 0),
                    (*snake_x, *snake_y),
                    (self.snake_logic.width(), self.snake_logic.height()),
                );
            }
            draw_snake_square(
                frame,
                iced::Color::from_rgb8(u8::MAX, 0, 0),
                self.snake_logic.food(),
                (self.snake_logic.width(), self.snake_logic.height()),
            );
        });
        vec![my_snake]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        _event: iced::widget::canvas::Event,
        _bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> (iced::widget::canvas::event::Status, Option<Message>) {
        (iced::widget::canvas::event::Status::Ignored, None)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
