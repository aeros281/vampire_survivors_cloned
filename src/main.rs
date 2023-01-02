use bevy::{prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod systems;
mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugin(LdtkPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -2000.0),
            ..default()
        })
        .insert_resource(LevelSelection::Uid(0))
        .add_startup_system(systems::setup)
        .add_system(systems::movement)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_int_cell::<components::WallBundle>(3)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .run();
}
