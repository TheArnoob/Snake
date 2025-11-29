pub mod logic;

use iced::widget::canvas::{Frame, Geometry};
use iced::window;
use iced::{Element, Fill, Rectangle, Renderer, Size, Subscription, Theme};

use std::time::Instant;
use std::u8;

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
}

impl State {
    pub fn new() -> State {
        State {
            system_cache: iced::widget::canvas::Cache::default(),
            now: Instant::now(),
        }
    }

    pub fn update(&mut self, now: Instant) {
        self.now = now;
        self.system_cache.clear();
    }
}
/// This function does a transformation from the logic system to the graphics and draws the square.
/// Can draw a square in any color
pub fn draw_snake_square(
    frame: &mut Frame<Renderer>,
    color: iced::Color,
    (square_x, square_y): (usize, usize),
    (game_height, game_square_height): (usize, usize),
) {
    let h_s = frame.height() as usize / game_square_height;
    let w_s = frame.width() as usize / game_height;
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
            draw_snake_square(frame, iced::Color::from_rgb8(0, 0, u8::MAX), (4, 0), (5, 7));
        });
        vec![my_snake]
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
