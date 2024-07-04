use bevy::prelude::*;
use sickle_ui::prelude::*;

const BUTTON_WIDTH: Val = Val::Px(256.);
const BUTTON_HEIGHT: Val = Val::Px(64.);
pub const BUTTON_COLOR: Color = Color::srgb(0.9, 0.5, 0.3);

const FONT_SIZE_TEXT: f32 = 32.;
const FONT_SIZE_TITLE: f32 = 48.;

// ······
// Traits
// ······

pub trait UiTextWidget {
    fn text(&mut self, text: String, font: Handle<Font>) -> UiBuilder<Entity>;
    fn title(&mut self, text: String, font: Handle<Font>) -> UiBuilder<Entity>;
}

impl UiTextWidget for UiBuilder<'_, Entity> {
    fn text(&mut self, text: String, font: Handle<Font>) -> UiBuilder<Entity> {
        self.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font,
                font_size: FONT_SIZE_TEXT,
                color: Color::WHITE,
            },
        ))
    }

    fn title(&mut self, text: String, font: Handle<Font>) -> UiBuilder<Entity> {
        self.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font,
                font_size: FONT_SIZE_TITLE,
                color: Color::WHITE,
            },
        ))
    }
}

pub trait UiButtonWidget {
    fn button<T: Component>(
        &mut self,
        component: T,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

impl UiButtonWidget for UiBuilder<'_, Entity> {
    fn button<T: Component>(
        &mut self,
        component: T,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        self.container(
            (
                ButtonBundle {
                    style: Style {
                        width: BUTTON_WIDTH,
                        height: BUTTON_HEIGHT,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: BUTTON_COLOR.into(),
                    ..default()
                },
                component,
            ),
            spawn_children,
        )
    }
}
