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
        window::PrimaryWindow,
    };
    use bevy_inspector_egui::{
        bevy_egui::{
            EguiContext,
            EguiContexts,
            EguiPlugin,
        },
        bevy_inspector::hierarchy::SelectedEntities,
        egui::{
            FontData,
            FontDefinitions,
            FontFamily,
            FontId,
            ScrollArea,
            SidePanel,
            TextStyle as ETextStyle,
        },
        DefaultInspectorConfigPlugin,
    };

    use crate::ui::*;

    // ······
    // Plugin
    // ······

    impl Plugin for super::DebugUIPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
                EguiPlugin,
                DefaultInspectorConfigPlugin,
            ))
            .init_resource::<DebugState>()
            .add_systems(Startup, init_egui)
            .add_systems(
                Update,
                (
                    handle_keys,
                    update_fps,
                    update_inspector.run_if(
                        resource_exists::<DebugState>()
                            .and_then(|state: Res<DebugState>| state.inspector),
                    ),
                ),
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

    fn init_egui(mut ctx: EguiContexts) {
        let ctx = ctx.ctx_mut();

        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "sans".to_owned(),
            FontData::from_static(include_bytes!(
                "../../assets/fonts/sans.ttf"
            )),
        );

        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "sans".to_owned());

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (
                ETextStyle::Heading,
                FontId::new(13.0, FontFamily::Proportional),
            ),
            (
                ETextStyle::Body,
                FontId::new(10.0, FontFamily::Proportional),
            ),
            (
                ETextStyle::Monospace,
                FontId::new(10.0, FontFamily::Proportional),
            ),
            (
                ETextStyle::Button,
                FontId::new(10.0, FontFamily::Proportional),
            ),
            (
                ETextStyle::Small,
                FontId::new(8.0, FontFamily::Proportional),
            ),
        ]
        .into();

        ctx.set_fonts(fonts);
        ctx.set_style(style);
    }

    fn handle_keys(
        mut state: ResMut<DebugState>,
        mut _win: Query<&mut Window>,
        keys: Res<Input<KeyCode>>,
    ) {
        if keys.just_pressed(KeyCode::I) {
            state.inspector = !state.inspector;
        }

        // TODO: Resize window / Viewport
    }

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

    fn update_inspector(world: &mut World, mut selected_entities: Local<SelectedEntities>) {
        let mut egui_context = world
            .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
            .single(world)
            .clone();
        SidePanel::left("hierarchy")
            .default_width(180.)
            .show(egui_context.get_mut(), |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Hierarchy");

                    bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(
                        world,
                        ui,
                        &mut selected_entities,
                    );

                    ui.allocate_space(ui.available_size());
                });
            });

        SidePanel::right("inspector")
            .default_width(180.)
            .show(egui_context.get_mut(), |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("Inspector");

                    match selected_entities.as_slice() {
                        &[entity] => {
                            bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui);
                        },
                        entities => {
                            bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(
                                world, entities, ui,
                            );
                        },
                    }

                    ui.allocate_space(ui.available_size());
                });
            });

        // TODO: Assets & resources
    }
}
