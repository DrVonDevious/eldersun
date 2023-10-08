use bevy::{prelude::*, window::WindowResolution};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(
        WindowPlugin {
            primary_window: Some(Window {
                title: "Eldersun".to_string(),
                resolution: WindowResolution::new(1280.0, 720.0),
                resizable: false,
                ..default()
            }),
            ..default()
        }
    ));

    app.run();
}
