use bevy::prelude::*;

mod components;
mod game;
mod plugins;

fn main() {
    App::new()
        .add_plugins(plugins::WindowConfigPlugin)
        .add_plugins(game::GamePlugin)
        .run();
}
