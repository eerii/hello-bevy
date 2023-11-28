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
        core_pipeline::clear_color::ClearColorConfig,
        diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
        prelude::*,
    };
    use bevy_inspector_egui::quick::WorldInspectorPlugin;

    // ······
    // Plugin
    // ······

    impl Plugin for super::DebugPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
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

    // ·········
    // Resources
    // ·········

    #[derive(Resource, Default)]
    struct DebugState {
        inspector: bool,
    }

    // ··········
    // Components
    // ··········

    #[derive(Component)]
    struct FpsText;

    // ·······
    // Systems
    // ·······

    fn init_fps(mut cmd: Commands, assets: Res<SplashAssets>) {
        cmd.spawn((Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::rgba(0., 0., 0., 0.)),
            },
            camera: Camera {
                order: -10,
                ..default()
            },
            ..default()
        },));

        cmd.spawn((
            TextBundle::from_sections([
                TextSection::from_style(TextStyle {
                    font: assets.font.clone(),
                    font_size: 16.0,
                    color: Color::WHITE,
                }),
                TextSection::from_style(TextStyle {
                    font: assets.font.clone(),
                    font_size: 16.0,
                    color: Color::GOLD,
                }),
            ]),
            FpsText,
        ));
    }

    fn update_fps(diagnostics: Res<DiagnosticsStore>, mut text: Query<&mut Text, With<FpsText>>) {
        for mut t in &mut text {
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    t.sections[0].value = format!("FPS {value:.0}");
                }
            }
            /*if let Some(frame_time) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
                if let Some(value) = frame_time.smoothed() {
                    t.sections[1].value = format!(" ({value:.2}ms)");
                }
            }*/
        }
    }

    fn handle_keys(mut state: ResMut<DebugState>, keyboard: Res<Input<KeyCode>>) {
        if keyboard.just_pressed(KeyCode::I) {
            state.inspector = !state.inspector;
        }
    }
}

// Only export this module if this is a debug build
#[cfg(debug_assertions)]
pub use only_in_debug::*;
