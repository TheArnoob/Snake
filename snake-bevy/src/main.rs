use bevy::{
    asset::load_internal_binary_asset, input::common_conditions::input_toggle_active, prelude::*,
    sprite::Text2dShadow,
};
use bevy::{input::common_conditions::input_just_pressed, sprite_render::Wireframe2dConfig};

use snake_game::game_with_menu::GameWithMenu;
#[derive(Debug, bevy::prelude::Resource, Default)]
struct GameWithMenuResource(GameWithMenu);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.init_resource::<GameWithMenuResource>();

    app.add_systems(Startup, setup)
        .add_systems(
            Update,
            snake_space_pressed.run_if(input_just_pressed(KeyCode::Space)),
        )
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
        .add_systems(
            Update,
            rotate.run_if(input_toggle_active(false, KeyCode::KeyR)),
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

const X_EXTENT: f32 = 1000.;
const Y_EXTENT: f32 = 150.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let text_font = TextFont {
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

    let rectangle = meshes.add(Rectangle::new(50.0, 100.0));

    // Distribute colors evenly across the rainbow.
    let color = Color::linear_rgb(0., 1., 0.);

    commands.spawn((
        Mesh2d(rectangle),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(-X_EXTENT / 2., Y_EXTENT / 2., 0.0),
    ));
}

fn toggle_wireframe(mut wireframe_config: ResMut<Wireframe2dConfig>) {
    wireframe_config.global = !wireframe_config.global;
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

fn rotate(mut query: Query<&mut Transform, With<Mesh2d>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_z(time.delta_secs() / 2.);
        transform.rotate_x(time.delta_secs() / 2.);
        transform.rotate_y(time.delta_secs() / 2.);
    }
}
