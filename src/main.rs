mod debug;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};

fn main() {
    let mut app = App::new();
    // Plugins
    app.insert_resource(ClearColor(Color::rgb(0.6, 0.7, 1.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Title".to_string(),
                resolution: WindowResolution::new(800., 800.),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }));

    // Debug plugins
    #[cfg(debug_assertions)]
    app.add_plugin(debug::inspector());

    // Systems
    app.add_systems(Startup, init).add_systems(Update, update);

    // Get debug schedule
    #[cfg(debug_assertions)]
    debug::save_schedule(&mut app, &["Startup", "Update"]);

    // Run
    app.run();
}

// Components
// ...

// Resources
// ...

// Startup systems

fn init(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

// Update systems

fn update() {}
