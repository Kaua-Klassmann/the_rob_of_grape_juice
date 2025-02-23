use bevy::prelude::*;

#[derive(Resource)]
struct WindowConfig {
    pub title: String,
    pub resolution: (f32, f32),
}

pub struct WindowConfigPlugin;

impl Plugin for WindowConfigPlugin {
    fn build(&self, app: &mut App) {
        let config = WindowConfig {
            title: "The rob of grape juice".to_string(),
            resolution: (640., 640.),
        };

        let window_plugin = WindowPlugin {
            primary_window: Some(Window {
                title: config.title,
                resolution: config.resolution.into(),
                resizable: false,
                ..default()
            }),
            ..default()
        };

        app.add_plugins(DefaultPlugins.set(window_plugin));
    }
}
