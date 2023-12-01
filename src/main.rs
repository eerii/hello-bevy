use bevy::{prelude::*, window::WindowResolution};
use hello_bevy::GamePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Hello Bevy!".to_string(), // [CHANGE]: Game title
                    resolution: WindowResolution::new(600., 600.),
                    resizable: false, // or use fit_canvas_to_parent: true for resizing on the web
                    canvas: Some("#bevy".to_string()),
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            //.set(ImagePlugin::default_nearest()), // [CHANGE]: Use if your game is pixel art
            GamePlugin,
        ))
        // Run
        .run();
}
