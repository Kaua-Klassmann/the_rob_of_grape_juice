use bevy::prelude::*;

use crate::objects::PlayerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin).add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            image: asset_server.load("mapa.png"),
            ..default()
        },
        Transform::from_xyz(32., 32. * 4., 0.),
    ));
}
