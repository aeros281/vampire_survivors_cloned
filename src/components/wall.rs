use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::physics::ColliderBundle;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,

    #[from_int_grid_cell]
    #[bundle]
    pub collider_bundle: ColliderBundle,
}
