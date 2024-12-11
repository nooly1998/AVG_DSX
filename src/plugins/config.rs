use crate::plugins::drop_down::*;
use bevy::a11y::accesskit::Size;
use bevy::app::{App, Plugin};
use bevy::prelude::*;

pub struct ConfigPlugin;

pub(crate) struct ScreenResource {
    size: Size,
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_entities)
            .add_plugins(DropDownPlugin {
                options: DropDownOptions {
                    options: vec!["720p".to_string(), "1080p".to_string(), "1440p".to_string()],
                    ..default()
                },
                ..default()
            });
        // .add_systems(Update, );
    }
}

fn spawn_entities(commands: Commands, _asset_server: ResMut<AssetServer>) {}
