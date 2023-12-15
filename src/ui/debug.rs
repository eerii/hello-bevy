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
            app.add_systems(
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

    fn update_fps(
        mut cmd: Commands,
        diagnostics: Res<DiagnosticsStore>,
        assets: Res<CoreAssets>,
        node: Query<Entity, With<UiNode>>,
        mut fps: Query<&mut Text, With<FpsText>>,
    ) {
        let Ok(mut text) = fps.get_single_mut() else {
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
                        bottom: Val::Px(5.0),
                        ..default()
                    }),
                    FpsText,
                ));
            });
            return;
        };

        let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) else {
            return;
        };
        text.sections[0].value = format!(
            "FPS {:.0}",
            fps.smoothed().unwrap_or(0.0)
        );
    }
}
