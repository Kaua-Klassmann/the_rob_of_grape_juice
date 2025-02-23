use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);
