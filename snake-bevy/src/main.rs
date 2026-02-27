const DOUBLE_TAP_TIMEOUT: Duration = Duration::from_millis(500);

use std::collections::BTreeMap;
use web_time::Duration;

use bevy::{
    asset::load_internal_binary_asset, input::common_conditions::input_just_pressed,
    platform::time::Instant, prelude::*, sprite::Text2dShadow,
};

use snake_game::{game_with_menu::GameWithMenu, traits::DrawableOn};
#[derive(Debug, bevy::prelude::Resource, Default)]
struct GameWithMenuResource(GameWithMenu);

#[repr(u8)]
#[derive(Debug, PartialEq)]
#[must_use]
enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
    NoOp,
    Center,
}

#[derive(Resource, Debug, Default)]
struct TouchPositions {
    mouse_positions: Option<(f32, f32)>,
    time_of_last_tap: Option<web_time::Instant>,
}

impl TouchPositions {
    #[cfg(test)]
    fn new() -> Self {
        Self {
            mouse_positions: None,
            time_of_last_tap: None,
        }
    }

    fn on_tap(&mut self, mut game_with_menu: ResMut<GameWithMenuResource>) {
        let is_double_tap = self.is_double_tap();

        match self.direction() {
            SwipeDirection::Up => {
                game_with_menu.0.up_pressed();
            }
            SwipeDirection::Down => {
                game_with_menu.0.down_pressed();
            }
            SwipeDirection::Left => {
                game_with_menu.0.left_pressed();
            }
            SwipeDirection::Right => {
                game_with_menu.0.right_pressed();
            }
            SwipeDirection::Center => {
                if is_double_tap {
                    game_with_menu.0.enter_or_space_pressed();
                }
            }
            SwipeDirection::NoOp => {}
        }
    }

    fn is_double_tap(&mut self) -> bool {
        // Initial state of self.time_of_last_tap: None.
        match self.time_of_last_tap {
            // User tapped and self.time_of_last_tap was captured.
            Some(time_of_last_tap) => {
                // Here we check if time_of_last_tap was less than DOUBLE_TAP_TIMEOUT.
                if Instant::now() - time_of_last_tap < DOUBLE_TAP_TIMEOUT {
                    self.time_of_last_tap = None;

                    return true;
                } else {
                    self.time_of_last_tap = None;
                }
            }
            // User did not tap yet or completed a double tap.
            None => self.time_of_last_tap = Some(Instant::now()),
        }
        false
    }

    fn direction(&self) -> SwipeDirection {
        let (mouse_positions_x, mouse_positions_y) = match self.mouse_positions {
            Some(pos) => pos,
            None => {
                return SwipeDirection::NoOp;
            }
        };

        let position_x = mouse_positions_x as u32;
        let position_y = mouse_positions_y as u32;
        let x_extent_third = X_EXTENT / 3;
        let y_extent_third = Y_EXTENT / 3;

        if (position_x > x_extent_third && position_x < x_extent_third * 2)
            && (position_y < y_extent_third)
        {
            return SwipeDirection::Up;
        } else if (position_x < x_extent_third * 2 && position_x > x_extent_third)
            && (position_y > y_extent_third * 2)
        {
            return SwipeDirection::Down;
        } else if (position_x < x_extent_third)
            && (position_y > y_extent_third && position_y < y_extent_third * 2)
        {
            return SwipeDirection::Left;
        } else if (position_x > x_extent_third * 2)
            && (position_y > y_extent_third && position_y < y_extent_third * 2)
        {
            return SwipeDirection::Right;
        } else if (position_x > x_extent_third && position_x < x_extent_third * 2)
            && (position_y < y_extent_third * 2 && position_y > y_extent_third)
        {
            return SwipeDirection::Center;
        } else {
            return SwipeDirection::NoOp;
        }
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (X_EXTENT, Y_EXTENT).into(),
            resizable: true,
            ..default()
        }),
        ..default()
    }));

    app.init_resource::<GameWithMenuResource>();

    app.init_resource::<TouchPositions>();
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
        );
    load_internal_binary_asset!(
        app,
        Handle::default(),
        "../fonts/FiraSans-Bold.ttf",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    app.run();
}
const X_EXTENT: u32 = 1000;
const Y_EXTENT: u32 = 600;

fn setup(mut commands: Commands) {
    let projection = Projection::Orthographic(OrthographicProjection {
        scaling_mode: bevy::camera::ScalingMode::Fixed {
            width: X_EXTENT as f32,
            height: Y_EXTENT as f32,
        },
        ..OrthographicProjection::default_2d()
    });

    commands.spawn((Camera2d, projection));
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
        let text_x = x as f32 - (X_EXTENT / 2) as f32;
        let text_y = -(y as f32) + (Y_EXTENT / 2) as f32;

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
                transform.translation = Vec3::new(text_x as f32, text_y as f32, text_z);
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
        // let game_top_left = (-775f32, 465f32);
        //   Position xyz for rectangle
        let rect_x = (top_left.0 as f32) - ((X_EXTENT) / 2) as f32 + size.0 as f32 / 2.;
        let rect_y = -(top_left.1 as f32) + ((Y_EXTENT) / 2) as f32 - size.1 as f32 / 2.;
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
                transform.translation = Vec3::new(rect_x as f32, rect_y as f32, rect_z);
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
                    Transform::from_xyz(rect_x as f32, rect_y as f32, rect_z),
                    Mesh2d(rectangle),
                ));

                //   Adding the drawn rectangle to th>e used list
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
    mut mouse_positions: ResMut<TouchPositions>,
    game_with_menu: ResMut<GameWithMenuResource>,
) {
    // We here need to take one finger. touches.iter().next() captures A finger if there is one.
    // We need this to set the position and detect that a finger was pressed.

    match touches.iter().next() {
        // No fingers are tapping
        None => {
            mouse_positions.mouse_positions = None;

            return;
        }

        // A finger or more are tapping.
        Some(touch) => match mouse_positions.mouse_positions {
            // Hold
            Some(_) => {
                return;
            }

            // Tap
            None => {
                mouse_positions.mouse_positions = Some(touch.position().into());
                mouse_positions.on_tap(game_with_menu);
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{SwipeDirection, TouchPositions};
    #[test]
    fn direction() {
        {
            let mouse_positions = TouchPositions::new();
            assert_eq!(mouse_positions.direction(), SwipeDirection::NoOp);
        }
    }
}
