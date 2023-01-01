use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::{Player, Wall};

const LEVEL_PATH: &str = "level/Typical_2D_platformer_example.ldtk";

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);

    let ldtk_handle = asset_server.load(LEVEL_PATH);
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        ..default()
    });
}

pub fn movement(input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in &mut query {
        let right = if input.pressed(KeyCode::D) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::A) { 1. } else { 0. };
        velocity.linvel.x = (right - left) * 200.;
        
        let top = if input.pressed(KeyCode::W) { 1. } else { 0. };
        let down = if input.pressed(KeyCode::S) { 1. } else { 0. };
        velocity.linvel.y = (top - down) * 200.;
    }
}

pub fn debug_wall(input: Res<Input<KeyCode>>, wall_query: Query<&mut Wall, With<Wall>>) {
    if input.pressed(KeyCode::T) {
        println!("Number of wall is {}", wall_query.into_iter().len());
    }
}
