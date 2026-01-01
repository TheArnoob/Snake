use std::time::Instant;

use bevy::{
    asset::load_internal_binary_asset, prelude::*,
    sprite::Text2dShadow,
    input::common_conditions::input_just_pressed,
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
        ).add_systems(Update, (update_time, draw_frame).chain())
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
     entities: Vec<Entity>,
 }
 
 struct Frame<'a, 'b> {
     commands: Commands<'a, 'b>,
     meshes: ResMut<'a, Assets<Mesh>>,
     materials: ResMut<'a, Assets<ColorMaterial>>,
     entities: ResMut<'a, Entities>,
 }

 impl DrawableOn for Frame<'_, '_> {
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
 
     fn width(&self)  -> usize {
         X_EXTENT as usize
     }
 
     fn fill_rectangle(
         &mut self,
         size: (usize, usize),
         color_rgb: (u8, u8, u8),
         top_left: (usize, usize),
     ) {
         self.commands.spawn(Camera2d);
 
         let rectangle = self
             .meshes
             .add(Rectangle::new(size.0 as f32, size.1 as f32));
 
         let rectangle_entity = self.commands.spawn((
             Mesh2d(rectangle),
             MeshMaterial2d(self.materials.add(Color::linear_rgb(
                 color_rgb.0 as f32 / 255.,
                 color_rgb.1 as f32 / 255.,
                 color_rgb.2 as f32 / 255.,
             ))),
             Transform::from_xyz(
                 (top_left.0 as f32) - X_EXTENT / 2.,
                  -(top_left.1 as f32) + Y_EXTENT / 2.,
                 0.,
             ),
         ));
 
         self.entities.entities.push(rectangle_entity.id());
     }
 }
 
fn update_time(mut game_with_menu: ResMut<GameWithMenuResource>) {
    game_with_menu.0.update(Instant::now());
}
 fn draw_frame(
     mut commands: Commands,
     meshes: ResMut<Assets<Mesh>>,
     materials: ResMut<Assets<ColorMaterial>>,
     mut entities: ResMut<Entities>,
     game_with_menu: ResMut<GameWithMenuResource>,
 ) {
     println!("Draw_frame was called!");
     entities.entities.iter().for_each(|entity| {
         commands.entity(*entity).despawn();
     });
     entities.entities.clear();
     println!("Capacity: {}", entities.entities.capacity());
     let mut frame = Frame {
         commands,
         meshes,
         materials,
         entities,
     };
    game_with_menu.0.draw(&mut frame);
}