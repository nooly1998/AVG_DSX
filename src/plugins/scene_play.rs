use crate::global_def::global_define::RESOLUTION_720P;
use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::Val::Px;
use bevy::prelude::*;
use bevy::text::BreakLineOn;
use bevy_kira_audio::prelude::*;

use crate::utils::string_utils::string_auto_split;
use bevy::time::Timer;

pub struct ScenePlayPlugin;

/// A resource that holds a handle to an audio instance for controlling playback.
#[derive(Resource)]
struct InstanceHandle(Handle<AudioInstance>);



/// Represents a text component with typing effect.
///
/// # Fields
/// - `full_text`: The complete text string that will be displayed typing effect.
/// - `displayed_text`: The portion of the text currently displayed on screen.
/// - `current_index`: The index of the next character to display from the full text.
/// - `timer`: Timer to control the typing speed.
#[derive(Component, Clone)]
pub struct TypingText {
    pub(crate) full_text: String,
    pub(crate) displayed_text: String,
    pub(crate) current_index: usize,
    pub(crate) timer: Timer,
}

#[derive(Component)]
pub struct AudioPlayControl;

#[derive(Component)]
pub struct BackgroundImage;

impl Plugin for ScenePlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_entities)
            .add_plugins(AudioPlugin)
            .add_systems(Update, (update_typing_text, control_music_play));
    }
}

pub fn update_typing_text(
    time: Res<Time>,
    input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut TypingText, &mut Text)>,
) {
    for (mut typing_text, mut text) in query.iter_mut() {
        if input.just_pressed(MouseButton::Left)
            && typing_text.current_index < typing_text.full_text.len()
        {
            typing_text.displayed_text = typing_text.full_text.clone();
            typing_text.current_index = typing_text.full_text.len();

            text.sections[0].value = typing_text.displayed_text.clone();
            return;
        }

        typing_text.timer.tick(time.delta());
        if typing_text.timer.finished() && typing_text.current_index < typing_text.full_text.len() {
            let clone_text = typing_text.clone();
            let Some(update_text) = clone_text.full_text.chars().nth(clone_text.current_index)
            else {
                break;
            };
            typing_text.displayed_text.push(update_text);
            typing_text.current_index += 1;
            text.sections[0].value = typing_text.displayed_text.clone();
        }
    }
}

fn control_music_play(
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    handle: Res<InstanceHandle>,
    mut text_query: Query<&mut Text>,
    mut button_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<AudioPlayControl>),
    >,
) {
    for (button, children) in button_query.iter_mut() {
        match button {
            Interaction::Pressed => {
                println!("audio ctl button is pressed");
                if let Some(instance) = audio_instances.get_mut(&handle.0) {
                    match instance.state() {
                        PlaybackState::Playing {..} => {
                            instance.pause(AudioTween::default());
                            for &child in children.iter() {
                                if let Ok(mut text) = text_query.get_mut(child) {
                                    text.sections[0].value = "StopPlay".to_string();
                                    // 修改文本
                                }
                            }
                        }
                        PlaybackState::Paused {..} => {
                            instance.resume(AudioTween::default());
                            for &child in children.iter() {
                                if let Ok(mut text) = text_query.get_mut(child) {
                                    text.sections[0].value = "AudioPlay".to_string();
                                    // 修改文本
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

fn spawn_entities(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let font = asset_server.load("fonts/zfft.ttf");
    let background_handle = asset_server.load("images/bg1_resized.png");
    let character_handle = asset_server.load("images/ch4.png");

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
        transform: Transform::from_translation(Vec3::new(-(RESOLUTION_720P.0 / 4f32), 0.0, 1.0)),
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
            builder
                .spawn(
                    (NodeBundle {
                        style: Style { ..default() },
                        ..default()
                    }),
                )
                .with_children(|button_bundle| {
                    button_bundle
                        .spawn(TextBundle {
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
                        })
                        .insert(TypingText {
                            full_text: string_auto_split(
                                "欢迎游玩DS \n开始游戏",
                                RESOLUTION_720P.0,
                                35,
                            ),
                            displayed_text: "".to_string(),
                            current_index: 0,
                            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                        });
                });
        });

    // 加载音频文件
    let music = asset_server.load("music/bgmusic1.ogg");

    let handle = audio.play(music).looped().handle();
    commands.insert_resource(InstanceHandle(handle));
}
