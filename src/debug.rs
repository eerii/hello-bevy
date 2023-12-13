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
        diagnostic::{
            DiagnosticsStore,
            FrameTimeDiagnosticsPlugin,
        },
        ecs::schedule::ScheduleLabel,
        prelude::*,
    };
    use bevy_inspector_egui::quick::WorldInspectorPlugin;

    use crate::{
        ui::*,
        GameAssets,
        GameState,
    };

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

    #[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
    struct ScheduleDebugGroup;

    // ··········
    // Components
    // ··········

    #[derive(Component)]
    struct FpsText;

    // ·······
    // Systems
    // ·······

    fn init_fps(
        mut cmd: Commands,
        node: Query<Entity, With<UiNode>>,
        assets: Res<GameAssets>,
        fps: Query<Entity, With<FpsText>>,
    ) {
        if fps.iter().next().is_some() {
            return;
        }

        if let Ok(node) = node.get_single() {
            if let Some(mut entity) = cmd.get_entity(node) {
                entity.with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section("", TextStyle {
                            font: assets.font.clone(),
                            font_size: 16.0,
                            color: Color::WHITE,
                        })
                        .with_style(Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(5.0),
                            top: Val::Px(5.0),
                            ..default()
                        }),
                        FpsText,
                    ));
                });
            }
        }
    }

    fn update_fps(diagnostics: Res<DiagnosticsStore>, mut text: Query<&mut Text, With<FpsText>>) {
        for mut t in &mut text {
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    t.sections[0].value = format!("FPS {value:.0}");
                }
            }
        }
    }

    fn handle_keys(mut state: ResMut<DebugState>, keyboard: Res<Input<KeyCode>>) {
        if keyboard.just_pressed(KeyCode::I) {
            state.inspector = !state.inspector;
        }
    }

    // ·····
    // Extra
    // ·····

    // Save the scheduling graphs for system stages (disabled for wasm)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_schedule(app: &mut App) {
        use std::path::Path;

        let graph_dir = Path::new(".data").join("graphs");
        if !graph_dir.exists() {
            std::fs::create_dir_all(&graph_dir).expect("Failed to create graph directory");
        }

        // Render graph
        {
            use bevy_mod_debugdump::*;
            let dot = render_graph_dot(app, &render_graph::Settings::default());
            save_dot(dot, "RenderGraph".to_string());
        }

        // Schedule graphs
        app.world
            .resource_scope::<Schedules, _>(|world, mut schedules| {
                use bevy_mod_debugdump::schedule_graph::*;

                let ignored_ambiguities = schedules.ignored_scheduling_ambiguities.clone();
                for (label, schedule) in schedules.iter_mut() {
                    schedule.graph_mut().initialize(world);
                    schedule
                        .graph_mut()
                        .build_schedule(
                            world.components(),
                            ScheduleDebugGroup.intern(),
                            &ignored_ambiguities,
                        )
                        .unwrap();

                    let settings = Settings {
                        collapse_single_system_sets: true,
                        ..default()
                    };
                    let dot = schedule_graph_dot(schedule, world, &settings);
                    save_dot(dot, schedule_label(label));
                }
            });
    }

    #[cfg(target_arch = "wasm32")]
    pub fn save_schedule(_: &mut App) {}

    #[allow(dead_code)]
    fn schedule_label(schedule: &dyn ScheduleLabel) -> String { format!("{:?}", schedule) }

    #[allow(dead_code)]
    fn save_dot(dot: String, name: String) {
        use std::path::Path;

        let graph_dir = Path::new(".data").join("graphs");
        if !graph_dir.exists() {
            std::fs::create_dir_all(&graph_dir).expect("Failed to create graph directory");
        }

        std::fs::write(
            graph_dir.join(format!("{}.dot", name)),
            dot,
        )
        .unwrap_or_else(|e| warn!("Failed to save graph: {}", e));

        if let Err(e) = std::process::Command::new("dot")
            .arg("-Tsvg")
            .arg("-o")
            .arg(graph_dir.join(format!("{}.svg", name)))
            .arg(graph_dir.join(format!("{}.dot", name)))
            .output()
        {
            warn!("Failed to convert graph to svg: {}", e);
        }
    }
}

// Only export this module if this is a debug build
#[cfg(debug_assertions)]
pub use only_in_debug::*;
