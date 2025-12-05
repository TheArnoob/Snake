mod logic;

use crate::logic::game::SnakeGame;
use iced::keyboard::Key;
use iced::widget::canvas::event::Status::{Captured, Ignored};
use iced::widget::canvas::{Frame, Geometry};
use iced::window;
use iced::{Element, Fill, Rectangle, Renderer, Size, Subscription, Theme};
use logic::Direction;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();
    iced::application(
        "Snaaaaaaaaaaaaaake! ============================================================================================<",
        SnakeGUI::update,
        SnakeGUI::view,
    )
    .subscription(SnakeGUI::subscription)
    .theme(SnakeGUI::theme)
    .run()
}

struct SnakeGUI {
    system_cache: iced::widget::canvas::Cache,
    now: Instant,
    snake_game: Arc<Mutex<SnakeGame>>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(Instant),
}

impl Default for Message {
    fn default() -> Self {
        Message::Tick(Instant::now())
    }
}

impl SnakeGUI {
    fn update(&mut self, message: Message) {
        match message {
            Message::Tick(now) => {
                self.now = now;
                self.snake_game.lock().expect("Poisoned").update(now);
                self.system_cache.clear();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        iced::widget::canvas(self).width(Fill).height(Fill).into()
    }

    fn theme(&self) -> Theme {
        Theme::Moonfly
    }

    fn subscription(&self) -> Subscription<Message> {
        window::frames().map(Message::Tick)
    }

    pub fn new() -> Self {
        let snake_logic = SnakeGame::new();
        Self {
            system_cache: iced::widget::canvas::Cache::default(),
            now: Instant::now(),
            snake_game: Arc::new(Mutex::new(snake_logic)),
        }
    }
}
/// This function does a transformation from the logic system to the graphics and draws the square.
/// Can draw a square in any color or size
pub fn draw_snake_square(
    frame: &mut Frame<Renderer>,
    color: iced::Color,
    (square_x, square_y): (usize, usize),
    (game_square_width, game_square_height): (usize, usize),
) {
    let h_s = frame.height() as usize / game_square_height;
    let w_s = frame.width() as usize / game_square_width;
    let ix = square_x * w_s;
    let iy: usize = square_y * h_s;
    let top_left = iced::Point {
        x: ix as f32,
        y: iy as f32,
    };

    frame.fill_rectangle(top_left, Size::new(w_s as f32, h_s as f32), color);
}

impl<T: Default> iced::widget::canvas::Program<T> for SnakeGUI {
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
            let game_width = self.snake_game.lock().expect("Poisoned").width();
            let game_height = self.snake_game.lock().expect("Poisoned").height();
            for (snake_x, snake_y) in self.snake_game.lock().expect("Poisoned").snake() {
                draw_snake_square(
                    frame,
                    iced::Color::from_rgb8(0, u8::MAX, 0),
                    (*snake_x, *snake_y),
                    (game_width, game_height),
                );
            }
            draw_snake_square(
                frame,
                iced::Color::from_rgb8(u8::MAX, 0, 0),
                self.snake_game.lock().expect("Poisoned").food(),
                (game_width, game_height),
            );
        });
        vec![my_snake]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: iced::widget::canvas::Event,
        _bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> (iced::widget::canvas::event::Status, Option<T>) {
        match event {
            iced::widget::canvas::Event::Mouse(_event) => (Ignored, None),
            iced::widget::canvas::Event::Touch(_event) => (Ignored, None),
            iced::widget::canvas::Event::Keyboard(event) => match event {
                iced::keyboard::Event::KeyPressed {
                    key,
                    modified_key: _modified_key,
                    physical_key: _physical_key,
                    location: _location,
                    modifiers: _modifiers,
                    text: _text,
                } => {
                    if key == Key::Named(iced::keyboard::key::Named::ArrowUp) {
                        let mut logic = self.snake_game.lock().expect("Poisoned");

                        logic.change_direction(Direction::Up);

                        (Captured, Some(T::default()))
                    } else if key == Key::Named(iced::keyboard::key::Named::ArrowDown) {
                        let mut logic = self.snake_game.lock().expect("Poisoned");

                        logic.change_direction(Direction::Down);

                        (Captured, Some(T::default()))
                    } else if key == Key::Named(iced::keyboard::key::Named::ArrowLeft) {
                        let mut logic = self.snake_game.lock().expect("Poisoned");

                        logic.change_direction(Direction::Left);
                        (Captured, Some(T::default()))
                    } else if key == Key::Named(iced::keyboard::key::Named::ArrowRight) {
                        let mut logic = self.snake_game.lock().expect("Poisoned");

                        logic.change_direction(Direction::Right);
                        (Captured, Some(T::default()))
                    } else if key == Key::Named(iced::keyboard::key::Named::Space) {
                        let mut logic = self.snake_game.lock().expect("Poisoned");
                        let logic_not_paused = !logic.is_paused();
                        logic.set_paused(logic_not_paused);
                        (Captured, Some(T::default()))
                    } else {
                        (Ignored, None)
                    }
                }
                iced::keyboard::Event::KeyReleased {
                    key: _,
                    location: _,
                    modifiers: _,
                } => (iced::widget::canvas::event::Status::Captured, None),
                iced::keyboard::Event::ModifiersChanged(_modifiers) => (Ignored, None),
            },
        }
    }
}

impl Default for SnakeGUI {
    fn default() -> Self {
        Self::new()
    }
}
