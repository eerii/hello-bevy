#![allow(dead_code)]

mod debug;
mod loading;
mod menu;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    ecs::system::EntityCommands,
    prelude::*,
    render::view::RenderLayers,
};
use bevy_persistent::Persistent;

use crate::{
    CoreAssets,
    GameOptions,
    GameState,
};

const MENU_WIDTH: Val = Val::Px(300.);
const MENU_ITEM_HEIGHT: Val = Val::Px(40.);
#[allow(dead_code)]
const MENU_ITEM_GAP: Val = Val::Px(10.);

pub const UI_LAYER: RenderLayers = RenderLayers::layer(10);
pub const FONT_MULTIPLIERS: [f32; 3] = [2.0, 1.0, 0.8];
pub const FONT_SIZES: [f32; 5] = [16.0, 20.0, 24.0, 28.0, 32.0];

// ······
// Plugin
// ······

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiStyle::default())
            .add_systems(OnEnter(GameState::Loading), init_ui)
            .add_systems(
                PreUpdate,
                clean_ui.run_if(state_changed::<GameState>()),
            )
            .add_systems(
                Update,
                change_background.run_if(
                    state_changed::<GameState>().or_else(resource_changed::<
                        Persistent<GameOptions>,
                    >()),
                ),
            )
            .add_systems(
                PostUpdate,
                (change_style.run_if(resource_changed::<
                    Persistent<GameOptions>,
                >()),),
            )
            .add_plugins(loading::LoadingUiPlugin);

        #[cfg(debug_assertions)]
        app.add_plugins(debug::DebugUiPlugin);

        #[cfg(feature = "menu")]
        app.add_plugins(menu::MenuUiPlugin);
    }
}

// ·········
// Resources
// ·········

#[derive(Resource, Default)]
struct UiStyle {
    title: TextStyle,
    text: TextStyle,
    button_text: TextStyle,

    button: Style,
    button_bg: BackgroundColor,
}

// ··········
// Components
// ··········

#[derive(Component)]
struct UiCamera;

#[derive(Component)]
struct UiNode;

// ·······
// Systems
// ·······

fn init_ui(mut cmd: Commands) {
    // Ui camera
    cmd.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 10,
                ..default()
            },
            ..default()
        },
        UI_LAYER,
        UiCameraConfig { show_ui: true },
        UiCamera,
    ));

    // Main node
    cmd.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                row_gap: Val::Px(12.),
                ..default()
            },
            ..default()
        },
        UI_LAYER,
        UiNode,
    ));
}

fn change_style(
    mut style: ResMut<UiStyle>,
    opts: Res<Persistent<GameOptions>>,
    assets: Res<CoreAssets>,
) {
    style.title = TextStyle {
        font: assets.font.clone(),
        font_size: opts.font_size.title,
        color: opts.color.mid,
    };

    style.text = TextStyle {
        font: assets.font.clone(),
        font_size: opts.font_size.text,
        color: opts.color.mid,
    };

    style.button_text = TextStyle {
        font: assets.font.clone(),
        font_size: opts.font_size.button_text,
        color: opts.color.dark,
    };

    style.button = Style {
        width: MENU_WIDTH,
        height: MENU_ITEM_HEIGHT,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    style.button_bg = opts.color.light.into();
}

fn change_background(
    opts: Res<Persistent<GameOptions>>,
    state: Res<State<GameState>>,
    mut cam: Query<&mut Camera2d, With<UiCamera>>,
) {
    if let Ok(mut cam) = cam.get_single_mut() {
        cam.clear_color = match state.get() {
            GameState::Play => ClearColorConfig::None,
            _ => ClearColorConfig::Custom(opts.color.dark),
        }
    }
}

fn clean_ui(mut cmd: Commands, node: Query<Entity, With<UiNode>>) {
    let Ok(node) = node.get_single() else { return };
    let Some(mut node) = cmd.get_entity(node) else { return };
    node.despawn_descendants();
}

// ·····
// Extra
// ·····

// Text

struct UiText {
    bundle: TextBundle,
}

impl UiText {
    fn new(style: &UiStyle, text: &str) -> Self {
        Self {
            bundle: TextBundle::from_section(text, style.text.clone()),
        }
    }

    fn new_title(style: &UiStyle, text: &str) -> Self {
        Self {
            bundle: TextBundle::from_section(text, style.title.clone()),
        }
    }

    fn with_style(mut self, style: Style) -> Self {
        self.bundle.style = style;
        self
    }

    fn with_size(mut self, size: f32) -> Self {
        self.bundle.text.sections[0].style.font_size = size;
        self
    }

    fn add<'w, 's, 'b>(
        self,
        parent: &'b mut ChildBuilder<'w, 's, '_>,
    ) -> EntityCommands<'w, 's, 'b> {
        parent.spawn((self.bundle, UI_LAYER))
    }

    fn add_with<'w, 's, 'b, T: Component>(
        self,
        parent: &'b mut ChildBuilder<'w, 's, '_>,
        tag: T,
    ) -> EntityCommands<'w, 's, 'b> {
        parent.spawn((self.bundle, UI_LAYER, tag))
    }
}

// Button
// TODO: Rounded button corners (Requires #8973 to be merged in 0.13)

struct UiButton {
    bundle: ButtonBundle,
    text: UiText,
}

impl UiButton {
    fn new(style: &UiStyle, text: &str) -> Self {
        Self {
            bundle: ButtonBundle {
                style: style.button.clone(),
                background_color: style.button_bg,
                ..default()
            },
            text: UiText::new(style, text),
        }
    }

    fn with_style(mut self, style: Style) -> Self {
        self.bundle.style = style;
        self
    }

    fn add<'w, 's, 'b>(
        self,
        parent: &'b mut ChildBuilder<'w, 's, '_>,
    ) -> EntityCommands<'w, 's, 'b> {
        let mut button = parent.spawn((self.bundle, UI_LAYER));
        button.with_children(|parent| {
            self.text.add(parent);
        });
        button
    }

    fn add_with<'w, 's, 'b, T: Component>(
        self,
        parent: &'b mut ChildBuilder<'w, 's, '_>,
        tag: T,
    ) -> EntityCommands<'w, 's, 'b> {
        let mut button = parent.spawn((self.bundle, UI_LAYER, tag));
        button.with_children(|parent| {
            self.text.add(parent);
        });
        button
    }
}

// Option row (label text + widget)

struct UiOption {
    bundle: NodeBundle,
    text: UiText,
}

impl UiOption {
    fn new(style: &UiStyle, label: &str) -> Self {
        Self {
            bundle: NodeBundle {
                style: Style {
                    width: MENU_WIDTH,
                    column_gap: MENU_ITEM_GAP,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            text: UiText::new(style, label).with_style(Style {
                flex_grow: 1.,
                ..default()
            }),
        }
    }

    fn with_style(mut self, style: Style) -> Self {
        self.bundle.style = style;
        self
    }

    fn add<'w, 's, 'b>(
        self,
        parent: &'b mut ChildBuilder<'w, 's, '_>,
    ) -> EntityCommands<'w, 's, 'b> {
        let mut row = parent.spawn((self.bundle, UI_LAYER));
        row.with_children(|parent| {
            self.text.add(parent);
        });
        row
    }
}

fn snake_to_upper(text: &str) -> String {
    text.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().next().unwrap_or(c)
            } else if c == '_' {
                ' '
            } else {
                c
            }
        })
        .collect::<String>()
}
