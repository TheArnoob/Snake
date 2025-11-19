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

    fn view(&self) -> Element<Message> {
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
    sun: image::Handle,
    earth: image::Handle,
    moon: image::Handle,
    space_cache: canvas::Cache,
    system_cache: canvas::Cache,
    start: Instant,
    now: Instant,
    stars: Vec<(Point, f32)>,
}

impl State {
    pub fn new() -> State {
        let now = Instant::now();
        let size = window::Settings::default().size;

        State {
            sun: image::Handle::from_bytes(include_bytes!("../assets/sun.png").as_slice()),
            earth: image::Handle::from_bytes(include_bytes!("../assets/earth.png").as_slice()),
            moon: image::Handle::from_bytes(include_bytes!("../assets/moon.png").as_slice()),
            space_cache: canvas::Cache::default(),
            system_cache: canvas::Cache::default(),
            start: now,
            now,
            stars: Self::generate_stars(size.width, size.height),
        }
    }

    pub fn update(&mut self, now: Instant) {
        self.now = now;
        self.system_cache.clear();
    }

    fn generate_stars(width: f32, height: f32) -> Vec<(Point, f32)> {
        use rand::Rng;

        let mut rng = rand::thread_rng();

        (0..100)
            .map(|_| {
                (
                    Point::new(
                        rng.gen_range((-width / 2.0)..(width / 2.0)),
                        rng.gen_range((-height / 2.0)..(height / 2.0)),
                    ),
                    rng.gen_range(0.5..1.0),
                )
            })
            .collect()
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
        vec![my_snake, my_food]
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
