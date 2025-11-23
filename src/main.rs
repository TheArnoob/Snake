mod logic;

use iced::mouse;
use iced::widget::canvas::stroke::{self, Stroke};
use iced::widget::canvas::{Geometry, Path};
use iced::widget::{canvas, image};
use iced::window;
use iced::{Color, Element, Fill, Point, Rectangle, Renderer, Size, Subscription, Theme, Vector};

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
        canvas(&self.state).width(Fill).height(Fill).into()
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
    system_cache: canvas::Cache,
    now: Instant,
}

impl State {
    pub fn new() -> State {
        State {
            system_cache: canvas::Cache::default(),
            now: Instant::now(),
        }
    }

    pub fn update(&mut self, now: Instant) {
        self.now = now;
        self.system_cache.clear();
    }
}

impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let my_snake = self.system_cache.draw(renderer, bounds.size(), |frame| {
            let center = frame.center();
            frame.fill_rectangle(center, Size::new(20., 20.), Color::from_rgb8(0, u8::MAX, 0));
        });

        let my_food = self.system_cache.draw(renderer, bounds.size(), |frame| {
            let center = frame.center();
            frame.fill_rectangle(center, Size::new(20., 20.), Color::from_rgb8(u8::MAX, 0, 0))
        });
        vec![my_food, my_snake]
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
