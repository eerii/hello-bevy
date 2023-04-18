// Debug helpers for bevy

// Constant that indicates if this is a debug build
#[cfg(debug_assertions)]
pub const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
pub const DEBUG: bool = false;

// Debug plugin
pub struct DebugPlugin;

// Only debug implementation
#[cfg(debug_assertions)]
mod only_in_debug {
    use crate::{load::FontAssets, GameState};
    use bevy::{
        core_pipeline::clear_color::ClearColorConfig,
        diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
        ecs::schedule::ScheduleLabel,
        prelude::*,
    };
    use bevy_inspector_egui::quick::WorldInspectorPlugin;

    // Add useful debug systems
    impl Plugin for super::DebugPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(
                    WorldInspectorPlugin::default().run_if(
                        resource_exists::<DebugState>()
                            .and_then(|state: Res<DebugState>| state.inspector),
                    ),
                )
                .init_resource::<DebugState>()
                .add_systems(OnEnter(GameState::Play), init_fps)
                .add_systems(
                    Update,
                    (update_fps, handle_keys).run_if(in_state(GameState::Play)),
                );
        }
    }

    // Save the scheduling graphs for system stages (disabled for wasm)
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(dead_code)]
    pub fn save_schedule(app: &mut App, stages: &[&'static str]) {
        use bevy_mod_debugdump::*;

        if !std::path::Path::new("graphs").exists() {
            std::fs::create_dir("graphs").unwrap();
        }

        for &name in stages {
            let graph = schedule_graph_dot(
                app,
                name_to_stage(name),
                &schedule_graph::Settings::default(),
            );
            std::fs::write(
                format!("graphs/{}-schedule.dot", name.to_lowercase()),
                graph,
            )
            .unwrap();
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[allow(dead_code)]
    pub fn save_schedule(_: &mut App, _: &[&'static str]) {}

    fn name_to_stage(name: &'static str) -> &dyn ScheduleLabel {
        match name {
            "PreStartup" => &PreStartup,
            "Startup" => &Startup,
            "PostStartup" => &PostStartup,
            "PreUpdate" => &PreUpdate,
            "Update" => &Update,
            "PostUpdate" => &PostUpdate,
            "FixedUpdate" => &FixedUpdate,
            _ => panic!("Unknown stage name: {}", name),
        }
    }

    // FPS counter
    #[derive(Component)]
    struct FpsText;

    #[derive(Component)]
    struct DebugUiCam;

    fn init_fps(mut cmd: Commands, fonts: Res<FontAssets>) {
        cmd.spawn((
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::rgba(0., 0., 0., 0.)),
                },
                camera: Camera {
                    order: 10,
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
                        font: fonts.gameboy.clone(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: fonts.gameboy.clone(),
                    font_size: 24.0,
                    color: Color::GOLD,
                }),
            ]),
            FpsText,
        ));
    }

    fn update_fps(diagnostics: Res<Diagnostics>, mut text: Query<&mut Text, With<FpsText>>) {
        if let Ok(mut text) = text.get_single_mut() {
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(fps) = fps.smoothed() {
                    text.sections[1].value = format!("{fps:.0}");
                }
            }
        }
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
