//! Ui module

use bevy::prelude::*;
use sickle_ui::{prelude::*, SickleUiPlugin};

use crate::{camera::FinalCamera, GameState};

#[cfg(feature = "loading")]
pub mod loading;
#[cfg(feature = "menu")]
pub mod menu;
#[cfg(feature = "navigation")]
pub mod navigation;
#[cfg(feature = "tts")]
pub mod tts;
pub mod widgets;

// ······
// Plugin
// ······

/// Ui
/// Uses bevy's Ui and Sickle to create beautiful interfaces
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SickleUiPlugin)
            .add_systems(OnExit(GameState::Startup), init);

        #[cfg(feature = "loading")]
        app.add_plugins(loading::LoadingScreenPlugin);

        #[cfg(feature = "menu")]
        app.add_plugins(menu::MenuPlugin);

        #[cfg(feature = "navigation")]
        app.add_plugins(navigation::NavigationPlugin);

        #[cfg(feature = "tts")]
        app.add_plugins(tts::SpeechPlugin);
    }
}

// ··········
// Components
// ··········

/// Marker for the ui root container
/// Everything ui related should be a child of this
/// Uses Sickle to provide greater flexibility and ease of use
#[derive(Component)]
struct UiRootContainer;

// ·······
// Systems
// ·······

/// Create a new input manager if there are no others
fn init(mut cmd: Commands, camera: Query<Entity, With<FinalCamera>>) {
    let Ok(camera) = camera.get_single() else { return };

    // Ui Root
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
