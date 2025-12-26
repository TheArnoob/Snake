//! Here we use shape primitives to build meshes in a 2D rendering context, making each mesh a certain color by giving that mesh's entity a material based off a [`Color`].
//!
//! Meshes are better known for their use in 3D rendering, but we can use them in a 2D context too. Without a third dimension, the meshes we're building are flat – like paper on a table. These are still very useful for "vector-style" graphics, picking behavior, or as a foundation to build off of for where to apply a shader.
//!
//! A "shape definition" is not a mesh on its own. A circle can be defined with a radius, i.e. [`Circle::new(50.0)`][Circle::new], but rendering tends to happen with meshes built out of triangles. So we need to turn shape descriptions into meshes.
//!
//! Thankfully, we can add shape primitives directly to [`Assets<Mesh>`] because [`Mesh`] implements [`From`] for shape primitives and [`Assets<T>::add`] can be given any value that can be "turned into" `T`!
//!
//! We apply a material to the shape by first making a [`Color`] then calling [`Assets<ColorMaterial>::add`] with that color as its argument, which will create a material from that color through the same process [`Assets<Mesh>::add`] can take a shape primitive.
//!
//! Both the mesh and material need to be wrapped in their own "newtypes". The mesh and material are currently [`Handle<Mesh>`] and [`Handle<ColorMaterial>`] at the moment, which are not components. Handles are put behind "newtypes" to prevent ambiguity, as some entities might want to have handles to meshes (or images, or materials etc.) for different purposes! All we need to do to make them rendering-relevant components is wrap the mesh handle and the material handle in [`Mesh2d`] and [`MeshMaterial2d`] respectively.
//!
//! You can toggle wireframes with the space bar except on wasm. Wasm does not support
//! `POLYGON_MODE_LINE` on the gpu.

use bevy::{
    asset::load_internal_binary_asset, input::common_conditions::input_toggle_active, prelude::*,
    sprite::Text2dShadow,
};
use bevy::{
    input::common_conditions::input_just_pressed,
    sprite_render::{Wireframe2dConfig, Wireframe2dPlugin},
};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, Wireframe2dPlugin::default()))
        .add_systems(Startup, setup);
    app.add_systems(
        Update,
        toggle_wireframe.run_if(input_just_pressed(KeyCode::Space)),
    );
    app.add_systems(
        Update,
        rotate.run_if(input_toggle_active(false, KeyCode::KeyR)),
    );
    app.add_systems(Update, setup.run_if(input_just_pressed(KeyCode::AltLeft)));
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

    let shapes = [meshes.add(Rectangle::new(50.0, 100.0))];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -X_EXTENT / 2. + i as f32 / (num_shapes) as f32 * X_EXTENT,
                Y_EXTENT / 2.,
                0.0,
            ),
        ));
    }
}

fn toggle_wireframe(mut wireframe_config: ResMut<Wireframe2dConfig>) {
    wireframe_config.global = !wireframe_config.global;
}

fn rotate(mut query: Query<&mut Transform, With<Mesh2d>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_z(time.delta_secs() / 2.);
        transform.rotate_x(time.delta_secs() / 2.);
        transform.rotate_y(time.delta_secs() / 2.);
    }
}
