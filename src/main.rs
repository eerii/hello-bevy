use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use hello_bevy::GamePlugin;

fn main() {
    let mut app = App::new()
        // Resources
        .insert_resource(ClearColor(Color::rgb(0.2, 0.3, 0.5)))
        // Plugins
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Hello Bevy!".to_string(), // [CHANGE]: Game title
                        resolution: WindowResolution::new(800., 800.),
                        resizable: false, // or use fit_canvas_to_parent: true for resizing on the web
                        canvas: Some("#bevy".to_string()),
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            GamePlugin,
        ))
        // Run
        .run();
}
