//! Widgets module
//! This extends sickle ui with custom widgets
//! It is very helpful to reduce verbosity and to group elements together easily
//! Creating a widget is done by extending UiBuilder with new custom traits,
//! providing the functions we define as chaining options for our components

use bevy::prelude::*;
use sickle_ui::prelude::*;

const BUTTON_WIDTH: Val = Val::Px(256.);
const BUTTON_HEIGHT: Val = Val::Px(64.);

const FONT_SIZE_MULT: f32 = if cfg!(feature = "pixel_perfect") { 0.5 } else { 1.0 };
const FONT_SIZE_TEXT: f32 = 36. * FONT_SIZE_MULT;
const FONT_SIZE_TITLE: f32 = 64. * FONT_SIZE_MULT;

/// Base color for UI buttons
pub(crate) const BUTTON_COLOR: Color = Color::srgb(0.3, 0.5, 0.9);

// ······
// Traits
// ······

/// Creates a text bundle with custom styling for titles and text
pub trait UiTextWidget {
    /// Append a text ui element
    fn text(&mut self, text: String, font: Handle<Font>) -> UiBuilder<Entity>;
    /// Append a text ui element, formatted like a title
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

/// Creates an image bundle
pub trait UiImageWidget {
    /// Append an image element
    fn image(&mut self, image: Handle<Image>) -> UiBuilder<Entity>;
}

impl UiImageWidget for UiBuilder<'_, Entity> {
    fn image(&mut self, image: Handle<Image>) -> UiBuilder<Entity> {
        self.spawn(ImageBundle {
            image: UiImage::new(image),
            ..default()
        })
    }
}

/// Creates a "button"
/// This is not a real bevy ui button if we are using custom navigation to avoid
/// issues with interactible parts
/// To add text, you can chain one of the text widgets we added before
pub trait UiButtonWidget {
    /// Append a button ui element
    fn button<T: Component>(
        &mut self,
        component: T,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

#[cfg(not(feature = "navigation"))]
type ButtonType = ButtonBundle;
#[cfg(feature = "navigation")]
type ButtonType = NodeBundle;

impl UiButtonWidget for UiBuilder<'_, Entity> {
    fn button<T: Component>(
        &mut self,
        component: T,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        self.container(
            (
                ButtonType {
                    style: Style {
                        width: BUTTON_WIDTH,
                        height: BUTTON_HEIGHT,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(6.0)),
                        ..default()
                    },
                    background_color: BUTTON_COLOR.into(),
                    border_color: BUTTON_COLOR.into(),
                    #[cfg(not(feature = "pixel_perfect"))]
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                #[cfg(feature = "navigation")]
                bevy_alt_ui_navigation_lite::prelude::Focusable::default(),
                component,
            ),
            spawn_children,
        )
    }
}
