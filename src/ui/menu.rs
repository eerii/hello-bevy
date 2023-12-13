#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

use crate::{ui::*, GameOptions, GameState, Keybind, Keybinds};
use bevy::{prelude::*, reflect::Struct};
use bevy_persistent::Persistent;

// ······
// Plugin
// ······

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), init_menu)
            .add_systems(
                Update,
                (
                    handle_buttons.run_if(in_state(GameState::Menu)),
                    return_to_menu,
                ),
            )
            .add_systems(OnExit(GameState::Menu), exit_menu)
            .add_systems(
                PreUpdate,
                clean_menu.run_if(
                    in_state(GameState::Menu).and_then(
                        state_changed::<MenuState>()
                            .or_else(resource_changed::<Persistent<GameOptions>>())
                            .or_else(resource_changed::<Persistent<Keybinds>>()),
                    ),
                ),
            )
            .add_systems(OnExit(MenuState::Main), may_be_cleaned)
            .add_systems(
                Update,
                (remap_keybind, handle_buttons).run_if(in_state(MenuState::Rebinding)),
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
    Exit,
}

// ·········
// Resources
// ·········

#[derive(Resource)]
struct MenuStarting;

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

fn init_menu(
    mut cmd: Commands,
    style: Res<UIStyle>,
    mut node: Query<(Entity, &mut BackgroundColor), With<UiNode>>,
    opts: Res<Persistent<GameOptions>>,
) {
    if let Ok((node, mut bg)) = node.get_single_mut() {
        cmd.insert_resource(MenuStarting);
        *bg = opts.color.darker.into();
        layout_main(cmd, node, &style);
    }
}

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
        if let Some(mut text) = child.and_then(|child| text.get_mut(*child).ok()) {
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
                                    if let Some(value) = field.downcast_mut::<f32>() {
                                        let j = FONT_SIZES
                                            .iter()
                                            .position(|size| (*size - *value / mult).abs() < 1.5)
                                            .unwrap_or(0);

                                        *value =
                                            (FONT_SIZES[(j + 1) % FONT_SIZES.len()] * mult).round();
                                    }
                                }
                            })
                            .unwrap_or_else(|e| error!("Failed to change font size: {}", e));
                        }
                        MenuButton::ChangeColor(_, _) => {
                            // TODO: Change color (Needs either a color picker or an input field)
                        }
                    }
                }
                Interaction::Hovered => {
                    bg.0 = opts.color.mid;
                    text.sections[0].style.color = opts.color.darker;
                }
                Interaction::None => {
                    bg.0 = opts.color.light;
                    text.sections[0].style.color = opts.color.dark;
                }
            }
        }
    }
}

fn may_be_cleaned(mut cmd: Commands, menu_starting: Option<Res<MenuStarting>>) {
    if menu_starting.is_some() {
        cmd.remove_resource::<MenuStarting>();
    }
}

fn clean_menu(
    mut cmd: Commands,
    state: Res<State<MenuState>>,
    node: Query<Entity, With<UiNode>>,
    style: Res<UIStyle>,
    opts: Res<Persistent<GameOptions>>,
    keybinds: Res<Persistent<Keybinds>>,
    rebind_key: Option<Res<KeyBeingRebound>>,
    menu_starting: Option<Res<MenuStarting>>,
) {
    if menu_starting.is_some() {
        return;
    }

    if let Ok(node) = node.get_single() {
        if let Some(mut entity) = cmd.get_entity(node) {
            entity.despawn_descendants();

            match state.get() {
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
                MenuState::Exit => {}
            }
        }
    }
}

fn exit_menu(
    mut cmd: Commands,
    mut next_state: ResMut<NextState<MenuState>>,
    mut node: Query<(Entity, &mut BackgroundColor), With<UiNode>>,
) {
    if let Ok((node, mut bg)) = node.get_single_mut() {
        if let Some(mut entity) = cmd.get_entity(node) {
            entity.despawn_descendants();
        }
        *bg = Color::rgba(0., 0., 0., 0.).into();
    }

    next_state.set(MenuState::Exit);
}

fn return_to_menu(
    mut game_state: ResMut<NextState<GameState>>,
    current_menu_state: Res<State<MenuState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    input: Res<Input<Keybind>>,
    keybinds: Res<Persistent<Keybinds>>,
) {
    if keybinds.pause.iter().any(|bind| input.just_pressed(*bind)) {
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
    if let Some(rebind_key) = rebind_key.as_ref() {
        let mut bind = None;

        if let Some(key) = keyboard.get_pressed().next() {
            bind = Some(Keybind::Key(*key));
        } else if let Some(button) = mouse.get_pressed().find(|b| **b != MouseButton::Left) {
            bind = Some(Keybind::Mouse(*button));
        } else if let Some(button) = gamepad_buttons.get_pressed().next() {
            bind = Some(Keybind::Gamepad(button.button_type));
        }

        if let Some(bind) = bind {
            info!("Remapping {} to {:?}", rebind_key.0, bind);
            keybinds
                .update(|keybinds| {
                    let len = keybinds.field_len();

                    // Remove the keybind from all fields
                    for i in 0..len {
                        if let Some(field) = keybinds.field_at_mut(i) {
                            if let Some(value) = field.downcast_mut::<Vec<Keybind>>() {
                                value.retain(|b| b != &bind);
                            }
                        }
                    }

                    // Add the keybind to the field
                    if let Some(field) = keybinds.field_mut(&rebind_key.0) {
                        if let Some(value) = field.downcast_mut::<Vec<Keybind>>() {
                            value.push(bind);
                        }
                    }
                })
                .unwrap_or_else(|e| error!("Failed to remap keybind: {}", e));

            cmd.remove_resource::<KeyBeingRebound>();
            menu_state.set(MenuState::Keybinds);
        }
    }
}

// ·····
// Extra
// ·····

fn layout_main(mut cmd: Commands, node: Entity, style: &UIStyle) {
    if let Some(mut node) = cmd.get_entity(node) {
        node.with_children(|parent| {
            UIText::new(style, "Hello Bevy").with_title().add(parent);

            UIButton::new(style, "Play", MenuButton::Play).add(parent);
            UIButton::new(style, "Settings", MenuButton::GoSettings).add(parent);
        });
    }
}

fn layout_options(mut cmd: Commands, node: Entity, style: &UIStyle) {
    if let Some(mut node) = cmd.get_entity(node) {
        node.with_children(|parent| {
            UIText::new(style, "Settings").with_title().add(parent);

            UIButton::new(style, "Keybinds", MenuButton::GoKeybinds).add(parent);
            UIButton::new(style, "Visual", MenuButton::GoVisual).add(parent);

            UIButton::new(style, "Back", MenuButton::GoMain).add(parent);
        });
    }
}

fn layout_keybinds(mut cmd: Commands, node: Entity, style: &UIStyle, keybinds: &Keybinds) {
    if let Some(mut node) = cmd.get_entity(node) {
        node.with_children(|parent| {
            UIText::new(style, "Keybinds").with_title().add(parent);

            // TODO: Scrollable section (Requires #8104 to be merged in 0.13)

            for (i, value) in keybinds.iter_fields().enumerate() {
                let field_name = keybinds.name_at(i).unwrap();
                if let Some(value) = value.downcast_ref::<Vec<Keybind>>() {
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
            }

            UIButton::new(style, "Reset", MenuButton::ResetKeybinds).add(parent);

            UIButton::new(style, "Back", MenuButton::GoSettings).add(parent);
        });
    }
}

fn layout_rebinding(mut cmd: Commands, node: Entity, style: &UIStyle, key: &str) {
    if let Some(mut node) = cmd.get_entity(node) {
        node.with_children(|parent| {
            UIText::new(
                style,
                &format!("Press a key or button for {}", snake_to_upper(key)),
            )
            .add(parent);

            UIButton::new(style, "Back", MenuButton::GoKeybinds).add(parent);
        });
    }
}

fn layout_visual(mut cmd: Commands, node: Entity, style: &UIStyle, opts: &GameOptions) {
    if let Some(mut node) = cmd.get_entity(node) {
        node.with_children(|parent| {
            UIText::new(style, "Visual settings")
                .with_title()
                .add(parent);

            for (i, value) in opts.font_size.iter_fields().enumerate() {
                let field_name = opts.font_size.name_at(i).unwrap().to_string();
                if let Some(value) = value.downcast_ref::<f32>() {
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
            }

            for (i, value) in opts.color.iter_fields().enumerate() {
                let field_name = format!("color_{}", opts.color.name_at(i).unwrap());
                if let Some(value) = value.downcast_ref::<Color>() {
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
            }

            UIButton::new(style, "Back", MenuButton::GoSettings).add(parent);
        });
    }
}
