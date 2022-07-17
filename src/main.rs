#![allow(dead_code, unused)] // TODO remove this
use bevy::{
    core::FixedTimestep,
    math::{const_vec2, const_vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    winit::WinitSettings,
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
const PADDLE_Y: f32 = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

// Wall constants
const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

// Ball constants
// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = const_vec3!([0.0, -50.0, 1.0]);
const BALL_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = const_vec2!([0.5, -0.5]);

// Colors
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

// Buttons
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// Meta
const COMPONENTS: &'static [&'static str] = &["Sprite", "Transform", "Collider", "Velocity", "Controllable", "Destructor", "Destructable"];

// pub struct HelloPlugin;
// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut App) {
//         // add things to your app here
//     }
// }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // // Only run the app when there is user input. This will significantly reduce CPU/GPU use but be unplayable.
        // .insert_resource(WinitSettings::desktop_app())
        // .add_plugin(HelloPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_controllable)
                // .with_system(button_system)
        )
        .run();
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Controllable;

// https://stackoverflow.com/questions/33485203/what-does-missing-lifetime-specifier-mean-when-storing-a-str-in-a-structure
// https://www.educative.io/answers/what-are-generic-lifetime-parameters-in-a-rust-function good lifetimes explanation
// struct SpawnHelper {
//     parent: Commands,
//     asset_server: Res<AssetServer>,
// }
// struct SpawnHelper {
//     commands: Commands,
//     asset_server: Res<AssetServer>,
// }
// struct SpawnHelper<'w, 'a, 's, 'd> {
//     commands: &'a mut Commands<'w, 's>,
//     asset_server: Res<'d, AssetServer>,
// }
// impl SpawnHelper<'_, '_, '_, '_> {
//     fn add_button(&mut self, text_value:String) {
//         // Buttons
//         self.commands
//             .spawn_bundle(ButtonBundle {
//                 style: Style {
//                     // flex_direction: FlexDirection::Row,
//                     size: Size::new(Val::Px(100.0), Val::Px(40.0)),
//                     // center button
//                     margin: Rect::all(Val::Auto),
//                     // horizontally center child text
//                     justify_content: JustifyContent::Center,
//                     // vertically center child text
//                     align_items: AlignItems::FlexEnd,
//                     ..default()
//                 },
//                 color: NORMAL_BUTTON.into(),
//                 ..default()
//             })
//             .with_children(|parent| {
//                 parent.spawn_bundle(TextBundle {
//                     text: Text::with_section(
//                         text_value,
//                         TextStyle {
//                             font: self.asset_server.load("fonts/FiraSans-Bold.ttf"),
//                             font_size: 18.0,
//                             color: Color::rgb(0.9, 0.9, 0.9),
//                         },
//                         Default::default(),
//                     ),
//                     ..default()
//                 });
//             });
//     }
// }

// asset_server: Res<AssetServer>, 
fn custom_add_button(parent:&mut ChildBuilder, button_text:String, color_value: f32) {
    parent.spawn_bundle(ButtonBundle {
        style: Style {
            // flex_direction: FlexDirection::Row,
            size: Size::new(Val::Px(100.0), Val::Px(40.0)),
            // center button
            // margin: Rect::all(Val::Auto),
            margin: Rect {left: Val::Auto, right: Val::Auto, bottom: Val::Px(0.0), top: Val::Px(0.0)},
            // // horizontally center child text
            // justify_content: JustifyContent::Center,
            // // vertically center child text
            // align_items: AlignItems::FlexEnd,
            ..default()
        },
        // color: NORMAL_BUTTON.into(),
        color: UiColor(Color::rgb(color_value, color_value, color_value)),
        ..default()
    });
    // .with_children(|parent| {
    //     parent.spawn_bundle(TextBundle {
    //         text: Text::with_section(
    //             button_text,
    //             TextStyle {
    //                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //                 font_size: 18.0,
    //                 color: Color::rgb(0.9, 0.9, 0.9),
    //             },
    //             Default::default(),
    //         ),
    //         ..default()
    //     });
    // });
}

// fn root(materials: &Res<MenuMaterials>) -> NodeBundle {
//     NodeBundle {
//         style: Style {
//             size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//             justify_content: JustifyContent::Center,
//             align_items: AlignItems::Center,
//             ..Default::default()
//         },
//         material: materials.root.clone(),
//         ..Default::default()
//     }
// }

// only run once, since it is only a startup_system
fn setup(
    // a reference to a Commands struct. The Commands struct can be used to spawn Bundles and to add Components.
    mut commands: Commands,
    // // an instance of a collection of ColorMaterials. Material is used to paint 2D or 3D objects on screen and ColorMaterial is the simplest Material as it only supports a single color as oposed to, for example, textures.
    // mut materials: ResMut<Assets<ColorMaterial>>, 
    // an instance of AssetServer. The AssetServer will be used in the last section of this tutorial to load the font we will use to display the game score.
    asset_server: Res<AssetServer>, 
) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // If I have commands: commands, I get mismatched types expected `&mut bevy::prelude::Commands<'_, '_>`, found struct `bevy::prelude::Commands`rustcE0308 main.rs(556, 19): consider mutably borrowing here: `&mut commands`
    
    // struct SpawnHelper {
    //     commands: Commands,
    //     asset_server: Res<AssetServer>,
    // }

    commands
        // root node
        .spawn_bundle(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                // align_items: AlignItems::FlexEnd,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    // Aligns columns to top
                    align_self: bevy::ui::AlignSelf::FlexEnd,
                    align_items: bevy::ui::AlignItems::FlexEnd,
                    // // Defines how flexbox items are ordered within a flexbox
                    // flex_direction: bevy::ui::FlexDirection::Column,
                    // The lower the padding, the farther apart the buttons get? 
                    // margin: Rect::all(Val::Px(200.0)),
                    // The padding of the node
                    // padding: Rect::all(Val::Px(1.0)),
                    size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                    // Spaces columns apart
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                // color: Color::NONE.into(),
                color: UiColor(Color::rgb(0.5, 0.5, 0.5)),
                ..default()
            })
            .with_children(|parent| {
                for x in 0..5 {
                    parent.spawn_bundle(
                        NodeBundle {
                            style: Style {
                                flex_direction: bevy::ui::FlexDirection::Column,
                                justify_content: JustifyContent::SpaceBetween,
                                // 0% by default
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        }
                    ).with_children(|parent| {
                        let mut y: i32 = 4;
                        for component in COMPONENTS {
                            custom_add_button(parent, component.to_string(), (y as f32)/15.);
                            y += 1;
                        }
                    });
                }
            });
        });



    
    // for component in COMPONENTS {
    //     spawn_helper.add_button(component.to_string());
    // }    for component in COMPONENTS {
    //     spawn_helper.add_button(component.to_string());
    // }    for component in COMPONENTS {
    //     spawn_helper.add_button(component.to_string());
    // }    for component in COMPONENTS {
    //     spawn_helper.add_button(component.to_string());
    // }    for component in COMPONENTS {
    //     spawn_helper.add_button(component.to_string());
    // }    for component in COMPONENTS {
    //     spawn_helper.add_button(component.to_string());
    // }    for component in COMPONENTS {
    //     spawn_helper.add_button(component.to_string());
    // }    for component in COMPONENTS {
    //     spawn_helper.add_button(component.to_string());
    // }    for component in COMPONENTS {
    //     spawn_helper.add_button(component.to_string());
    // }    for component in COMPONENTS {
    //     spawn_helper.add_button(component.to_string());
    // }    for component in COMPONENTS {
    //     spawn_helper.add_button(component.to_string());
    // }    for component in COMPONENTS {
    //     spawn_helper.add_button(component.to_string());
    // }

    // Next we spawn a sprite to represent our paddle. A sprite is usually a simple little graphical 2D object, like a tree or a player character (like 2D Mario for example).
    // The spawn() creates the empty entity and we're adding components. Probably still should use a bundle. Here we're inserting a Paddle component, a SpriteBundle, t
    commands
        .spawn() // Create an empty Entity and return an EntityCommands builder for it
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, PADDLE_Y, 0.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Collider)
        .insert(Controllable);

}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

// I believe this function is called every frame. Updates the paddle position keeping it within the bounds if the left or right keys are pressed.
fn move_controllable(
    keyboard_input: Res<Input<KeyCode>>, // an instance of the default Input resource under the name keyboard_input.
    mut query: Query<&mut Transform, With<Controllable>>, // an instance of a Query that references our Paddle and its Transform.
) {
    let mut entity_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    } else if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    } else {
        return
    }

    // Calculate the new horizontal paddle position based on player input
    let new_entity_position = entity_transform.translation.x + direction * PADDLE_SPEED * TIME_STEP;

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

    entity_transform.translation.x = new_entity_position.clamp(left_bound, right_bound);
}

/* 
When do you want to specify a builder? In the breakout example they do so for wall bundle because it's created 4 times and has some logic for setting the position.
In my case I'm just creating a single paddle, but what if I wanted to add more? Putting it in a function also keeps the setup function cleaner.
A bundle is a statically typed collection of components (can also use Dynamic Bundle)

I want to parameterize the creation of all these things for e.g. multiple paddles or balls. But I can try that afterwards.

So if I make PaddleBundle made up of sprite collider and controllable components, is that compromising?

Let's assume that through the ui elements I have at the very least the list of components that each entity type should have. Let's assume that these components are dumb and don't contain anything. Then the setup function would be responsible for adding/spawning these components according to parameterized rules. 

Do I want to have a reset button or dynamic components? In either case I need to have a reset button. So I guess I can start there. How does that system work? You press reset, there's some system that I guess is checking every frame if the button is pressed, then it mutates things. That's for a single button though, in my case how do I correspond the button to the feature I want to change? As a hack I can access the text of the button, just have a mapping.

Ah wait I do need to define ball brick etc components.

Okay in general how to I define dynamic components? 

+--------------+------+--------+-------+--------+-------+
|              | Ball | Paddle | Wall  | Brick  | Score |
+--------------+------+--------+-------+--------+-------+
| Sprite       | Y    | Y      | Y     | Y      | Y     |
+--------------+------+--------+-------+--------+-------+
| Transform    | Y    | Y      | Y     | Y      | Y     |
+--------------+------+--------+-------+--------+-------+
| Collider     | Y    | Y      | Y     | Y      | N     |
+--------------+------+--------+-------+--------+-------+
| Velocity     | Y    | N      | N     | N      | N     |
+--------------+------+--------+-------+--------+-------+
| Controllable | N    | Y      | N     | N      | N     |
+--------------+------+--------+-------+--------+-------+
| Destructor   | Y    | N      | N     | N      | N     |
+--------------+------+--------+-------+--------+-------+
| Destructable | N    | N      | N     | Y      | N     |

7/10/2022
-config file for defining entities. config doesn't have an entry per entity but has the constants needed like the initial positions, velocities, and sizes of the entities
-setup function that reads the config file on startup and creates the entities
-setup system that creates buttons from the config as well and stores their ids.

config {
    "balls": [
        1: [Velocity(Vec2)]
    ]
}

could edit the config in place. what's easier if I just have meta components?

can take or leave the startup config...then button system just queries for those meta components, and removes them according to the builders. ah but it needs to remember the ones it removes. 
it does so by keeping a hashmap indexed by entity id
if something despawns remove from hashmap, not too necessary.

i have to have meta components anyway, this isn't space efficient but i can start with it and see about removing them later.

state {
    123: {
        Velocity: Velocity(Vec2)
    }
}

List of gotchas
* https://stackoverflow.com/questions/70919554/is-the-default-font-in-bevy-unusable 2022-07-12T05:12:28.964203Z  WARN bevy_asset::asset_server: encountered an error while reading an asset: path not found: C:\Users\spenc\Documents\rust_stuff\badingo\assets\fonts/FiraSans-Bold.ttf https://github.com/mozilla/Fira/blob/master/ttf/FiraSans-Bold.ttf

*/
