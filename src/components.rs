//! Some common components and their associated systems.

use bevy::sprite::MaterialMesh2dBundle;

use crate::prelude::*;

pub mod camera;
pub mod music;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, music::plugin))
        .add_systems(OnEnter(GameState::Play), init);
}

/// The prelude for this module.
pub mod prelude {
    pub use super::camera::{FinalCamera, GameCamera};
}

/// Spawn test mesh (DELETE)
fn init(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color: Color = css::SEA_GREEN.into();
    cmd.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Capsule2d::default()).into(),
            transform: Transform::from_xyz(0., 0., 2.).with_scale(Vec3::splat(32.)),
            material: materials.add(color),
            ..default()
        },
        StateScoped(GameState::Play),
    ));
}
