use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

mod components;
mod config;
mod game;
mod objects;

fn main() {
    App::new()
        .add_plugins(config::WindowConfigPlugin)
        .add_plugins(game::GamePlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(FpsTimer(Timer::from_seconds(1., TimerMode::Repeating)))
        .add_systems(Update, print_fps)
        .run();
}

#[derive(Resource)]
struct FpsTimer(Timer);

fn print_fps(time: Res<Time>, mut timer: ResMut<FpsTimer>, diagnostics: Res<DiagnosticsStore>) {
    if timer.0.tick(time.delta()).just_finished() {
        if let Some(fps) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|d| d.smoothed())
        {
            println!("FPS: {:.2}", fps);
        }
    }
}
