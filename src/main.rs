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