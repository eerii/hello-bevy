use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::{config::GameOptions, input::Bind, load::GameAssets};

// TODO: Input field
// TODO: Color picker

// ······
// Plugin
// ······

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UIStyle::default()).add_systems(
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
    pub title: TextStyle,
    pub text: TextStyle,
    pub button: Style,
    pub button_text: TextStyle,
    pub button_bg: BackgroundColor,
}

// ·······
// Systems
// ·······

pub fn change_style(
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

    style.button = Style {
        width: Val::Px(196.),
        height: Val::Px(48.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    style.button_text = TextStyle {
        font: assets.font.clone(),
        font_size: 24.,
        color: opts.color.dark,
    };

    style.button_bg = opts.color.light.into();
}

// ·····
// Extra
// ·····

pub fn create_title(parent: &mut ChildBuilder, style: &UIStyle, text: &str) {
    parent.spawn(TextBundle::from_section(text, style.title.clone()));
}

pub fn create_button<T: Component>(
    parent: &mut ChildBuilder,
    style: &UIStyle,
    text: &str,
    action: T,
) {
    parent
        .spawn((
            ButtonBundle {
                style: style.button.clone(),
                background_color: style.button_bg,
                ..default()
            },
            action,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(text, style.button_text.clone()));
        });
}

pub fn create_keybind_remap<T: Component>(
    parent: &mut ChildBuilder,
    style: &UIStyle,
    text: &str,
    action: T,
    bind: &[Bind],
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                min_width: Val::Px(196.),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(12.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let name = text
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_uppercase().next().unwrap()
                    } else if c == '_' {
                        ' '
                    } else {
                        c
                    }
                })
                .collect::<String>();

            parent.spawn(
                TextBundle::from_section(name, style.text.clone()).with_style(Style {
                    flex_grow: 1.,
                    ..default()
                }),
            );

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(96.),
                            ..style.button.clone()
                        },
                        background_color: style.button_bg,
                        ..default()
                    },
                    action,
                ))
                .with_children(|parent| {
                    let name = bind
                        .iter()
                        .map(|bind| bind.name())
                        .collect::<Vec<String>>()
                        .join(", ");
                    let font_size = if name.len() > 1 { 16. } else { 24. };
                    parent.spawn(TextBundle::from_section(
                        name,
                        TextStyle {
                            font: style.button_text.font.clone(),
                            font_size,
                            color: style.button_text.color,
                        },
                    ));
                });
        });
}
