use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub velocity: Velocity,
    pub rigid_body: RigidBody,
    pub friction: Friction,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(value: EntityInstance) -> Self {
        match value.identifier.as_ref() {
            "Player" => ColliderBundle {
                rigid_body: RigidBody::Dynamic,
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                ..default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

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
