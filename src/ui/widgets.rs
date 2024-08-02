//! Reusable Ui widgets to easily build interfaces.

use bevy::{
    ecs::{
        component::{ComponentHooks, StorageType},
        system::EntityCommands,
        world::DeferredWorld,
    },
    state::state::FreelyMutableState,
    ui::Val::*,
};
use bevy_mod_picking::prelude::*;

use crate::prelude::*;

/// The default gap between Ui elements.
const UI_GAP: Val = Px(10.);

/// An extension trait for spawning useful Ui widgets.
pub trait Widget {
    /// An Ui element that is a box with text inside. For it to be functional,
    /// add navigation with `.nav()` or `.nav_state()`.
    fn button(&mut self, text: impl Into<String>) -> EntityCommands;
    /// A text bundle with one section.
    fn text(&mut self, text: impl Into<String>) -> EntityCommands;
}

impl<T: SpawnExt> Widget for T {
    fn button(&mut self, text: impl Into<String>) -> EntityCommands {
        let text = text.into();
        let mut button = self.spawn((
            NodeBundle {
                style: Style {
                    width: Px(200.),
                    height: Px(65.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            UiBackgroundColor("primary"),
        ));
        button.with_children(|node| {
            node.text(text).insert(Pickable::IGNORE);
        });
        button
    }

    fn text(&mut self, text: impl Into<String>) -> EntityCommands {
        self.spawn((
            TextBundle::from_section(text, TextStyle {
                font_size: 24.,
                ..default()
            }),
            UiTextColor("light"),
        ))
    }
}

/// An extension trait for spawning Ui containers.
pub trait Container {
    /// Creates an Ui node that orders elements vertically (a div)
    fn col(&mut self) -> EntityCommands;
    /// Creates an Ui node that orders elements horizontally (a span)
    fn row(&mut self) -> EntityCommands;
    /// Base Ui node from where to build interfaces.
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

/// Ui node that takes the whole screen and centers the content.
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

/// Convenience function for easily modifying usual style properties of an Ui
/// node.
pub trait StyleBuilder {
    /// Modifies the `width` of the node.
    fn width(&mut self, value: Val);
    /// Modifies the `height` of the node.
    fn height(&mut self, value: Val);
    /// Modifies the `flex_direction` of the node.
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

/// Convenience function for modifying the style of an Ui node.
pub trait Stylable {
    /// Returns a reference to the `Style` of this node.
    fn style(&mut self) -> &mut Style;
}

impl Stylable for NodeBundle {
    fn style(&mut self) -> &mut Style {
        &mut self.style
    }
}

/// Convenience functions for adding navigation capabilities to Ui nodes.
pub trait NavigableExt<'a> {
    /// Converts the node into a `NavContainer`, allowing for navigation of its
    /// child elements.
    fn nav_container(&'a mut self) -> &mut EntityCommands;
    /// Converts the node into `Navigable`, adding the propper bundles. Takes a
    /// bevy system as a callback.
    fn nav<Marker>(&'a mut self, callback: impl IntoSystem<(), (), Marker>) -> &mut EntityCommands;
    /// Converts the node into `Navigable`, making the callback a transition
    /// into a new state.
    fn nav_state<S: FreelyMutableState>(&'a mut self, state: S) -> &mut EntityCommands;
}

impl<'a> NavigableExt<'a> for EntityCommands<'a> {
    fn nav_container(&'a mut self) -> &mut EntityCommands {
        self.insert(NavContainer);
        self
    }

    fn nav<Marker>(&'a mut self, callback: impl IntoSystem<(), (), Marker>) -> &mut EntityCommands {
        self.insert((
            Navigable {
                label: "TODO".into(),
            },
            NavBundle::default(),
            On::<NavActionEvent>::run(callback),
        ));
        self
    }

    fn nav_state<S: FreelyMutableState>(&'a mut self, state: S) -> &mut EntityCommands {
        let callback = move |mut next_state: ResMut<NextState<S>>| {
            next_state.set(state.clone());
        };
        self.nav(callback)
    }
}

/// An internal trait for types that can spawn entities.
/// This is here so that [`Widgets`] can be implemented on all types that
/// are able to spawn entities.
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
trait SpawnExt {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl SpawnExt for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl SpawnExt for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

/// Adds a themable background color to an Ui node. The literal parameter must
/// be the name of one of the fields of `ColorPalette`.
#[derive(Clone)]
struct UiBackgroundColor(&'static str);

impl Default for UiBackgroundColor {
    fn default() -> Self {
        Self("primary")
    }
}

impl Component for UiBackgroundColor {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _id| {
            let field = world
                .get::<UiBackgroundColor>(entity)
                .cloned()
                .unwrap_or_default()
                .0;
            let color = color_from_palette(&world, field);
            let Some(mut background) = world.get_mut::<BackgroundColor>(entity) else {
                return;
            };
            *background = color.into();
        });
    }
}

/// Adds a themable text color to an Ui text node. The literal parameter must
/// be the name of one of the fields of `ColorPalette`.
#[derive(Clone)]
struct UiTextColor(&'static str);

impl Default for UiTextColor {
    fn default() -> Self {
        Self("light")
    }
}

impl Component for UiTextColor {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _id| {
            // Color
            let field = world
                .get::<UiTextColor>(entity)
                .cloned()
                .unwrap_or_default()
                .0;
            let color = color_from_palette(&world, field);

            // Font
            let Some(font) = world.get_resource::<AssetMap<FontAssetKey>>() else { return };
            let font = font.get(&FontAssetKey::Main);

            let Some(mut text) = world.get_mut::<Text>(entity) else {
                return;
            };
            for section in &mut text.sections {
                section.style.color = color;
                section.style.font = font.clone_weak();
            }
        });
    }
}

/// Converts from a named palette field to the corresponding color.
fn color_from_palette(world: &DeferredWorld, field: &'static str) -> Color {
    let palette = world
        .get_resource::<GameOptions>()
        .map(|data| data.palette)
        .unwrap_or_default();
    let Some(reflect) = palette.field(field) else { return css::RED.into() };
    reflect
        .downcast_ref::<Color>()
        .cloned()
        .unwrap_or(css::RED.into())
}
