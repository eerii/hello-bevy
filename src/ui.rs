mod debug;
mod loading;
mod menu;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
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
const MENU_ITEM_GAP: Val = Val::Px(10.);

pub const UI_LAYER: RenderLayers = RenderLayers::layer(10);
pub const FONT_MULTIPLIERS: [f32; 3] = [2.0, 1.0, 0.8];
pub const FONT_SIZES: [f32; 5] = [16.0, 20.0, 24.0, 28.0, 32.0];

// ······
// Plugin
// ······

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UIStyle::default())
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
            .add_plugins((
                menu::MenuUIPlugin,
                loading::LoadingUIPlugin,
            ));

        #[cfg(debug_assertions)]
        app.add_plugins(debug::DebugUIPlugin);
    }
}

// ·········
// Resources
// ·········

#[derive(Resource, Default)]
struct UIStyle {
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
    mut style: ResMut<UIStyle>,
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

struct UIText<'a> {
    text: TextBundle,
    style: &'a UIStyle,
}

impl<'a> UIText<'a> {
    fn new(style: &'a UIStyle, text: &str) -> Self {
        Self {
            text: TextBundle::from_section(text, style.text.clone()),
            style,
        }
    }

    fn with_title(mut self) -> Self {
        self.text.text.sections[0].style = self.style.title.clone();
        self
    }

    fn with_style(mut self, style: Style) -> Self {
        self.text.style = style;
        self
    }

    fn add(self, parent: &mut ChildBuilder) { parent.spawn((self.text, UI_LAYER)); }
}

// Button
// TODO: Rounded button corners (Requires #8973 to be merged in 0.13)

struct UIButton<T: Component> {
    button: ButtonBundle,
    text: TextBundle,
    action: T,
}

impl<T: Component> UIButton<T> {
    fn new(style: &UIStyle, text: &str, action: T) -> Self {
        Self {
            button: ButtonBundle {
                style: style.button.clone(),
                background_color: style.button_bg,
                ..default()
            },
            text: TextBundle::from_section(text, style.button_text.clone()),
            action,
        }
    }

    fn with_width(mut self, width: Val) -> Self {
        self.button.style.width = width;
        self
    }

    fn with_font_scale(mut self, scale: f32) -> Self {
        self.text.text.sections[0].style.font_size *= scale;
        self
    }

    fn add(self, parent: &mut ChildBuilder) {
        let _text = self.text.text.sections[0].value.clone();
        let _id = parent
            .spawn((self.button, self.action, UI_LAYER))
            .with_children(|button| {
                button.spawn((self.text, UI_LAYER));
            })
            .id();
    }
}

// Option row (label text + widget)

struct UIOption<'a> {
    row: NodeBundle,
    label: UIText<'a>,
}

impl<'a> UIOption<'a> {
    fn new(style: &'a UIStyle, label: &str) -> Self {
        Self {
            row: NodeBundle {
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
            label: UIText::new(style, &snake_to_upper(label)).with_style(Style {
                flex_grow: 1.,
                ..default()
            }),
        }
    }

    fn add(self, parent: &mut ChildBuilder, children: impl FnOnce(&mut ChildBuilder)) {
        parent.spawn((self.row, UI_LAYER)).with_children(|row| {
            self.label.add(row);
            children(row);
        });
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
