use std::{collections::BTreeMap, time::Instant};

use bevy::{
    asset::load_internal_binary_asset, input::common_conditions::input_just_pressed, prelude::*,
    sprite::Text2dShadow,
};

use snake_game::{game_with_menu::GameWithMenu, traits::DrawableOn};
#[derive(Debug, bevy::prelude::Resource, Default)]
struct GameWithMenuResource(GameWithMenu);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.init_resource::<GameWithMenuResource>();
    app.init_resource::<Entities>();

    app.add_systems(Startup, setup)
        .add_systems(
            Update,
            snake_space_pressed.run_if(input_just_pressed(KeyCode::Space)),
        )
        .add_systems(Update, (update_time, draw_frame).chain())
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let text_font: TextFont = TextFont {
        font_size: 50.,
        ..Default::default()
    };

    commands.spawn((
        Text2d::new("Hello"),
        text_font,
        TextLayout::new_with_justify(Justify::Center),
        TextBackgroundColor(Color::BLACK.with_alpha(0.5)),
        Text2dShadow::default(),
        TextColor::WHITE,
    ));
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
}

struct Frame<'a, 'b, 'c, 'd, 'e> {
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
        ),
    >,
}

impl DrawableOn for Frame<'_, '_, '_, '_, '_> {
    fn draw_text(
        &mut self,
        _text: &str,
        _color_rgb: (u8, u8, u8),
        _x: usize,
        _y: usize,
        _size: f32,
    ) {
        let text_font = TextFont {
            font_size: 50.,
            ..Default::default()
        };

        self.commands.spawn((
            Text2d::new("Hello"),
            text_font,
            TextLayout::new_with_justify(Justify::Center),
            TextBackgroundColor(Color::BLACK.with_alpha(0.5)),
            Text2dShadow::default(),
            TextColor::WHITE,
        ));
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
        // Position xyz for rectangle
        let rect_x = (top_left.0 as f32) - X_EXTENT / 2.;
        let rect_y = -(top_left.1 as f32) + Y_EXTENT / 2.;
        let rect_z = 0.;
        // This takes the color id from the btreemap and inserts the id if it is not there.
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

        let rectangle = self
            .meshes // Add should only occur once
            .add(Rectangle::new(size.0 as f32, size.1 as f32));
        match self.entities.unused_rects.pop() {
            Some(rect) => {
                let (mut transform, mut vis, mut color) =
                    self.rect_query.get_mut(rect).expect("Cannot fail");
                // Changing the position to the desired position (See rect_x/y/z)
                transform.translation = Vec3::new(rect_x, rect_y, rect_z);
                // Making the rectangle visible
                *vis = Visibility::Visible;
                // Changing the color the color_material_handle color
                *color = MeshMaterial2d(color_material_handle);
                // Adding the drawn rectangle to the used list
                self.entities.used_rects.push(rect);
            }
            None => {
                let rectangle_entity = self.commands.spawn((
                    Mesh2d(rectangle),
                    MeshMaterial2d(color_material_handle),
                    Transform::from_xyz(rect_x, rect_y, rect_z),
                ));

                // Adding the drawn rectangle to the used list
                self.entities.used_rects.push(rectangle_entity.id());
            }
        }

        // Going over all leftover rectangles and then doing the following:
        self.entities.unused_rects.iter().for_each(|entity| {
            // Destructuring queries entity
            let (_transform, mut vis, mut mesh) =
                self.rect_query.get_mut(*entity).expect("Cannot fail");
            // Changing their visibility to invisible
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
    rect_query: Query<(
        &mut Transform,
        &mut Visibility,
        &mut MeshMaterial2d<ColorMaterial>,
    )>,
) {
    // Adding all used rectangles to the unused ones.
    let mut buffer = Vec::new();
    buffer.append(&mut entities.used_rects);
    entities.unused_rects.append(&mut buffer);

    let mut frame = Frame {
        commands,
        meshes,
        materials,
        entities,
        rect_query,
    };
    game_with_menu.0.draw(&mut frame);
}
