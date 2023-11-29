use bevy::{log::LogPlugin, prelude::*, window::WindowResolution};
use hello_bevy::{GamePlugin, COLOR_DARKER};

fn main() {
    App::new()
        // Resources
        .insert_resource(ClearColor(COLOR_DARKER))
        // Plugins
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Hello Bevy!".to_string(), // [CHANGE]: Game title
                        resolution: WindowResolution::new(600., 600.),
                        resizable: false, // or use fit_canvas_to_parent: true for resizing on the web
                        canvas: Some("#bevy".to_string()),
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .disable::<LogPlugin>(),
            //.set(ImagePlugin::default_nearest()), // use if your game is pixel art
            GamePlugin,
        ))
        // Run
        .run();
}
