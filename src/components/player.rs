use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::physics::ColliderBundle;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,

    #[sprite_bundle("player.png")]
    #[bundle]
    pub sprite_bundle: SpriteBundle,

    #[from_entity_instance]
    entity_instance: EntityInstance,
}

