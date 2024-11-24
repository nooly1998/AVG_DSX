//! Shows text rendering with moving, rotating and scaling text.
//!
//! Note that this uses [`Text2dBundle`] to display text alongside your other entities in a 2D scene.
//!
//! For an example on how to render text as part of a user interface, independent from the world
//! viewport, you may want to look at `games/contributors.rs` or `ui/text.rs`.

mod global_def;
mod utils;
use crate::utils::string_utils::*;
use crate::global_def::global_define::RESOLUTION_720P;
use bevy::window::WindowResolution;
use bevy::{
    prelude::*
    ,
    text::{BreakLineOn, Text2dBounds},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(RESOLUTION_720P.0, RESOLUTION_720P.1),
                title: "My AVG Game".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update,update_typing_text)
        // .add_systems(
        //     Update,
        //     (animate_translation, animate_rotation, animate_scale),
        // )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/zfft.ttf");
    let background_handle = asset_server.load("images/vrgoz.png");
    let character_handle = asset_server.load("images/character.png");

    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: background_handle,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0), // Position it at the center
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        texture: character_handle,
        transform: Transform {
            translation: Vec3::new(-(RESOLUTION_720P.0/ 4f32), -0.0, 1.0), // Position it at the center
            ..Default::default()
        },
        ..Default::default()
    });

    let slightly_smaller_text_style = TextStyle {
        font,
        font_size: 35.0,
        ..default()
    };
    let box_size = Vec2::new(RESOLUTION_720P.0 , RESOLUTION_720P.1*0.3);
    let box_position = Vec2::new(0.0, -RESOLUTION_720P.1/4f32);
    let box_text_position = Vec2::new(-RESOLUTION_720P.0/3f32,0.0);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(box_size.x, box_size.y)),
                ..default()
            },
            transform: Transform::from_translation(box_position.extend(2.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "",
                        slightly_smaller_text_style.clone(),
                    )],
                    justify: JustifyText::Left,
                    linebreak_behavior: BreakLineOn::AnyCharacter,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: box_size,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(box_text_position.extend(1.0)),
                ..default()
            }).insert(TypingText {
                full_text: string_auto_split("欢迎游玩DS \n(Unicode linebreaks)",30),
                displayed_text: "".to_string(),
                current_index: 0,
                timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            });
        });
}




