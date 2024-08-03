//! Reusable Ui widgets to easily build interfaces.

use bevy::{
    ecs::{system::EntityCommands, world::DeferredWorld},
    state::state::FreelyMutableState,
    ui::Val::*,
};
use bevy_mod_picking::prelude::*;

use crate::prelude::*;

/// The default gap between Ui elements.
const UI_GAP: Val = Px(10.);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        add_target_camera_to_ui.run_if(any_with_component::<UiRoot>),
    );
}

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
                    border: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                border_radius: BorderRadius::MAX,
                ..default()
            },
            UiBackgroundColor("primary"),
            UiBorderColor("light"),
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
        root.insert((UiRoot, Name::new("UI Root")));
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

component_palette!(UiBackgroundColor, BackgroundColor, "primary");

/// Adds a themable border color to an Ui node. The literal parameter must
/// be the name of one of the fields of `ColorPalette`.
#[derive(Clone)]
struct UiBorderColor(&'static str);

component_palette!(UiBorderColor, BorderColor, "dark");

/// Adds a themable text color to an Ui text node and also sets the propper
/// font. The literal parameter must be the name of one of the fields of
/// `ColorPalette`.
#[derive(Clone)]
struct UiTextColor(&'static str);

component_palette!(
    UiTextColor,
    Text,
    "light",
    |mut world: DeferredWorld, entity: Entity, color: Color| {
        let Some(font) = world.get_resource::<AssetMap<FontAssetKey>>() else { return };
        let font = font.get(&FontAssetKey::Main);

        let Some(mut text) = world.get_mut::<Text>(entity) else {
            return;
        };
        for section in &mut text.sections {
            section.style.color = color;
            section.style.font = font.clone_weak();
        }
    }
);

/// Helper component to add the Ui root node to the propper target camera.
#[derive(Component)]
struct UiRoot;

/// Iterates through all of the Ui roots and sets the target camera. Note that
/// the `UiRoot` component will be removed after the target camera is appended
/// so don't use it for queries. This is done to avoid running this system every
/// frame and instead only do it when a root is added.
/// This can't easily be a component hook since it is hard to make queries from
/// a DeferredWorld.
fn add_target_camera_to_ui(
    mut cmd: Commands,
    roots: Query<Entity, With<UiRoot>>,
    camera: Query<Entity, With<FinalCamera>>,
) {
    let camera = single!(camera);
    for entity in &roots {
        cmd.entity(entity)
            .remove::<UiRoot>()
            .insert(TargetCamera(camera));
    }
}
