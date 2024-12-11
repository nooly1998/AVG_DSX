use crate::global_def::global_define::RESOLUTION_720P;
use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::Val::Px;
use bevy::prelude::*;
use bevy::text::BreakLineOn;
use bevy_kira_audio::prelude::*;

use crate::core::event_bus::*;
use crate::utils::string_utils::string_auto_split;
use bevy::time::Timer;

pub struct ScenePlayPlugin;

#[derive(Clone)]
struct ScenePlayStage {
    id: u32,
    background_path: &'static str,
    character_path: &'static str,
    music_path: &'static str,
}

/// A resource that holds a handle to an audio instance for controlling playback.
#[derive(Resource)]
struct BgmHandle(Handle<AudioInstance>);

#[derive(Resource)]
struct BgImageHandle(Handle<Image>);

// #[derive(Resource)]
// struct CharacterHandle(Handle<Image>);

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
struct CharacterComponent {
    path: &'static str,
}

#[derive(Component)]
pub struct BackgroundComponent {
    path: &'static str,
}

#[derive(Component)]
pub struct AudioPlayControl;

#[derive(Component)]
pub struct BackgroundControl;

#[derive(Component)]
pub struct CharacterControl;

impl Plugin for ScenePlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_entities)
            .init_resource::<Events<GenericEvent<ScenePlayStage>>>()
            .add_plugins(AudioPlugin)
            .add_systems(
                Update,
                (
                    update_typing_text,
                    control_music_play,
                    control_character_play,
                    control_background_play,
                    event_sender_system,
                    event_receiver_system,
                ),
            );
    }
}

fn event_sender_system(
    mut event_writer: EventWriter<GenericEvent<ScenePlayStage>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        let scene_event = GenericEvent {
            data: ScenePlayStage {
                id: 1,
                background_path: "images/bg2_resized.png",
                character_path: "images/ch5.png",
                music_path: "images/music1.mp3",
            },
        };
        event_writer.send(scene_event);
        println!("GenericEvent<ScenePlayStage> sent!");
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

fn event_receiver_system(
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<GenericEvent<ScenePlayStage>>,
    mut background_query: Query<(
        &mut Handle<Image>,
        Option<&mut BackgroundComponent>,
        Option<&mut CharacterComponent>,
    )>,
) {
    for (event, _) in event_reader.par_read() {
        for (mut handle, background, character) in &mut background_query.iter_mut() {
            if background.is_some() {
                let path = event.data.background_path;
                // background.unwrap().path = path.clone();
                *handle = asset_server.load(path);
            }
            if character.is_some() {
                let path = event.data.character_path;
                // character.unwrap().path = path.clone();
                *handle = asset_server.load(path);
            }
            {}
        }
    }
}

fn control_music_play(
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    handle: Res<BgmHandle>,
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
                        PlaybackState::Playing { .. } => {
                            instance.pause(AudioTween::default());
                            for &child in children.iter() {
                                if let Ok(mut text) = text_query.get_mut(child) {
                                    text.sections[0].value = "StopPlay".to_string();
                                    // 修改文本
                                }
                            }
                        }
                        PlaybackState::Paused { .. } => {
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

fn control_background_play(
    asset_server: Res<AssetServer>,
    mut entity_query: Query<(&mut Handle<Image>, &BackgroundComponent)>,
    mut button_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<BackgroundControl>),
    >,
) {
    for (interaction, _) in button_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                println!("background pressed button");
                for (mut handle, background) in &mut entity_query.iter_mut() {
                    let path = background.path;
                    *handle = asset_server.load(path);
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn control_character_play(
    asset_server: Res<AssetServer>,
    mut entity_query: Query<(&mut Handle<Image>, &CharacterComponent)>,
    mut button_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<CharacterControl>),
    >,
) {
    for (interaction, _) in button_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                println!("character pressed button");
                for (mut handle, character) in &mut entity_query.iter_mut() {
                    let path = character.path;
                    *handle = asset_server.load(path);
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn spawn_entities(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let font = asset_server.load("fonts/zfft.ttf");
    let background_handle = asset_server.load("images/bg2_resized.png");
    let character_handle = asset_server.load("images/ch5.png");

    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            texture: background_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0), // Position it at the center
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BackgroundComponent {
            path: "images/bg1_resized.png",
        });

    commands
        .spawn(SpriteBundle {
            texture: character_handle,
            transform: Transform::from_translation(Vec3::new(
                -(RESOLUTION_720P.0 / 4f32),
                0.0,
                1.0,
            )),
            ..Default::default()
        })
        .insert(CharacterComponent {
            path: "images/ch4.png",
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
                .spawn(NodeBundle {
                    style: Style { ..default() },
                    ..default()
                })
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
    commands.insert_resource(BgmHandle(handle));
}
