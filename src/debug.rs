// TODO: Add more egui interfaces for debugging

// Constant that indicates if this is a debug build
#[cfg(debug_assertions)]
#[allow(dead_code)]
pub const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
#[allow(dead_code)]
pub const DEBUG: bool = false;

// Debug plugin
#[allow(dead_code)]
pub struct DebugPlugin;

// Only debug implementation
#[cfg(debug_assertions)]
mod only_in_debug {
    use bevy::{
        diagnostic::FrameTimeDiagnosticsPlugin,
        ecs::schedule::ScheduleLabel,
        prelude::*,
    };
    use bevy_inspector_egui::quick::WorldInspectorPlugin;

    use crate::GameState;

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
            .add_systems(
                Update,
                handle_keys.run_if(in_state(GameState::Play)),
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

    // ·······
    // Systems
    // ·······

    #[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
    struct ScheduleDebugGroup;

    fn handle_keys(mut state: ResMut<DebugState>, keyboard: Res<Input<KeyCode>>) {
        if keyboard.just_pressed(KeyCode::I) {
            state.inspector = !state.inspector;
        }
    }
}
