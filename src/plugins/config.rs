use bevy::a11y::accesskit::Size;
use bevy::app::{App, Plugin};
use bevy::prelude::*;

pub struct ConfigPlugin;

pub(crate) struct ScreenResource {
    size: Size,
}


impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_entities);
            // .add_systems(Update, );
    }
}





fn spawn_entities(mut commands: Commands, asset_server: ResMut<AssetServer>) {

}
