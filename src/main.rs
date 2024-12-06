
mod global_def;
mod utils;
mod plugins;
mod core;

use crate::global_def::global_define::RESOLUTION_720P;
use crate::plugins::scene_play::{AudioPlayControl, ScenePlayPlugin};
use crate::plugins::scroll_view::{ScrollViewPlugin, TextFiledHiddenButton};
use bevy::{
    prelude::*

    ,
    ui::*,
    window::WindowResolution
};
use bevy::prelude::Val::Px;

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
        .add_plugins((ScrollViewPlugin,ScenePlayPlugin))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let box_size = Vec2::new(RESOLUTION_720P.0, RESOLUTION_720P.1 * 0.3);
    let box_text_position = Vec2::new(-RESOLUTION_720P.0 / 3f32, 0.0);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    width: Val::Percent(box_size.x),
                    height: Val::Percent(30.0),
                    padding: UiRect::all(Px(RESOLUTION_720P.0 * 0.01)),
                    top: Px(RESOLUTION_720P.1 * 0.7 - 30.0 - 2.0 * RESOLUTION_720P.0 * 0.01),
                    ..default()
                },
                transform: Transform::from_translation(box_text_position.extend(1.0)),
                ..default()
            },
            Name::new("ButtonList"),
        ))
        .with_children(|button_list| {
            button_list
                .spawn((
                    ButtonBundle {
                        style: {
                            Style {
                                width: Px(150.0),
                                height: Px(30.0),
                                border: UiRect::all(Px(5.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            }
                        },
                        interaction: Interaction::None,
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    Name::new("TextFiledHidden"),
                    TextFiledHiddenButton,
                ))
                .with_children(|button_bundle| {
                    button_bundle.spawn(
                        (TextBundle::from_section(
                            "FiledHidden",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 18.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                            },
                        )),
                    );
                });

            button_list
                .spawn((
                    ButtonBundle {
                        style: {
                            Style {
                                width: Px(150.0),
                                height: Px(30.0),
                                border: UiRect::all(Px(5.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            }
                        },
                        interaction: Interaction::None,
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    Name::new("TextFiledHidden"),AudioPlayControl
                ))
                .with_children(|button_bundle| {
                    button_bundle.spawn(
                        (TextBundle::from_section(
                            "AudioPlay",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 18.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                            },
                        )),
                    );
                });
        });
}




