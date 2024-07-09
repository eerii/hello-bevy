use bevy::prelude::*;
use sickle_ui::{prelude::*, SickleUiPlugin};

use crate::camera::FinalCamera;

#[cfg(feature = "menu")]
pub mod menu;
#[cfg(feature = "navigation")]
pub mod navigation;
pub mod widgets;

// ······
// Plugin
// ······

// Ui
// Uses bevy's Ui and Sickle to create beautiful interfaces
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SickleUiPlugin).add_systems(Startup, init);

        #[cfg(feature = "menu")]
        app.add_plugins(menu::MenuPlugin);

        #[cfg(feature = "navigation")]
        app.add_plugins(navigation::NavigationPlugin);
    }
}

// ··········
// Components
// ··········

// Marker for the ui root container
#[derive(Component)]
pub struct UiRootContainer;

// ·······
// Systems
// ·······

// Create a new input manager if there are no others
fn init(mut cmd: Commands, camera: Query<Entity, With<FinalCamera>>) {
    let Ok(camera) = camera.get_single() else {
        return;
    };

    // Ui Root
    // This is the main Ui root, everything should be a child of it
    // Uses Sickle to provide greater flexibility and ease of use
    cmd.ui_builder(UiRoot).container(
        (
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            TargetCamera(camera),
            UiRootContainer,
        ),
        |_container| {},
    );
}
