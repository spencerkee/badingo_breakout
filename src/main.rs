use bevy::{
    core::FixedTimestep,
    math::{const_vec2, const_vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(setup)
        .run();
}

// A paddle component
struct Paddle {
    speed: f32,
}

// Enum with 3 fields, contains types of collisions for the ball
enum Collider {
    Solid,
    Scorable,
    Paddle,
}

// only run once, since it is only a startup_system
fn setup(
    // a reference to a Commands struct. The Commands struct can be used to spawn Bundles and to add Components.
    mut commands: Commands,
    // // an instance of a collection of ColorMaterials. Material is used to paint 2D or 3D objects on screen and ColorMaterial is the simplest Material as it only supports a single color as oposed to, for example, textures.
    // mut materials: ResMut<Assets<ColorMaterial>>, 
    // an instance of AssetServer. The AssetServer will be used in the last section of this tutorial to load the font we will use to display the game score.
    asset_server: Res<AssetServer>, 
) {
    // use the commands struct to spawn (or in another word, add) a couple of Bundles. Namely we spawn an orthographic and a separate UI camera. This is done because it is usually easier to render the UI using a different camera then the one the game objects are using.
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    // Next we spawn a sprite to represent our paddle. A sprite is usually a simple little graphical 2D object, like a tree or a player character (like 2D Mario for example).
}