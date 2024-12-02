
mod global_def;
mod utils;
mod plugins;

use crate::global_def::global_define::RESOLUTION_720P;
use crate::plugins::scene_play::ScenePlayPlugin;
use crate::plugins::scroll_view::ScrollViewPlugin;
use bevy::{
    prelude::*

    ,
    ui::*,
    window::WindowResolution
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
        // .add_systems(Startup, setup)
        .add_plugins((ScrollViewPlugin,ScenePlayPlugin))
        .run();
}

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {}




