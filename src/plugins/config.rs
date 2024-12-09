use bevy::a11y::accesskit::Size;
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::*;
use bevy::ui::Val::Px;

pub struct ConfigPlugin;

pub(crate) struct ScreenResource {
    size: Size,
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_entities)
            .add_systems(Update, (button_system));
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        *color = match *interaction {
            Interaction::Pressed => BackgroundColor(Color::srgb(0.0, 1.0, 0.0)),
            Interaction::Hovered => BackgroundColor(Color::srgb(0.827, 0.827, 0.827)),
            Interaction::None => BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        };
    }
}

fn spawn_entities(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let options = vec!["720p", "1080p", "1440p"]; // 你的选项向量

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Px(200.0),
                display: Display::Flex,
                height: Px(100.0),
                margin: UiRect::all(Val::Px(5.0)),
                // justify_content: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor::from(Color::srgb(0.15, 0.15, 0.15)),
            ..default()
        })
        .with_children(|parent| {
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");

            for option in options {
                parent
                    .spawn(ButtonBundle {
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
                    })
                    .with_children(|drop_down| {
                        drop_down.spawn(TextBundle {
                            text: Text::from_section(
                                option,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 24.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..default()
                        });
                    });
            }
        });
}
