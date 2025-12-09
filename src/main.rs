mod logic;

use crate::logic::game::SnakeGame;
use iced::keyboard::Key;
use iced::widget::canvas::event::Status::{Captured, Ignored};
use iced::widget::canvas::{Frame, Geometry, Text};
use iced::window;
use iced::{Element, Fill, Font, Pixels, Rectangle, Renderer, Size, Subscription, Theme};
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
        let snake_logic = SnakeGame::new(25, 25);
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
    let iy = square_y * h_s;
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
        let horizontal_alignment = iced::alignment::Horizontal::Center;
        let vertical_alignment = iced::alignment::Vertical::Center;
        let shaping = iced::widget::text::Shaping::Basic;
        let line_height = iced::widget::text::LineHeight::Absolute(25.into());

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
            let size = Pixels::from(25.);
            let font = Font {
                family: iced::font::Family::default(),
                weight: iced::font::Weight::Medium,
                stretch: iced::font::Stretch::Normal,
                style: iced::font::Style::Normal,
            };

            let score = Text {
                content: format!(
                    "Your score: {:?}",
                    self.snake_game.lock().expect("Poisoned").score()
                ),
                position: iced::Point { x: 500., y: 725. },
                color: iced::Color::from_rgb8(255, 0, 0),
                size,
                line_height,
                font,
                horizontal_alignment,
                vertical_alignment,
                shaping,
            };

            frame.fill_text(score);

            if self.snake_game.lock().expect("Poisoned").is_over() {
                let game_over = Text {
                    content: format!(
                        "Game Over. Press space to start a new game. Your score: {:?}",
                        self.snake_game.lock().expect("Poisoned").score()
                    ),
                    position: frame.center(),
                    color: iced::Color::from_rgb8(255, 0, 0),
                    size,
                    line_height,
                    font,
                    horizontal_alignment,
                    vertical_alignment,
                    shaping,
                };

                frame.fill_text(game_over);
            }
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
                        let mut game = self.snake_game.lock().expect("Poisoned");

                        game.change_direction(Direction::Up);

                        (Captured, Some(T::default()))
                    } else if key == Key::Named(iced::keyboard::key::Named::ArrowDown) {
                        let mut game = self.snake_game.lock().expect("Poisoned");

                        game.change_direction(Direction::Down);

                        (Captured, Some(T::default()))
                    } else if key == Key::Named(iced::keyboard::key::Named::ArrowLeft) {
                        let mut game = self.snake_game.lock().expect("Poisoned");

                        game.change_direction(Direction::Left);
                        (Captured, Some(T::default()))
                    } else if key == Key::Named(iced::keyboard::key::Named::ArrowRight) {
                        let mut game = self.snake_game.lock().expect("Poisoned");

                        game.change_direction(Direction::Right);
                        (Captured, Some(T::default()))
                    } else if key == Key::Named(iced::keyboard::key::Named::Space) {
                        let mut game = self.snake_game.lock().expect("Poisoned");
                        if game.is_over() {
                            *game = SnakeGame::new(game.width(), game.height());
                            (Captured, Some(T::default()))
                        } else {
                            let logic_not_paused = !game.is_paused();
                            game.set_paused(logic_not_paused);
                            (Captured, Some(T::default()))
                        }
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
