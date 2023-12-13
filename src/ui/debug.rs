#[allow(dead_code)]
pub struct DebugUIPlugin;

#[cfg(debug_assertions)]
mod only_in_debug {
    use bevy::{
        diagnostic::{
            DiagnosticsStore,
            FrameTimeDiagnosticsPlugin,
        },
        prelude::*,
    };

    use crate::{
        ui::*,
        GameState,
    };

    // ······
    // Plugin
    // ······

    impl Plugin for super::DebugUIPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::Play), init_fps)
                .add_systems(
                    Update,
                    update_fps.run_if(in_state(GameState::Play)),
                );
        }
    }

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
        assets: Res<CoreAssets>,
        fps: Query<Entity, With<FpsText>>,
    ) {
        if fps.iter().next().is_some() {
            return;
        }

        let Ok(node) = node.get_single() else { return };
        let Some(mut node) = cmd.get_entity(node) else { return };
        node.with_children(|parent| {
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

    fn update_fps(diagnostics: Res<DiagnosticsStore>, mut text: Query<&mut Text, With<FpsText>>) {
        for mut t in &mut text {
            let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) else {
                continue;
            };
            let Some(value) = fps.smoothed() else { continue };
            t.sections[0].value = format!("FPS {:.0}", value);
        }
    }
}

// Only export this module if this is a debug build
