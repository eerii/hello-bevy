// Based on
// - Inspector: https://github.com/jakobhellermann/bevy-inspector-egui/blob/main/crates/bevy-inspector-egui/examples/integrations/egui_dock.rs
// - Virtual time: https://github.com/bevyengine/bevy/blob/main/examples/time/virtual_time.rs

#[allow(dead_code)]
pub struct DebugUiPlugin;

#[cfg(all(debug_assertions, feature = "inspector"))]
mod _debug {
    use std::{any::TypeId, time::Duration};

    use bevy::{
        asset::{ReflectAsset, UntypedAssetId},
        diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
        input::common_conditions::{input_just_pressed, input_pressed},
        prelude::*,
        reflect::TypeRegistry,
        render::camera::{CameraProjection, Viewport},
        time::common_conditions::on_real_timer,
        transform::TransformSystem,
        window::{PrimaryWindow, WindowResized},
    };
    use bevy_inspector_egui::{
        bevy_egui::{EguiContext, EguiContexts, EguiPlugin, EguiSet, EguiSettings},
        bevy_inspector::{
            by_type_id::{ui_for_asset, ui_for_resource},
            hierarchy::{hierarchy_ui, SelectedEntities},
            ui_for_entities_shared_components, ui_for_entity_with_children,
        },
        egui::{Context, FontData, FontDefinitions, FontFamily, Rect as ERect, Ui, WidgetText},
        DefaultInspectorConfigPlugin,
    };
    use egui_dock::{DockArea, DockState, NodeIndex, Style as EStyle, TabViewer};
    use egui_gizmo::{Gizmo, GizmoMode, GizmoOrientation};

    use crate::{
        camera::{FinalCamera, GameCamera},
        ui::*,
    };

    const INSPECTOR_OFFSET: f32 = 400.;

    // ······
    // Plugin
    // ······

    impl Plugin for super::DebugUiPlugin {
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
                    toggle_inspector.run_if(input_just_pressed(KeyCode::KeyI)),
                    toggle_pause.run_if(input_just_pressed(KeyCode::KeyP)),
                    (
                        update_fps_text,
                        update_speed_text,
                        update_ui_node,
                        change_time_speed::<1>.run_if(input_pressed(KeyCode::BracketRight)),
                        change_time_speed::<-1>.run_if(input_pressed(KeyCode::BracketLeft)),
                        change_gizmo_mode,
                    )
                        .run_if(on_real_timer(Duration::from_millis(
                            100,
                        ))),
                ),
            )
            .add_systems(
                PostUpdate,
                (
                    update_inspector
                        .run_if(
                            resource_exists::<DebugState>()
                                .and_then(|state: Res<DebugState>| state.inspector),
                        )
                        .before(EguiSet::ProcessOutput)
                        .before(TransformSystem::TransformPropagate),
                    update_camera_viewport.run_if(resource_changed::<DebugState>()),
                ),
            );
        }
    }

    // ·········
    // Resources
    // ·········

    #[derive(Resource)]
    struct DebugState {
        inspector: bool,
        state: DockState<InspectorTab>,
        viewport_rect: ERect,
        selected_entities: SelectedEntities,
        selection: InspectorSelection,
        gizmo_mode: GizmoMode,
    }

    impl Default for DebugState {
        fn default() -> Self {
            let mut state = DockState::new(vec![InspectorTab::Game]);
            let tree = state.main_surface_mut();
            let [game, nodes] = tree.split_left(NodeIndex::root(), 0.2, vec![
                InspectorTab::Nodes,
            ]);
            let [_, _] = tree.split_right(game, 0.75, vec![
                InspectorTab::Inspector,
            ]);
            let [_, _] = tree.split_below(nodes, 0.6, vec![
                InspectorTab::Resources,
                InspectorTab::Assets,
            ]);

            Self {
                inspector: false,
                state,
                selected_entities: SelectedEntities::default(),
                selection: InspectorSelection::Entities,
                viewport_rect: ERect::NOTHING,
                gizmo_mode: GizmoMode::Translate,
            }
        }
    }

    impl DebugState {
        fn render(&mut self, world: &mut World, ctx: &mut Context) {
            let mut inspector = Inspector {
                world,
                viewport_rect: &mut self.viewport_rect,
                selected_entities: &mut self.selected_entities,
                selection: &mut self.selection,
                gizmo_mode: self.gizmo_mode,
            };

            DockArea::new(&mut self.state)
                .style(EStyle::from_egui(ctx.style().as_ref()))
                .show(ctx, &mut inspector);
        }
    }

    // ··········
    // Components
    // ··········

    #[derive(Component)]
    struct FpsText;

    #[derive(Component)]
    struct SpeedText;

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

        ctx.set_fonts(fonts);
    }

    fn toggle_inspector(
        mut state: ResMut<DebugState>,
        mut win: Query<(Entity, &mut Window), With<PrimaryWindow>>,
        mut event_resize: EventWriter<WindowResized>,
    ) {
        state.inspector = !state.inspector;

        let Ok((entity, mut win)) = win.get_single_mut() else {
            return;
        };

        let (x, y) = (
            win.resolution.width(),
            win.resolution.height(),
        );
        let offset = INSPECTOR_OFFSET * if state.inspector { 1. } else { -1. };

        win.resolution.set(x + offset, y);
        event_resize.send(WindowResized {
            window: entity,
            width: x + offset,
            height: y,
        });
    }

    fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
        if time.is_paused() {
            time.unpause();
        } else {
            time.pause();
        }
    }

    fn change_time_speed<const DELTA: i8>(mut time: ResMut<Time<Virtual>>) {
        let time_speed = (time.relative_speed() + DELTA as f32 * 0.1).clamp(0.2, 5.);

        time.set_relative_speed(time_speed);
    }

    fn change_gizmo_mode(input: Res<ButtonInput<KeyCode>>, mut state: ResMut<DebugState>) {
        for (key, mode) in [
            (KeyCode::KeyR, GizmoMode::Rotate),
            (KeyCode::KeyT, GizmoMode::Translate),
            (KeyCode::KeyS, GizmoMode::Scale),
        ] {
            if input.just_pressed(key) {
                state.gizmo_mode = mode;
            }
        }
    }

    fn update_fps_text(
        mut cmd: Commands,
        diagnostics: Res<DiagnosticsStore>,
        style: Res<UiStyle>,
        node: Query<Entity, With<UiNode>>,
        mut text: Query<&mut Text, With<FpsText>>,
    ) {
        let Ok(mut text) = text.get_single_mut() else {
            let Ok(node) = node.get_single() else { return };
            let Some(mut node) = cmd.get_entity(node) else {
                return;
            };
            node.with_children(|parent| {
                UiText::new(&style, "")
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(5.),
                        bottom: Val::Px(5.),
                        ..default()
                    })
                    .with_size(16.)
                    .add_with(parent, FpsText);
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

    fn update_speed_text(
        mut cmd: Commands,
        time: Res<Time<Virtual>>,
        style: Res<UiStyle>,
        node: Query<Entity, With<UiNode>>,
        mut text: Query<&mut Text, With<SpeedText>>,
    ) {
        let Ok(mut text) = text.get_single_mut() else {
            let Ok(node) = node.get_single() else { return };
            let Some(mut node) = cmd.get_entity(node) else {
                return;
            };
            node.with_children(|parent| {
                UiText::new(&style, "")
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        right: Val::Px(5.),
                        bottom: Val::Px(5.),
                        ..default()
                    })
                    .with_size(16.)
                    .add_with(parent, SpeedText);
            });
            return;
        };

        let speed = time.relative_speed();

        text.sections[0].value = if time.is_paused() {
            "Paused".into()
        } else if speed == 1. {
            "".into()
        } else {
            format!("Speed {:.1}", speed)
        };
    }

    fn update_inspector(world: &mut World) {
        let Ok(ctx) = world
            .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
            .get_single(world)
        else {
            return;
        };
        let mut ctx = ctx.clone();

        world
            .resource_scope::<DebugState, _>(|world, mut state| state.render(world, ctx.get_mut()));
    }

    fn update_ui_node(
        mut node: Query<&mut Style, With<UiNode>>,
        win: Query<&Window, With<PrimaryWindow>>,
        cam: Query<&Camera, With<FinalCamera>>,
        mut _resize_reader: EventReader<WindowResized>,
        _state: Res<DebugState>,
        mut only_once: Local<bool>,
    ) {
        let Ok(mut style) = node.get_single_mut() else {
            return;
        };

        if !*only_once {
            let Ok(win) = win.get_single() else { return };
            let Ok(cam) = cam.get_single() else { return };

            let size = if let Some(viewport) = cam.viewport.as_ref() {
                viewport.physical_size.as_vec2() / win.scale_factor()
            } else {
                Vec2::new(win.width(), win.height())
            };

            style.width = Val::Px(size.x);
            style.height = Val::Px(size.y);
            *only_once = true;
        }

        #[cfg(feature = "resizable")]
        for e in _resize_reader.read() {
            let offset = if _state.inspector { INSPECTOR_OFFSET } else { 0. };

            style.width = Val::Px(e.width - offset);
            style.height = Val::Px(e.height);
        }
    }

    fn update_camera_viewport(
        state: Res<DebugState>,
        egui_settings: Res<EguiSettings>,
        mut cam: Query<&mut Camera, With<FinalCamera>>,
        win: Query<&mut Window, With<PrimaryWindow>>,
    ) {
        let Ok(win) = win.get_single() else { return };
        let Ok(mut cam) = cam.get_single_mut() else {
            return;
        };

        let scale_factor = win.scale_factor() * egui_settings.scale_factor as f32;

        let viewport_size = state.viewport_rect.size() * scale_factor;
        if !state.inspector || !viewport_size.x.is_normal() || !viewport_size.y.is_normal() {
            cam.viewport = None;
            return;
        }
        let viewport_pos = state.viewport_rect.left_top().to_vec2() * scale_factor;

        cam.viewport = Some(Viewport {
            physical_position: UVec2::new(
                viewport_pos.x as u32,
                viewport_pos.y as u32,
            ),
            physical_size: UVec2::new(
                viewport_size.x as u32,
                viewport_size.y as u32,
            ),
            depth: 0.0..1.0,
        });
    }

    // ·····
    // Extra
    // ·····

    #[derive(Debug)]
    enum InspectorTab {
        Game,
        Nodes,
        Resources,
        Assets,
        Inspector,
    }

    #[derive(Eq, PartialEq)]
    enum InspectorSelection {
        Entities,
        Resource(TypeId, String),
        Asset(TypeId, String, UntypedAssetId),
    }

    struct Inspector<'a> {
        world: &'a mut World,
        selected_entities: &'a mut SelectedEntities,
        selection: &'a mut InspectorSelection,
        viewport_rect: &'a mut ERect,
        gizmo_mode: GizmoMode,
    }

    impl TabViewer for Inspector<'_> {
        type Tab = InspectorTab;

        fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
            format!("{:?}", tab).into()
        }

        fn clear_background(&self, tab: &Self::Tab) -> bool {
            !matches!(tab, InspectorTab::Game)
        }

        fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
            let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
            let type_registry = type_registry.read();

            match tab {
                InspectorTab::Game => {
                    *self.viewport_rect = ui.clip_rect();
                    draw_gizmo(
                        ui,
                        self.world,
                        self.selected_entities,
                        self.gizmo_mode,
                    );
                },
                InspectorTab::Nodes => {
                    if hierarchy_ui(self.world, ui, self.selected_entities) {
                        *self.selection = InspectorSelection::Entities;
                    }
                },
                InspectorTab::Resources => select_resource(ui, &type_registry, self.selection),
                InspectorTab::Assets => select_asset(
                    ui,
                    &type_registry,
                    self.world,
                    self.selection,
                ),
                InspectorTab::Inspector => match *self.selection {
                    InspectorSelection::Entities => match self.selected_entities.as_slice() {
                        &[entity] => ui_for_entity_with_children(self.world, entity, ui),
                        entities => ui_for_entities_shared_components(self.world, entities, ui),
                    },
                    InspectorSelection::Resource(type_id, ref name) => {
                        ui.label(name);
                        ui_for_resource(
                            self.world,
                            type_id,
                            ui,
                            name,
                            &type_registry,
                        )
                    },
                    InspectorSelection::Asset(type_id, ref name, handle) => {
                        ui.label(name);
                        ui_for_asset(
                            self.world,
                            type_id,
                            handle,
                            ui,
                            &type_registry,
                        );
                    },
                },
            }
        }
    }

    fn select_resource(
        ui: &mut Ui,
        type_registry: &TypeRegistry,
        selection: &mut InspectorSelection,
    ) {
        let mut resources: Vec<_> = type_registry
            .iter()
            .filter(|registration| registration.data::<ReflectResource>().is_some())
            .map(|registration| {
                (
                    registration.type_info().type_path_table().short_path(),
                    registration.type_id(),
                )
            })
            .collect();
        resources.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));

        for (resource_name, type_id) in resources {
            let selected = match *selection {
                InspectorSelection::Resource(selected, _) => selected == type_id,
                _ => false,
            };

            if ui.selectable_label(selected, resource_name).clicked() {
                *selection = InspectorSelection::Resource(type_id, resource_name.to_string());
            }
        }
    }

    fn select_asset(
        ui: &mut Ui,
        type_registry: &TypeRegistry,
        world: &World,
        selection: &mut InspectorSelection,
    ) {
        let mut assets: Vec<_> = type_registry
            .iter()
            .filter_map(|registration| {
                let reflect_asset = registration.data::<ReflectAsset>()?;
                Some((
                    registration.type_info().type_path_table().short_path(),
                    registration.type_id(),
                    reflect_asset,
                ))
            })
            .collect();
        assets.sort_by(|(name_a, ..), (name_b, ..)| name_a.cmp(name_b));

        for (asset_name, asset_type_id, reflect_asset) in assets {
            let handles: Vec<_> = reflect_asset.ids(world).collect();

            ui.collapsing(
                format!("{asset_name} ({})", handles.len()),
                |ui| {
                    for handle in handles {
                        let selected = match *selection {
                            InspectorSelection::Asset(_, _, selected_id) => selected_id == handle,
                            _ => false,
                        };

                        if ui
                            .selectable_label(selected, format!("{:?}", handle))
                            .clicked()
                        {
                            *selection = InspectorSelection::Asset(
                                asset_type_id,
                                asset_name.to_string(),
                                handle,
                            );
                        }
                    }
                },
            );
        }
    }

    fn draw_gizmo(
        ui: &mut Ui,
        world: &mut World,
        selected_entities: &SelectedEntities,
        gizmo_mode: GizmoMode,
    ) {
        let Ok((cam_transform, projection)) = world
            .query_filtered::<(&GlobalTransform, &Projection), With<GameCamera>>()
            .get_single(world)
        else {
            return;
        };

        let view_matrix = Mat4::from(cam_transform.affine().inverse());
        let projection_matrix = projection.get_projection_matrix();

        if selected_entities.len() != 1 {
            return;
        }

        for selected in selected_entities.iter() {
            let Some(transform) = world.get::<Transform>(selected) else {
                continue;
            };
            let model_matrix = transform.compute_matrix();

            // TODO: Beautify gizmos
            // TODO: Gizmos in 2d
            // TODO: Raycast selection
            let Some(result) = Gizmo::new(selected)
                .model_matrix(model_matrix.to_cols_array_2d().into())
                .view_matrix(view_matrix.to_cols_array_2d().into())
                .projection_matrix(projection_matrix.to_cols_array_2d().into())
                .orientation(GizmoOrientation::Local)
                .mode(gizmo_mode)
                .interact(ui)
            else {
                continue;
            };

            let mut transform = world.get_mut::<Transform>(selected).unwrap();
            *transform = Transform {
                translation: Vec3::from(<[f32; 3]>::from(result.translation)),
                rotation: Quat::from_array(<[f32; 4]>::from(result.rotation)),
                scale: Vec3::from(<[f32; 3]>::from(result.scale)),
            };
        }
    }
}
