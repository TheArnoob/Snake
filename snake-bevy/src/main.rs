use bevy::platform::time::Instant;
use std::{
    collections::{BTreeMap, VecDeque},
    f32::consts::PI,
};

use bevy::{
    asset::load_internal_binary_asset, input::common_conditions::input_just_pressed, prelude::*,
    sprite::Text2dShadow,
};

use snake_game::{game_with_menu::GameWithMenu, traits::DrawableOn};
#[derive(Debug, bevy::prelude::Resource, Default)]
struct GameWithMenuResource(GameWithMenu);

#[derive(Resource, Debug, Default)]
struct MousePositions {
    mouse_positions: VecDeque<(f32, f32)>,
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
#[must_use]
enum MouseDirection {
    Up,
    Down,
    Left,
    Right,
    Tap,
    NoOp,
}

#[derive(Debug)]
enum FitResult {
    TooSmall,
    Normal(f32),
    InfinitySlope,
    NoOp,
}

impl MousePositions {
    const SMALLEST_SIZE_OF_MOUSE_MOVE: usize = 3;

    #[cfg(test)]
    fn new() -> Self {
        Self {
            mouse_positions: VecDeque::new(),
        }
    }

    fn linear_fit(&self) -> FitResult {
        if self.mouse_positions.is_empty() {
            return FitResult::NoOp;
        }
        if self.mouse_positions.len() < Self::SMALLEST_SIZE_OF_MOUSE_MOVE {
            return FitResult::TooSmall;
        }
        let x_average = self.mouse_positions.iter().map(|(x, _)| x).sum::<f32>()
            / self.mouse_positions.len() as f32;
        let y_average = self.mouse_positions.iter().map(|(_, y)| y).sum::<f32>()
            / self.mouse_positions.len() as f32;

        let slope_denominator = self
            .mouse_positions
            .iter()
            .map(|(x, _)| (x - x_average) * (x - x_average))
            .sum::<f32>();
        let slope_numerator = self
            .mouse_positions
            .iter()
            .map(|(x, y)| (x - x_average) * (y - y_average))
            .sum::<f32>();
        if slope_denominator == 0. {
            return FitResult::InfinitySlope;
        }
        FitResult::Normal(slope_numerator / slope_denominator)
    }

    fn direction(&self) -> MouseDirection {
        match self.linear_fit() {
            FitResult::TooSmall => MouseDirection::Tap,
            FitResult::Normal(slope) => {
                let angle = slope.atan();
                if angle < PI / 4. && angle > -PI / 4. {
                    if self.mouse_positions.front().expect("Empty Vector").0
                        < self.mouse_positions.back().expect("Empty Vector").0
                    {
                        return MouseDirection::Right;
                    } else {
                        return MouseDirection::Left;
                    }
                } else {
                    if self.mouse_positions.front().expect("Empty Vector").1
                        < self.mouse_positions.back().expect("Empty Vector").1
                    {
                        return MouseDirection::Down;
                    } else {
                        return MouseDirection::Up;
                    }
                }
            }
            FitResult::InfinitySlope => {
                if self.mouse_positions.front().expect("Empty Vector").1
                    < self.mouse_positions.back().expect("Empty Vector").1
                {
                    MouseDirection::Down
                } else {
                    MouseDirection::Up
                }
            }
            FitResult::NoOp => MouseDirection::NoOp,
        }
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.init_resource::<GameWithMenuResource>();

    app.init_resource::<MousePositions>();

    app.init_resource::<Entities>();

    app.add_systems(Startup, setup)
        .add_systems(
            Update,
            snake_space_pressed.run_if(input_just_pressed(KeyCode::Space)),
        )
        .add_systems(Update, (update_time, draw_frame).chain())
        .add_systems(Update, touch_system)
        .add_systems(
            Update,
            snake_left_pressed.run_if(input_just_pressed(KeyCode::ArrowLeft)),
        )
        .add_systems(
            Update,
            snake_right_pressed.run_if(input_just_pressed(KeyCode::ArrowRight)),
        )
        .add_systems(
            Update,
            snake_up_pressed.run_if(input_just_pressed(KeyCode::ArrowUp)),
        )
        .add_systems(
            Update,
            snake_down_pressed.run_if(input_just_pressed(KeyCode::ArrowDown)),
        )
        .add_systems(
            Update,
            snake_space_pressed.run_if(input_just_pressed(KeyCode::Enter)),
        )
        .add_systems(
            Update,
            snake_space_pressed.run_if(input_just_pressed(KeyCode::Space)),
        )
        .add_systems(Update, setup.run_if(input_just_pressed(KeyCode::AltLeft)));
    load_internal_binary_asset!(
        app,
        Handle::default(),
        "../fonts/FiraSans-Bold.ttf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    app.run();
}

const X_EXTENT: f32 = 1220.;
const Y_EXTENT: f32 = 620.;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn snake_space_pressed(mut game_with_menu: ResMut<GameWithMenuResource>) {
    game_with_menu.0.enter_or_space_pressed();
}

fn snake_left_pressed(mut game_with_menu: ResMut<GameWithMenuResource>) {
    game_with_menu.0.left_pressed();
}

fn snake_right_pressed(mut game_with_menu: ResMut<GameWithMenuResource>) {
    game_with_menu.0.right_pressed();
}

fn snake_up_pressed(mut game_with_menu: ResMut<GameWithMenuResource>) {
    game_with_menu.0.up_pressed();
}

fn snake_down_pressed(mut game_with_menu: ResMut<GameWithMenuResource>) {
    game_with_menu.0.down_pressed();
}

#[derive(Resource, Default)]
struct Entities {
    used_rects: Vec<Entity>,
    unused_rects: Vec<Entity>,
    materials: BTreeMap<(u8, u8, u8), AssetId<ColorMaterial>>,
    mesh_map: BTreeMap<(usize, usize), AssetId<Mesh>>,
    unused_text: Vec<Entity>,
    used_text: Vec<Entity>,
}

struct Frame<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j> {
    commands: Commands<'a, 'b>,
    meshes: ResMut<'a, Assets<Mesh>>,
    materials: ResMut<'a, Assets<ColorMaterial>>,
    entities: ResMut<'a, Entities>,
    rect_query: Query<
        'a,
        'b,
        (
            &'c mut Transform,
            &'d mut Visibility,
            &'e mut MeshMaterial2d<ColorMaterial>,
            &'j mut Mesh2d,
        ),
        Without<Text2d>,
    >,
    text_query: Query<
        'a,
        'b,
        (
            &'f mut Transform,
            &'g mut Visibility,
            &'h mut TextColor,
            &'i mut Text2d,
        ),
        Without<MeshMaterial2d<ColorMaterial>>,
    >,
}

impl DrawableOn for Frame<'_, '_, '_, '_, '_, '_, '_, '_, '_, '_> {
    fn draw_text(&mut self, text: &str, color_rgb: (u8, u8, u8), x: usize, y: usize, size: f32) {
        let text_x = (x as f32) - X_EXTENT / 2.;
        let text_y = -(y as f32) + Y_EXTENT / 2.;

        let text_z = 0f32;
        let text_color = TextColor(Color::linear_rgb(
            color_rgb.0 as f32 / 255.,
            color_rgb.1 as f32 / 255.,
            color_rgb.2 as f32 / 255.,
        ));

        match self.entities.unused_text.pop() {
            Some(t) => {
                let (mut transform, mut vis, mut color, mut text_comp) =
                    self.text_query.get_mut(t).expect("Cannot fail");
                transform.translation = Vec3::new(text_x, text_y, text_z);
                *vis = Visibility::Visible;
                *color = text_color;
                text_comp.0 = text.to_string();
                self.entities.used_text.push(t);
            }
            None => {
                let text_font: TextFont = TextFont {
                    font_size: size / 1.2,
                    ..Default::default()
                };

                let text_entity = self.commands.spawn((
                    Text2d::new(text.to_string()),
                    text_font,
                    Transform::from_xyz(0., 100., 0.),
                    Text2dShadow::default(),
                    text_color,
                ));
                self.entities.used_text.push(text_entity.id());
            }
        }

        //   Going over all leftover text and then doing the following:
        self.entities.unused_text.iter().for_each(|entity| {
            //   Destructuring queries entity
            let (_transform, mut vis, _, _text) =
                self.text_query.get_mut(*entity).expect("Cannot fail");
            //   Changing their visibility to invisible
            *vis = Visibility::Hidden;
        });
    }
    fn height(&self) -> usize {
        Y_EXTENT as usize
    }

    fn width(&self) -> usize {
        X_EXTENT as usize
    }

    fn fill_rectangle(
        &mut self,
        size: (usize, usize),
        color_rgb: (u8, u8, u8),
        top_left: (usize, usize),
    ) {
        //   Position xyz for rectangle
        let rect_x = (top_left.0 as f32) - X_EXTENT / 2.;
        let rect_y = -(top_left.1 as f32) + Y_EXTENT / 2.;
        let rect_z = 0.;
        //   This takes the color id from the btreemap and inserts the id if it is not there.
        let color_id = self.entities.materials.entry(color_rgb).or_insert(
            self.materials
                .add(Color::linear_rgb(
                    color_rgb.0 as f32 / 255.,
                    color_rgb.1 as f32 / 255.,
                    color_rgb.2 as f32 / 255.,
                ))
                .id(),
        );

        let color_material_handle = self
            .materials
            .get_strong_handle(*color_id)
            .expect("Cannot fail");

        let rectangle_id = self.entities.mesh_map.entry(size).or_insert(
            self.meshes
                .add(Rectangle::new(size.0 as f32, size.1 as f32))
                .id(),
        );

        let rectangle = self
            .meshes
            .get_strong_handle(*rectangle_id)
            .expect("Cannot fail");

        match self.entities.unused_rects.pop() {
            Some(rect) => {
                let (mut transform, mut vis, mut color, mut mesh) =
                    self.rect_query.get_mut(rect).expect("Cannot fail");
                //   Changing the position to the desired position (See rect_x/y/z)
                transform.translation = Vec3::new(rect_x, rect_y, rect_z);
                //   Making the rectangle visible
                *vis = Visibility::Visible;
                *mesh = Mesh2d(rectangle);
                //   Changing the color the color_material_handle color
                *color = MeshMaterial2d(color_material_handle);
                //   Adding the drawn rectangle to the used list
                self.entities.used_rects.push(rect);
            }
            None => {
                let rectangle_entity = self.commands.spawn((
                    MeshMaterial2d(color_material_handle),
                    Transform::from_xyz(rect_x, rect_y, rect_z),
                    Mesh2d(rectangle),
                ));

                //   Adding the drawn rectangle to the used list
                self.entities.used_rects.push(rectangle_entity.id());
            }
        }

        //   Going over all leftover rectangles and then doing the following:
        self.entities.unused_rects.iter().for_each(|entity| {
            //   Destructuring queries entity
            let (_transform, mut vis, _, _) =
                self.rect_query.get_mut(*entity).expect("Cannot fail");
            //   Changing their visibility to invisible
            *vis = Visibility::Hidden;
        });
    }
}

fn update_time(mut game_with_menu: ResMut<GameWithMenuResource>) {
    game_with_menu.0.update(Instant::now());
}
fn draw_frame(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut entities: ResMut<Entities>,
    game_with_menu: ResMut<GameWithMenuResource>,
    rect_query: Query<
        (
            &mut Transform,
            &mut Visibility,
            &mut MeshMaterial2d<ColorMaterial>,
            &mut Mesh2d,
        ),
        Without<Text2d>,
    >,
    text_query: Query<
        (&mut Transform, &mut Visibility, &mut TextColor, &mut Text2d),
        Without<MeshMaterial2d<ColorMaterial>>,
    >,
) {
    //   Adding all used rectangles to the unused ones.
    {
        let mut buffer = Vec::new();
        buffer.append(&mut entities.used_rects);
        entities.unused_rects.append(&mut buffer);
    }
    {
        let mut buffer = Vec::new();
        buffer.append(&mut entities.used_text);
        entities.unused_text.append(&mut buffer);
    }
    let mut frame = Frame {
        commands,
        meshes,
        materials,
        entities,
        rect_query,
        text_query,
    };
    game_with_menu.0.draw(&mut frame);
}

fn touch_system(
    touches: Res<Touches>,
    mut mouse_positions: ResMut<MousePositions>,
    mut game_with_menu: ResMut<GameWithMenuResource>,
) {
    match touches.iter().next() {
        Some(touch) => {
            mouse_positions
                .mouse_positions
                .push_back((touch.position().x, touch.position().y));

            if mouse_positions.mouse_positions.len() > 1000 {
                mouse_positions.mouse_positions.pop_front();
            }
        }
        None => {
            match mouse_positions.direction() {
                MouseDirection::Up => game_with_menu.0.up_pressed(),
                MouseDirection::Down => game_with_menu.0.down_pressed(),
                MouseDirection::Left => game_with_menu.0.left_pressed(),
                MouseDirection::Right => game_with_menu.0.right_pressed(),
                MouseDirection::Tap => game_with_menu.0.enter_or_space_pressed(),
                MouseDirection::NoOp => {}
            }
            mouse_positions.mouse_positions.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{MouseDirection, MousePositions};

    #[test]
    fn direction() {
        {
            let mouse_positions = MousePositions::new();
            assert_eq!(mouse_positions.direction(), MouseDirection::NoOp);
        }
        {
            let mut mouse_positions = MousePositions::new();
            mouse_positions.mouse_positions.push_back((1., 1.));
            assert_eq!(mouse_positions.direction(), MouseDirection::Tap);
        }

        {
            let mouse_positions = MousePositions {
                mouse_positions: [
                    (1., 1.),
                    (0., 2.),
                    (1., 3.),
                    (2., 4.),
                    (3., 5.),
                    (4., 6.),
                    (3., 7.),
                    (2., 8.),
                    (1., 9.),
                    (2., 10.),
                    (3., 11.),
                    (2., 12.),
                    (3., 13.),
                    (4., 14.),
                    (3., 15.),
                    (2., 16.),
                ]
                .into(),
            };

            assert_eq!(mouse_positions.direction(), MouseDirection::Down);
            let mouse_positions = mouse_positions
                .mouse_positions
                .clone()
                .into_iter()
                .rev()
                .collect::<VecDeque<(f32, f32)>>();
            assert_eq!(
                MousePositions { mouse_positions }.direction(),
                MouseDirection::Up
            );
        }

        {
            let mouse_positions = MousePositions {
                mouse_positions: (0..=17).map(|i| (0., i as f32)).collect(),
            };
            assert_eq!(mouse_positions.direction(), MouseDirection::Down);
            let mouse_positions = mouse_positions
                .mouse_positions
                .clone()
                .into_iter()
                .rev()
                .collect::<VecDeque<(f32, f32)>>();
            assert_eq!(
                MousePositions { mouse_positions }.direction(),
                MouseDirection::Up
            );
        }
        {
            let positions = MousePositions {
                mouse_positions: [
                    (3., 1.),
                    (4., 2.),
                    (5., 2.),
                    (6., 1.),
                    (7., 2.),
                    (8., 1.),
                    (9., 1.),
                    (10., 1.),
                    (11., 3.),
                    (12., 2.),
                    (13., 5.),
                    (14., 6.),
                    (15., 4.),
                    (16., 2.),
                    (17., 1.),
                    (18., 2.),
                ]
                .into(),
            };

            assert_eq!(positions.direction(), MouseDirection::Right);

            let mouse_positions: VecDeque<(f32, f32)> = positions
                .mouse_positions
                .clone()
                .into_iter()
                .rev()
                .collect();
            let mouse_positions = MousePositions { mouse_positions };
            assert_eq!(mouse_positions.direction(), MouseDirection::Left)
        }

        {
            let positions = MousePositions {
                mouse_positions: [
                    (3., 1.),
                    (4., 1.),
                    (5., 1.),
                    (6., 1.),
                    (7., 1.),
                    (8., 1.),
                    (9., 1.),
                    (10., 1.),
                    (11., 1.),
                    (12., 1.),
                    (13., 1.),
                    (14., 1.),
                    (15., 1.),
                    (16., 1.),
                    (17., 1.),
                    (18., 1.),
                ]
                .into(),
            };

            assert_eq!(positions.direction(), MouseDirection::Right);

            let mouse_positions: VecDeque<(f32, f32)> = positions
                .mouse_positions
                .clone()
                .into_iter()
                .rev()
                .collect();
            let mouse_positions = MousePositions { mouse_positions };
            assert_eq!(mouse_positions.direction(), MouseDirection::Left)
        }
    }
}
