use bevy::prelude::*;

use crate::plugins::PlayerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin);
    }
}
