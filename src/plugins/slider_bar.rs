use bevy::app::{App, Startup};
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::ui::Style;
use bevy::ui::Val::Px;
use bevy::utils::default;

pub struct SliderBarPlugin;

#[derive(Clone, Component)]
struct SliderBarStage {
    pub value: f32,
    pub max_value: f32,
    pub size: Vec2,
    pub bar_color: Color,
    pub background_color: Color,
    pub ctl_color: Color,
}

impl Plugin for SliderBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_entities)
            .add_systems(Update, update_slider_plugin);
    }
}

fn update_slider_plugin(
    mut commands: Commands,
    mut entity_query: Query<(Entity, &SliderBarStage, Option<&mut Children>)>,
) {
    for (entity, stage, mut children) in entity_query.iter_mut() {
        match children {
            Some(_) => {}
            None => {
                let background = commands
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: BackgroundColor::from(stage.background_color),
                        ..default()
                    })
                    .id();

                let fill = commands
                    .spawn((NodeBundle {
                        style: Style {
                            width: Val::Percent(0.5 * 100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: stage.bar_color.into(),
                        ..default()
                    },))
                    .id();

                let slider_stage = commands
                    .spawn(NodeBundle {
                        style: Style {
                            top: Px(-stage.size.y * 0.05),
                            width: Px(stage.size.x * 0.05),
                            height: Px(stage.size.y * 1.1),
                            ..default()
                        },
                        background_color: BackgroundColor::from(stage.ctl_color),
                        ..default()
                    })
                    .id();

                println!("create plugin");
                commands
                    .entity(entity)
                    .push_children(&[background, fill, slider_stage]);
            }
        }
    }
}

fn spawn_entities(mut commands: Commands, assert_server: ResMut<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(300.0),
                height: Val::Px(30.0),
                position_type: PositionType::Absolute,
                left: Val::Px(100.0),
                top: Val::Px(200.0),
                ..default()
            },
            ..default()
        })
        .insert(SliderBarStage {
            value: 0.0,
            max_value: 100.0,
            size: Vec2::new(300.0, 30.0),
            bar_color: Color::srgb(0.25, 0.75, 0.25),
            background_color: Color::srgb(0.15, 0.15, 0.15),
            ctl_color: Color::srgb(1.0, 1.0, 1.0),
        });
}
