// Debug helpers for bevy

// Constant that indicates if this is a debug build
#[cfg(debug_assertions)]
pub const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
pub const DEBUG: bool = false;

// Debug plugin
pub struct DebugPlugin;

// TODO: Add back save schedule

// Only debug implementation
#[cfg(debug_assertions)]
mod only_in_debug {
    use crate::{load::SplashAssets, GameState};
    use bevy::{
        core_pipeline::clear_color::ClearColorConfig, diagnostic::FrameTimeDiagnosticsPlugin,
        prelude::*,
    };
    use bevy_inspector_egui::quick::WorldInspectorPlugin;

    // Add useful debug systems
    impl Plugin for super::DebugPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin::default(),
                WorldInspectorPlugin::default().run_if(
                    resource_exists::<DebugState>()
                        .and_then(|state: Res<DebugState>| state.inspector),
                ),
            ))
            .init_resource::<DebugState>()
            .add_systems(OnEnter(GameState::Play), init_fps)
            .add_systems(
                Update,
                (update_fps, handle_keys).run_if(in_state(GameState::Play)),
            );
        }
    }

    // FPS counter
    #[derive(Component)]
    struct FpsText;

    #[derive(Component)]
    struct DebugUiCam;

    fn init_fps(mut cmd: Commands, assets: Res<SplashAssets>) {
        cmd.spawn((
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::rgba(0., 0., 0., 0.)),
                },
                camera: Camera {
                    order: -10,
                    ..default()
                },
                ..default()
            },
            DebugUiCam,
        ));

        cmd.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: assets.font.clone(),
                    font_size: 24.0,
                    color: Color::GOLD,
                }),
            ]),
            FpsText,
        ));
    }

    fn update_fps(mut text: Query<&mut Text, With<FpsText>>) {
        // TODO: Fix this
    }

    // State for debug
    #[derive(Resource, Default)]
    struct DebugState {
        inspector: bool,
    }

    // Handle keys
    fn handle_keys(mut state: ResMut<DebugState>, keyboard: Res<Input<KeyCode>>) {
        if keyboard.just_pressed(KeyCode::I) {
            state.inspector = !state.inspector;
        }
    }
}

#[cfg(debug_assertions)]
pub use only_in_debug::*;

// Save schedule disabled function when not in debug
#[cfg(not(debug_assertions))]
pub fn save_schedule(_: &mut bevy::app::App, _: &[&'static str]) {}
