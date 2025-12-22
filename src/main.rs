mod logic;

use crate::logic::game_with_menu::GameWithMenu;
use crate::logic::traits::DrawableOn;
use iced::keyboard::Key;
use iced::widget::canvas::event::Status::{Captured, Ignored};
use iced::widget::canvas::{Frame, Geometry, Text};
use iced::{Color, Point, window};
use iced::{Element, Fill, Font, Pixels, Rectangle, Renderer, Size, Subscription, Theme};
use logic::Direction;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();
    iced::application("Snake by Arnold Afach", SnakeGUI::update, SnakeGUI::view)
        .subscription(SnakeGUI::subscription)
        .theme(SnakeGUI::theme)
        .run()
}

struct SnakeGUI {
    system_cache: iced::widget::canvas::Cache,
    now: Instant,
    game_with_menu: Arc<Mutex<GameWithMenu>>,
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
                self.game_with_menu.lock().expect("Poisoned").update(now);
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
}

impl DrawableOn for Frame<Renderer> {
    fn draw_text(&mut self, text: &str, color: (u8, u8, u8), x: usize, y: usize, size: f32) {
        let color = Color::from_rgb8(color.0, color.1, color.2);
        let text = Text {
            content: text.to_string(),
            position: Point {
                x: x as f32,
                y: y as f32,
            },
            color,
            size: Pixels(size),
            line_height: iced::widget::text::LineHeight::Absolute(Pixels(20.)),
            font: Font {
                family: iced::font::Family::SansSerif,
                weight: iced::font::Weight::Black,
                stretch: iced::font::Stretch::Normal,
                style: iced::font::Style::Normal,
            },
            horizontal_alignment: iced::alignment::Horizontal::Center,
            vertical_alignment: iced::alignment::Vertical::Center,
            shaping: iced::widget::text::Shaping::Basic,
        };
        self.fill_text(text);
    }

    fn height(&self) -> usize {
        self.height() as usize
    }

    fn width(&self) -> usize {
        self.width() as usize
    }

    fn fill_rectangle(
        &mut self,
        size: (usize, usize),
        color_rgb: (u8, u8, u8),
        top_left: (usize, usize),
    ) {
        self.fill_rectangle(
            Point {
                x: top_left.0 as f32,
                y: top_left.1 as f32,
            },
            Size {
                width: size.0 as f32,
                height: size.1 as f32,
            },
            Color::from_rgb8(color_rgb.0, color_rgb.1, color_rgb.2),
        );
    }
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
            self.game_with_menu.lock().expect("Poisoned").draw(frame);
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
                        let mut game_with_menu = self.game_with_menu.lock().expect("Poisoned");
                        game_with_menu.up_pressed();
                        (Captured, Some(T::default()))
                    } else if key == Key::Named(iced::keyboard::key::Named::ArrowDown) {
                        let mut game_with_menu = self.game_with_menu.lock().expect("Poisoned");
                        game_with_menu.down_pressed();
                        (Captured, Some(T::default()))
                    } else if key == Key::Named(iced::keyboard::key::Named::ArrowLeft) {
                        let mut game_with_menu = self.game_with_menu.lock().expect("Poisoned");
                        game_with_menu.left_pressed();
                        (Captured, Some(T::default()))
                    } else if key == Key::Named(iced::keyboard::key::Named::ArrowRight) {
                        let mut game_with_menu = self.game_with_menu.lock().expect("Poisoned");
                        game_with_menu.right_pressed();
                        (Captured, Some(T::default()))
                    } else if key == Key::Named(iced::keyboard::key::Named::Space)
                        || key == Key::Named(iced::keyboard::key::Named::Enter)
                    {
                        let mut game_with_menu = self.game_with_menu.lock().expect("Poisoned");
                        game_with_menu.enter_or_space_pressed();
                        (Captured, Some(T::default()))
                    } else if key == Key::Named(iced::keyboard::key::Named::Escape) {
                        std::process::exit(0)
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
        Self {
            system_cache: Default::default(),
            now: Instant::now(),
            game_with_menu: Arc::new(Mutex::new(GameWithMenu::default())),
        }
    }
}
