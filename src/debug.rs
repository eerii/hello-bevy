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

    use bevy::{
        diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
        ecs::schedule::ScheduleLabel,
        prelude::*,
    };
    use bevy_inspector_egui::quick::WorldInspectorPlugin;

    // Add useful debug systems
    impl Plugin for super::DebugPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin {
                    wait_duration: std::time::Duration::from_secs(5),
                    ..default()
                })
                .add_plugin(WorldInspectorPlugin::default());
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
}

#[cfg(debug_assertions)]
pub use only_in_debug::*;

// Save schedule disabled function when not in debug
#[cfg(not(debug_assertions))]
pub fn save_schedule(_: &mut bevy::app::App, _: &[&'static str]) {}
