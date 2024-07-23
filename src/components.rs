use bevy::prelude::*;

mod camera;
mod music;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, music::plugin));
}

pub mod prelude {
    pub use super::camera::GameCamera;
}
