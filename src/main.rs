//! Shows text rendering with moving, rotating and scaling text.
//!
//! Note that this uses [`Text2dBundle`] to display text alongside your other entities in a 2D scene.
//!
//! For an example on how to render text as part of a user interface, independent from the world
//! viewport, you may want to look at `games/contributors.rs` or `ui/text.rs`.

mod global_def;
mod utils;

use crate::global_def::global_define::RESOLUTION_720P;
use crate::utils::string_utils::*;
use bevy::ui::Val::Px;
use bevy::window::WindowResolution;
use bevy::{
    prelude::*,
    text::BreakLineOn,
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
        .add_systems(Update, (update_typing_text, scroll_view_system,scroll_bar_drag_system,scroll_view_drag_system))
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
            translation: Vec3::new(-(RESOLUTION_720P.0 / 4f32), -0.0, 1.0), // Position it at the center
            ..Default::default()
        },
        ..Default::default()
    });

    let slightly_smaller_text_style = TextStyle {
        font,
        font_size: 35.0,
        ..default()
    };
    let box_size = Vec2::new(RESOLUTION_720P.0, RESOLUTION_720P.1 * 0.3);
    let box_position = Vec2::new(0.0, -RESOLUTION_720P.1 / 4f32);
    let box_text_position = Vec2::new(-RESOLUTION_720P.0 / 3f32, 0.0);

    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                width: Val::Percent(box_size.x),
                height: Val::Percent(box_size.y),
                padding: UiRect::all(Px(RESOLUTION_720P.0 * 0.01)),
                top: Px(RESOLUTION_720P.1 * 0.7),
                ..default()
            },
            background_color: BackgroundColor::from(Color::srgb(0.25, 0.25, 0.75)),
            transform: Transform::from_translation(box_position.extend(2.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "",
                        slightly_smaller_text_style.clone(),
                    )],
                    justify: JustifyText::Left,
                    linebreak_behavior: BreakLineOn::AnyCharacter,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(box_text_position.extend(1.0)),
                ..default()
            }).insert(TypingText {
                full_text: string_auto_split("欢迎游玩DS \n(Unicode linebreaks)", RESOLUTION_720P.0, 35),
                displayed_text: "".to_string(),
                current_index: 0,
                timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            });
        });

    let view_size = Vec2::new(RESOLUTION_720P.0 * 0.3, RESOLUTION_720P.1 * 0.2);
    let view_position = Vec2::new(RESOLUTION_720P.0 * 0.3, RESOLUTION_720P.1 * 0.4);
    let cover_size = Vec2::new(RESOLUTION_720P.0 * 0.3, RESOLUTION_720P.1);

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Px(view_size.x),
                height: Px(view_size.y),
                left: Px(view_position.x),
                top: Px(view_position.y),
                overflow: Overflow::clip(),
                ..default()
            },
            background_color: BackgroundColor::from(Color::srgb(0.25, 0.25, 0.75)),
            transform: Transform::from_translation(view_position.extend(2.0)),
            ..default()
        }, Name::new("scroll_view"),TextFiledHidden))
        .with_children(|builder| {
        builder.spawn((
            NodeBundle {
                style: Style {
                    width: Px(cover_size.x - 16.0),
                    height: Px(cover_size.y),
                    left: Px(16.0),
                    ..default()
                },
                ..default()
            },
            Name::new("content_view")))
            .insert(ScrollView{
                content_len:cover_size.y + view_size.y,
                view_top : view_position.y,
                view_len:view_size.y,
                bar:false,
                ..default()
            })
            .with_children(|content| {
            for i in 0..10 {
                content.spawn(TextBundle {
                    text: Text::from_section(
                        format!("Item {}", i + 1),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    ),
                    style: Style {
                        margin: UiRect::all(Px(10.0)),
                        ..default()
                    },
                    ..default()
                });
            }
        });

        });
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Px(16.0),
                height: Px(view_size.y),
                left: Px(view_position.x - 16.0),
                top: Px(view_position.y),
                ..default()
            },
            background_color: BackgroundColor::from(Color::srgb(0.9, 0.9, 0.9)),
            transform: Transform::from_translation(view_position.extend(5.0)),
            ..default()
        },Name::new("scroll_bar"),TextFiledHidden))
        .with_children(|status| {
        status.spawn((NodeBundle {
            style: Style {
                width: Px(16.0),
                height: Px(20.0),
                ..default()
            },
            transform: Transform::from_translation(view_position.extend(2.0)),
            background_color: BackgroundColor::from(Color::srgb(0.5, 0.5, 0.5)),
            ..default()
        },Name::new("scroll_status"),)).insert(ScrollView{
            parent_top: view_position.y,
            parent_len: view_size.y,
            current_top: view_position.y,
            current_len: 20.0,
            is_dragging:false,
            drag_offset:0.0,
            bar:true,
            ..default()
        });

    });


}




