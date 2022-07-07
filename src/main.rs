use bevy::{
    core::FixedTimestep,
    math::{const_vec2, const_vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

// Paddle size/movement
// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
// The `const_vec3!` macros are needed as functions that operate on floats cannot be constant in Rust.
const PADDLE_SIZE: Vec3 = const_vec3!([120.0, 20.0, 0.0]);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 10.0;

// Walls
const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

// Colors
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                // Unlike C/C++, there's no restriction on the order of function definitions. move_paddle() can be defined later.
                .with_system(move_paddle)
        )
        .run();
}

// A paddle component
#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Collider;

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

// impl Struct ... adds some methods to Struct.
impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left => Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS),
            WallLocation::Right => Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS),
            WallLocation::Bottom => Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS),
            WallLocation::Top => Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS),
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
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
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

    // Here we're inserting a Paddle component, a SpriteBundle, and a Collider. This seems odd, why wouldn't these be grouped together into one object? Ah I see, the spawn() creates the empty entity and we're adding components. Probably still should use a bundle.
    commands
        .spawn() // Create an empty Entity and return an EntityCommands builder for it
        .insert(Paddle)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, paddle_y, 0.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Collider);

    // Walls
    commands.spawn_bundle(WallBundle::new(WallLocation::Left));
    commands.spawn_bundle(WallBundle::new(WallLocation::Right));
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom));
    commands.spawn_bundle(WallBundle::new(WallLocation::Top));
}

// I believe this function is called every frame. Updates the paddle position keeping it within the bounds if the left or right keys are pressed.
fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>, // an instance of the default Input resource under the name keyboard_input.
    mut query: Query<&mut Transform, With<Paddle>>, // an instance of a Query that references our Paddle and its Transform.
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    } else if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    } else {
        return
    }

    // Calculate the new horizontal paddle position based on player input
    let new_paddle_position = paddle_transform.translation.x + direction * PADDLE_SPEED * TIME_STEP;

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}