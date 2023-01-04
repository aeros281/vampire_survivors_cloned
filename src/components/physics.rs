use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub velocity: Velocity,
    pub rigid_body: RigidBody,
    pub gravity_scale: GravityScale,
    pub density: ColliderMassProperties,
    pub friction: Friction,
    pub collider: Collider,
    pub rotation_constraints: LockedAxes,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(value: EntityInstance) -> Self {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;
        match value.identifier.as_ref() {
            "Player" => ColliderBundle {
                rigid_body: RigidBody::Dynamic,
                collider: Collider::cuboid(8., 8.),
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                rotation_constraints,
                ..default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

impl From<IntGridCell> for ColliderBundle {
    fn from(cell: IntGridCell) -> Self {
        match cell.value {
            1 | 3 => ColliderBundle {
                rigid_body: RigidBody::Fixed,
                collider: Collider::cuboid(8., 8.),
                friction: Friction::new(1.0),
                ..default()
            },
            _ => ColliderBundle::default(),
        }
    }
}

