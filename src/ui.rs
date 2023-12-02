use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::{config::GameOptions, load::GameAssets, GameState};

const MENU_WIDTH: Val = Val::Px(300.);
const MENU_ITEM_HEIGHT: Val = Val::Px(40.);
const MENU_ITEM_GAP: Val = Val::Px(10.);

// TODO: Tweening and animation (Look into https://github.com/djeedai/bevy_tweening)
// TODO: Rounded button corners (Requires #8973 to be merged in 0.13)

// ······
// Plugin
// ······

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UIStyle::default())
            .add_systems(OnEnter(GameState::Loading), init_ui)
            .add_systems(
                PostUpdate,
                change_style.run_if(resource_changed::<Persistent<GameOptions>>()),
            );
    }
}

// ·········
// Resources
// ·········

#[derive(Resource, Default)]
pub struct UIStyle {
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
pub struct UiCam;

#[derive(Component)]
pub struct UiNode;

// ·······
// Systems
// ·······

pub fn init_ui(mut cmd: Commands) {
    cmd.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 10,
                ..default()
            },
            ..default()
        },
        UiCam,
    ));

    cmd.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(12.),
                ..default()
            },
            ..default()
        },
        UiNode,
    ));
}

fn change_style(
    mut style: ResMut<UIStyle>,
    opts: Res<Persistent<GameOptions>>,
    assets: Res<GameAssets>,
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

// ·····
// Extra
// ·····

// Text

pub struct UIText<'a> {
    text: TextBundle,
    style: &'a UIStyle,
}

impl<'a> UIText<'a> {
    pub fn new(style: &'a UIStyle, text: &str) -> Self {
        Self {
            text: TextBundle::from_section(text, style.text.clone()),
            style,
        }
    }

    pub fn with_title(mut self) -> Self {
        self.text.text.sections[0].style = self.style.title.clone();
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.text.style = style;
        self
    }

    pub fn add(self, parent: &mut ChildBuilder) {
        parent.spawn(self.text);
    }
}

// Button

pub struct UIButton<T: Component> {
    button: ButtonBundle,
    text: TextBundle,
    action: T,
}

impl<T: Component> UIButton<T> {
    pub fn new(style: &UIStyle, text: &str, action: T) -> Self {
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

    pub fn with_width(mut self, width: Val) -> Self {
        self.button.style.width = width;
        self
    }

    pub fn with_font_scale(mut self, scale: f32) -> Self {
        self.text.text.sections[0].style.font_size *= scale;
        self
    }

    pub fn add(self, parent: &mut ChildBuilder) {
        let _text = self.text.text.sections[0].value.clone();
        let _id = parent
            .spawn((self.button, self.action))
            .with_children(|button| {
                button.spawn(self.text);
            })
            .id();
    }
}

// Option row (label text + widget)

pub struct UIOption<'a> {
    row: NodeBundle,
    label: UIText<'a>,
}

impl<'a> UIOption<'a> {
    pub fn new(style: &'a UIStyle, label: &str) -> Self {
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

    pub fn add(self, parent: &mut ChildBuilder, children: impl FnOnce(&mut ChildBuilder)) {
        parent.spawn(self.row).with_children(|row| {
            self.label.add(row);
            children(row);
        });
    }
}

pub fn snake_to_upper(text: &str) -> String {
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
