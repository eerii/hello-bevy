#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use bevy::{
    prelude::*,
    reflect::Struct,
};
use bevy_persistent::Persistent;

use crate::{
    ui::*,
    GameOptions,
    GameState,
    KeyBind,
    Keybinds,
};

// ······
// Plugin
// ······

pub struct MenuUIPlugin;

impl Plugin for MenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_systems(
                PreUpdate,
                (
                    clean_menu.run_if(state_changed::<GameState>()),
                    clean_menu.run_if(state_changed::<MenuState>()),
                    clean_menu.run_if(resource_changed::<Persistent<Keybinds>>()),
                    clean_menu.run_if(resource_changed::<
                        Persistent<GameOptions>,
                    >()),
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
        (
            &Interaction,
            &MenuButton,
            &Children,
            &mut BackgroundColor,
        ),
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
                    },
                    MenuButton::GoMain => {
                        menu_state.set(MenuState::Main);
                    },
                    MenuButton::GoSettings => {
                        menu_state.set(MenuState::Settings);
                    },
                    MenuButton::GoKeybinds => {
                        menu_state.set(MenuState::Keybinds);
                    },
                    MenuButton::GoVisual => {
                        menu_state.set(MenuState::Visual);
                    },
                    MenuButton::RemapKeybind(key) => {
                        menu_state.set(MenuState::Rebinding);
                        cmd.insert_resource(KeyBeingRebound(key.clone()));
                    },
                    MenuButton::ResetKeybinds => {
                        keybinds
                            .revert_to_default()
                            .unwrap_or_else(|e| error!("Failed to reset keybinds: {}", e));
                    },
                    MenuButton::ChangeFont(name) => {
                        opts.update(|opts| {
                            assert_eq!(
                                FONT_MULTIPLIERS.len(),
                                opts.font_size.field_len()
                            );
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

                                *value = (FONT_SIZES[(j + 1) % FONT_SIZES.len()] * mult).round();
                            }
                        })
                        .unwrap_or_else(|e| error!("Failed to change font size: {}", e));
                    },
                    MenuButton::ChangeColor(..) => {
                        // TODO: Change color (Needs either a color picker or an input field)
                    },
                }
            },
            Interaction::Hovered => {
                bg.0 = opts.color.mid;
                text.sections[0].style.color = opts.color.dark;
            },
            Interaction::None => {
                bg.0 = opts.color.light;
                text.sections[0].style.color = opts.color.dark;
            },
        }
    }
}

fn clean_menu(
    mut cmd: Commands,
    game_state: Res<State<GameState>>,
    menu_state: Res<State<MenuState>>,
    node: Query<Entity, With<UiNode>>,
    style: Res<UIStyle>,
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
        },
        MenuState::Visual => layout_visual(cmd, node, &style, &opts),
    }
}

fn return_to_menu(
    mut game_state: ResMut<NextState<GameState>>,
    current_menu_state: Res<State<MenuState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    input: Res<Input<KeyBind>>,
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
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    gamepad_buttons: Res<Input<GamepadButton>>,
) {
    let Some(rebind_key) = rebind_key.as_ref() else { return };
    let mut bind = None;

    if let Some(key) = keyboard.get_pressed().next() {
        bind = Some(KeyBind::Key(*key));
    } else if let Some(button) = mouse.get_pressed().find(|b| **b != MouseButton::Left) {
        bind = Some(KeyBind::Mouse(*button));
    } else if let Some(button) = gamepad_buttons.get_pressed().next() {
        bind = Some(KeyBind::Gamepad(button.button_type));
    }

    let Some(bind) = bind else { return };
    info!(
        "Remapping {} to {:?}",
        rebind_key.0, bind
    );
    keybinds
        .update(|keybinds| {
            let len = keybinds.field_len();

            // Remove the keybind from all fields
            for i in 0..len {
                let Some(field) = keybinds.field_at_mut(i) else { continue };
                let Some(value) = field.downcast_mut::<Vec<KeyBind>>() else { continue };
                value.retain(|b| b != &bind);
            }

            // Add the keybind to the field
            let Some(field) = keybinds.field_mut(&rebind_key.0) else { return };
            let Some(value) = field.downcast_mut::<Vec<KeyBind>>() else { return };
            value.push(bind);
        })
        .unwrap_or_else(|e| error!("Failed to remap keybind: {}", e));

    cmd.remove_resource::<KeyBeingRebound>();
    menu_state.set(MenuState::Keybinds);
}

// ·····
// Extra
// ·····

fn layout_main(mut cmd: Commands, node: Entity, style: &UIStyle) {
    let Some(mut node) = cmd.get_entity(node) else { return };

    node.with_children(|parent| {
        UIText::new(style, "Hello Bevy").with_title().add(parent);

        UIButton::new(style, "Play", MenuButton::Play).add(parent);
        UIButton::new(
            style,
            "Settings",
            MenuButton::GoSettings,
        )
        .add(parent);
    });
}

fn layout_options(mut cmd: Commands, node: Entity, style: &UIStyle) {
    let Some(mut node) = cmd.get_entity(node) else { return };

    node.with_children(|parent| {
        UIText::new(style, "Settings").with_title().add(parent);

        UIButton::new(
            style,
            "Keybinds",
            MenuButton::GoKeybinds,
        )
        .add(parent);
        UIButton::new(style, "Visual", MenuButton::GoVisual).add(parent);

        UIButton::new(style, "Back", MenuButton::GoMain).add(parent);
    });
}

fn layout_keybinds(mut cmd: Commands, node: Entity, style: &UIStyle, keybinds: &Keybinds) {
    let Some(mut node) = cmd.get_entity(node) else { return };

    node.with_children(|parent| {
        UIText::new(style, "Keybinds").with_title().add(parent);

        // TODO: Scrollable section (Requires #8104 to be merged in 0.13)
        for (i, value) in keybinds.iter_fields().enumerate() {
            let field_name = keybinds.name_at(i).unwrap();
            let Some(value) = value.downcast_ref::<Vec<KeyBind>>() else { continue };

            UIOption::new(style, field_name).add(parent, |row| {
                let keys = value
                    .iter()
                    .map(|bind| bind.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                UIButton::new(
                    style,
                    &keys,
                    MenuButton::RemapKeybind(field_name.to_string()),
                )
                .with_width(Val::Px(128.))
                .with_font_scale(0.7)
                .add(row);
            });
        }

        UIButton::new(
            style,
            "Reset",
            MenuButton::ResetKeybinds,
        )
        .add(parent);

        UIButton::new(style, "Back", MenuButton::GoSettings).add(parent);
    });
}

fn layout_rebinding(mut cmd: Commands, node: Entity, style: &UIStyle, key: &str) {
    let Some(mut node) = cmd.get_entity(node) else { return };

    node.with_children(|parent| {
        UIText::new(
            style,
            &format!(
                "Press a key or button for {}",
                snake_to_upper(key)
            ),
        )
        .add(parent);

        UIButton::new(style, "Back", MenuButton::GoKeybinds).add(parent);
    });
}

fn layout_visual(mut cmd: Commands, node: Entity, style: &UIStyle, opts: &GameOptions) {
    let Some(mut node) = cmd.get_entity(node) else { return };

    node.with_children(|parent| {
        UIText::new(style, "Visual settings")
            .with_title()
            .add(parent);

        for (i, value) in opts.font_size.iter_fields().enumerate() {
            let field_name = opts.font_size.name_at(i).unwrap().to_string();
            let Some(value) = value.downcast_ref::<f32>() else { continue };

            UIOption::new(style, &format!("font_{}", field_name)).add(parent, |row| {
                UIButton::new(
                    style,
                    &format!("{}", value),
                    MenuButton::ChangeFont(field_name),
                )
                .with_width(Val::Px(40.))
                .add(row);
            });
        }

        for (i, value) in opts.color.iter_fields().enumerate() {
            let field_name = format!(
                "color_{}",
                opts.color.name_at(i).unwrap()
            );
            let Some(value) = value.downcast_ref::<Color>() else { continue };

            UIOption::new(style, &field_name).add(parent, |row| {
                UIButton::new(
                    style,
                    &format!("{:.0}", value.r() * 255.),
                    MenuButton::ChangeColor(field_name.clone(), "r".to_string()),
                )
                .with_width(Val::Px(40.))
                .add(row);

                UIButton::new(
                    style,
                    &format!("{:.0}", value.g() * 255.),
                    MenuButton::ChangeColor(field_name.clone(), "g".to_string()),
                )
                .with_width(Val::Px(40.))
                .add(row);

                UIButton::new(
                    style,
                    &format!("{:.0}", value.b() * 255.),
                    MenuButton::ChangeColor(field_name.clone(), "b".to_string()),
                )
                .with_width(Val::Px(40.))
                .add(row);
            });
        }

        UIButton::new(style, "Back", MenuButton::GoSettings).add(parent);
    });
}
