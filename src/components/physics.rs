
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
