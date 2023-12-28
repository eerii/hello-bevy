#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

#[allow(dead_code)]
pub struct MenuUiPlugin;

#[cfg(feature = "menu")]
mod _menu {
    use bevy::{prelude::*, reflect::Struct};
    use bevy_persistent::Persistent;

    use crate::{input::BindSet, ui::*, GameOptions, GameState, KeyBind, Keybinds};

    // ······
    // Plugin
    // ······

    impl Plugin for super::MenuUiPlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<MenuState>()
                .add_systems(
                    PreUpdate,
                    (
                        clean_menu.run_if(state_changed::<GameState>()),
                        clean_menu.run_if(state_changed::<MenuState>()),
                        clean_menu.run_if(resource_changed::<Persistent<Keybinds>>()),
                        clean_menu.run_if(resource_changed::<Persistent<GameOptions>>()),
                        // Non short circuiting or else
                    )
                        .after(clean_ui),
                )
                .add_systems(
                    Update,
                    (
                        handle_buttons.run_if(in_state(GameState::Menu)),
                        (remap_keybind, handle_buttons).run_if(in_state(MenuState::Rebinding)),
                        return_to_menu,
                    ),
                );
        }
    }

    #[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
    enum MenuState {
        #[default]
        Main,
        Settings,
        Keybinds,
        Rebinding,
        Visual,
    }

    // ·········
    // Resources
    // ·········

    #[derive(Resource)]
    struct KeyBeingRebound(String);

    // ··········
    // Components
    // ··········

    #[derive(Component)]
    struct MenuText;

    #[derive(Component)]
    enum MenuButton {
        Play,
        GoMain,
        GoSettings,
        GoKeybinds,
        GoVisual,
        RemapKeybind(String),
        ResetKeybinds,
        ChangeFont(String),
        ChangeColor(String, String),
    }

    // ·······
    // Systems
    // ·······

    fn handle_buttons(
        mut cmd: Commands,
        mut game_state: ResMut<NextState<GameState>>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut text: Query<&mut Text>,
        mut buttons: Query<
            (&Interaction, &MenuButton, &Children, &mut BackgroundColor),
            Changed<Interaction>,
        >,
        mut opts: ResMut<Persistent<GameOptions>>,
        mut keybinds: ResMut<Persistent<Keybinds>>,
    ) {
        for (inter, button, child, mut bg) in &mut buttons {
            let child = child.iter().next();
            let Some(mut text) = child.and_then(|child| text.get_mut(*child).ok()) else {
                continue;
            };

            match inter {
                Interaction::Pressed => {
                    bg.0 = opts.color.dark;
                    text.sections[0].style.color = opts.color.light;

                    match button {
                        MenuButton::Play => {
                            game_state.set(GameState::Play);
                        }
                        MenuButton::GoMain => {
                            menu_state.set(MenuState::Main);
                        }
                        MenuButton::GoSettings => {
                            menu_state.set(MenuState::Settings);
                        }
                        MenuButton::GoKeybinds => {
                            menu_state.set(MenuState::Keybinds);
                        }
                        MenuButton::GoVisual => {
                            menu_state.set(MenuState::Visual);
                        }
                        MenuButton::RemapKeybind(key) => {
                            menu_state.set(MenuState::Rebinding);
                            cmd.insert_resource(KeyBeingRebound(key.clone()));
                        }
                        MenuButton::ResetKeybinds => {
                            keybinds
                                .revert_to_default()
                                .unwrap_or_else(|e| error!("Failed to reset keybinds: {}", e));
                        }
                        MenuButton::ChangeFont(name) => {
                            opts.update(|opts| {
                                assert_eq!(FONT_MULTIPLIERS.len(), opts.font_size.field_len());
                                for (i, mult) in FONT_MULTIPLIERS
                                    .iter()
                                    .enumerate()
                                    .take(opts.font_size.field_len())
                                {
                                    if name != opts.font_size.name_at(i).unwrap() {
                                        continue;
                                    }
                                    let field = opts.font_size.field_at_mut(i).unwrap();
                                    let Some(value) = field.downcast_mut::<f32>() else {
                                        continue;
                                    };

                                    let j = FONT_SIZES
                                        .iter()
                                        .position(|size| (*size - *value / mult).abs() < 1.5)
                                        .unwrap_or(0);

                                    *value =
                                        (FONT_SIZES[(j + 1) % FONT_SIZES.len()] * mult).round();
                                }
                            })
                            .unwrap_or_else(|e| error!("Failed to change font size: {}", e));
                        }
                        MenuButton::ChangeColor(..) => {
                            // TODO: Change color (Needs either a color picker or an input field)
                        }
                    }
                }
                Interaction::Hovered => {
                    bg.0 = opts.color.mid;
                    text.sections[0].style.color = opts.color.dark;
                }
                Interaction::None => {
                    bg.0 = opts.color.light;
                    text.sections[0].style.color = opts.color.dark;
                }
            }
        }
    }

    fn clean_menu(
        mut cmd: Commands,
        game_state: Res<State<GameState>>,
        menu_state: Res<State<MenuState>>,
        node: Query<Entity, With<UiNode>>,
        style: Res<UiStyle>,
        opts: Res<Persistent<GameOptions>>,
        keybinds: Res<Persistent<Keybinds>>,
        rebind_key: Option<Res<KeyBeingRebound>>,
    ) {
        let Ok(node) = node.get_single() else { return };
        if let Some(mut node) = cmd.get_entity(node) {
            node.despawn_descendants();
        }

        if !matches!(game_state.get(), GameState::Menu) {
            return;
        }

        match menu_state.get() {
            MenuState::Main => layout_main(cmd, node, &style),
            MenuState::Settings => layout_options(cmd, node, &style),
            MenuState::Keybinds => layout_keybinds(cmd, node, &style, &keybinds),
            MenuState::Rebinding => {
                let rebind_key = match rebind_key {
                    Some(key) => key.0.clone(),
                    None => "none".to_string(),
                };
                layout_rebinding(cmd, node, &style, &rebind_key)
            }
            MenuState::Visual => layout_visual(cmd, node, &style, &opts),
        }
    }

    fn return_to_menu(
        mut game_state: ResMut<NextState<GameState>>,
        current_menu_state: Res<State<MenuState>>,
        mut next_menu_state: ResMut<NextState<MenuState>>,
        input: Res<ButtonInput<KeyBind>>,
        keybinds: Res<Persistent<Keybinds>>,
    ) {
        if keybinds.pause.just_pressed(&input) {
            match *current_menu_state.get() {
                MenuState::Keybinds | MenuState::Visual => next_menu_state.set(MenuState::Settings),
                _ => next_menu_state.set(MenuState::Main),
            }
            game_state.set(GameState::Menu);
        }
    }

    fn remap_keybind(
        mut cmd: Commands,
        mut menu_state: ResMut<NextState<MenuState>>,
        rebind_key: Option<Res<KeyBeingRebound>>,
        mut keybinds: ResMut<Persistent<Keybinds>>,
        keyboard: Res<ButtonInput<KeyCode>>,
        mouse: Res<ButtonInput<MouseButton>>,
        gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    ) {
        let Some(rebind_key) = rebind_key.as_ref() else {
            return;
        };
        let mut bind = None;

        if let Some(key) = keyboard.get_pressed().next() {
            bind = Some(KeyBind::Key(*key));
        } else if let Some(button) = mouse.get_pressed().find(|b| **b != MouseButton::Left) {
            bind = Some(KeyBind::Mouse(*button));
        } else if let Some(button) = gamepad_buttons.get_pressed().next() {
            bind = Some(KeyBind::Gamepad(button.button_type));
        }

        let Some(bind) = bind else { return };
        info!("Remapping {} to {:?}", rebind_key.0, bind);
        keybinds
            .update(|keybinds| {
                let len = keybinds.field_len();

                // Remove the keybind from all fields
                for i in 0..len {
                    let Some(field) = keybinds.field_at_mut(i) else {
                        continue;
                    };
                    let Some(value) = field.downcast_mut::<Vec<KeyBind>>() else {
                        continue;
                    };
                    value.retain(|b| b != &bind);
                }

                // Add the keybind to the field
                let Some(field) = keybinds.field_mut(&rebind_key.0) else {
                    return;
                };
                let Some(value) = field.downcast_mut::<Vec<KeyBind>>() else {
                    return;
                };
                value.push(bind);
            })
            .unwrap_or_else(|e| error!("Failed to remap keybind: {}", e));

        cmd.remove_resource::<KeyBeingRebound>();
        menu_state.set(MenuState::Keybinds);
    }

    // ·····
    // Extra
    // ·····

    fn layout_main(mut cmd: Commands, node: Entity, style: &UiStyle) {
        let Some(mut node) = cmd.get_entity(node) else {
            return;
        };

        node.with_children(|parent| {
            UiText::new_title(style, "Hello Bevy").add(parent);
            UiButton::new(style, "Play").add_with(parent, MenuButton::Play);
            UiButton::new(style, "Settings").add_with(parent, MenuButton::GoSettings);
        });
    }

    fn layout_options(mut cmd: Commands, node: Entity, style: &UiStyle) {
        let Some(mut node) = cmd.get_entity(node) else {
            return;
        };

        node.with_children(|parent| {
            UiText::new_title(style, "Settings").add(parent);
            UiButton::new(style, "Keybinds").add_with(parent, MenuButton::GoKeybinds);
            UiButton::new(style, "Visual").add_with(parent, MenuButton::GoVisual);
            UiButton::new(style, "Back").add_with(parent, MenuButton::GoMain);
        });
    }

    fn layout_keybinds(mut cmd: Commands, node: Entity, style: &UiStyle, keybinds: &Keybinds) {
        let Some(mut node) = cmd.get_entity(node) else {
            return;
        };

        node.with_children(|parent| {
            UiText::new_title(style, "Keybinds").add(parent);

            // TODO: Scrollable section (Requires #8104 to be merged in 0.13)
            // TODO: Fix for multiple key types
            // TODO: Add key icons
            for (i, value) in keybinds.iter_fields().enumerate() {
                let field_name = keybinds.name_at(i).unwrap();
                let Some(value) = value.downcast_ref::<BindSet<KeyBind>>() else {
                    continue;
                };

                UiOption::new(style, field_name)
                    .add(parent)
                    .with_children(|row| {
                        let keys = value
                            .0
                            .iter()
                            .map(|bind| bind.to_string())
                            .collect::<Vec<_>>()
                            .join(", ");

                        UiButton::new(style, &keys)
                            .with_style(Style {
                                width: Val::Px(128.),
                                ..default()
                            })
                            .add_with(row, MenuButton::RemapKeybind(field_name.to_string()));
                    });
            }
            UiButton::new(style, "Reset").add_with(parent, MenuButton::ResetKeybinds);
            UiButton::new(style, "Back").add_with(parent, MenuButton::GoSettings);
        });
    }

    fn layout_rebinding(mut cmd: Commands, node: Entity, style: &UiStyle, key: &str) {
        let Some(mut node) = cmd.get_entity(node) else {
            return;
        };

        node.with_children(|parent| {
            UiText::new(
                style,
                &format!("Press a key or button for {}", snake_to_upper(key)),
            )
            .add(parent);

            UiButton::new(style, "Back").add_with(parent, MenuButton::GoKeybinds);
        });
    }

    fn layout_visual(mut cmd: Commands, node: Entity, style: &UiStyle, opts: &GameOptions) {
        let Some(mut node) = cmd.get_entity(node) else {
            return;
        };

        node.with_children(|parent| {
            UiText::new_title(style, "Visual settings").add(parent);

            for (i, value) in opts.font_size.iter_fields().enumerate() {
                let field_name = opts.font_size.name_at(i).unwrap().to_string();
                let Some(value) = value.downcast_ref::<f32>() else {
                    continue;
                };

                UiOption::new(style, &format!("font_{}", field_name))
                    .add(parent)
                    .with_children(|row| {
                        UiButton::new(style, &format!("{}", value))
                            .with_style(Style {
                                width: Val::Px(40.),
                                ..default()
                            })
                            .add_with(row, MenuButton::ChangeFont(field_name));
                    });
            }

            /*for (i, value) in opts.color.iter_fields().enumerate() {
                let field_name = format!(
                    "color_{}",
                    opts.color.name_at(i).unwrap()
                );
                let Some(value) = value.downcast_ref::<Color>() else { continue };

                UiOption::new(style, &field_name)
                    .add(parent)
                    .with_children(|row| {
                        // TODO: Color picker
                    });
            }*/

            UiButton::new(style, "Back").add_with(parent, MenuButton::GoSettings);
        });
    }
}
