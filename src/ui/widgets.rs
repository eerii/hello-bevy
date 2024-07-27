#![allow(dead_code)]

use bevy::{color::palettes::css, ecs::system::EntityCommands, prelude::*, ui::Val::*};
use bevy_mod_picking::prelude::*;

use crate::prelude::*;

const UI_GAP: Val = Px(10.);

pub trait Widget {
    fn button(&mut self, text: impl Into<String>) -> EntityCommands;
    fn text(&mut self, text: impl Into<String>) -> EntityCommands;
}

impl<T: Spawn> Widget for T {
    fn button(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut button = self.spawn((
            NodeBundle {
                style: Style {
                    width: Px(200.),
                    height: Px(65.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: css::ROYAL_BLUE.into(),
                ..default()
            },
            NavBundle::default(),
        ));
        button.with_children(|node| {
            node.text(text).insert(Pickable::IGNORE);
        });
        button
    }

    fn text(&mut self, text: impl Into<String>) -> EntityCommands {
        self.spawn(TextBundle::from_section(text, TextStyle {
            font_size: 24.,
            color: Color::WHITE,
            ..default()
        }))
    }
}

/// An extension trait for spawning UI containers.
pub trait Container {
    fn col(&mut self) -> EntityCommands;
    fn row(&mut self) -> EntityCommands;
    fn ui_root(&mut self) -> EntityCommands;
}

impl Container for Commands<'_, '_> {
    fn col(&mut self) -> EntityCommands {
        let col = container();
        self.spawn(col)
    }

    fn row(&mut self) -> EntityCommands {
        let mut row = container();
        row.style().dir(FlexDirection::Row);
        self.spawn(row)
    }

    fn ui_root(&mut self) -> EntityCommands {
        let mut root = self.col();
        root.insert(Name::new("UI Root"));
        root
    }
}

fn container() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Percent(100.),
            height: Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: UI_GAP,
            column_gap: UI_GAP,
            position_type: PositionType::Absolute,
            ..default()
        },
        ..default()
    }
}

pub trait StyleBuilder {
    fn width(&mut self, value: Val);
    fn height(&mut self, value: Val);
    fn dir(&mut self, dir: FlexDirection);
}

impl StyleBuilder for Style {
    fn width(&mut self, value: Val) {
        self.width = value;
    }

    fn height(&mut self, value: Val) {
        self.height = value;
    }

    fn dir(&mut self, dir: FlexDirection) {
        self.flex_direction = dir;
    }
}

pub trait Stylable {
    fn style(&mut self) -> &mut Style;
}

impl Stylable for NodeBundle {
    fn style(&mut self) -> &mut Style {
        &mut self.style
    }
}

/// An internal trait for types that can spawn entities.
/// This is here so that [`Widgets`] can be implemented on all types that
/// are able to spawn entities.
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}
