use bevy::{prelude::*, window::WindowResolution};
use palette::Palette;
use plugins::{glyph::{GlyphAtlas, GlyphPlugin}, action_queue::ActionQueuePlugin, map::{MapConfig, MapPlugin}};

pub mod bundles;
pub mod components;
pub mod plugins;
pub mod systems;
pub mod palette;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(
        WindowPlugin {
            primary_window: Some(Window {
                title: "Eldersun".to_string(),
                resolution: WindowResolution::new(1280.0, 720.0),
                resizable: true,
                ..default()
            }),
            ..default()
        }
    ).set(ImagePlugin::default_nearest()));

    app.add_plugins(GlyphPlugin {
        glyph_atlas: GlyphAtlas {
            tileset: "cp437_8x16.png".to_string(),
            glyph_size: Vec2::new(8.0, 16.0),
            background_color: Palette::BLACK,
            columns: 32,
            rows: 8,
            scale: 1.0,
        },
    });

    app.add_plugins(MapPlugin {
        map_config: MapConfig {
            width: 16,
            height: 8,
            seed: "arda".to_string(),
        },
    });

    app.add_plugins(ActionQueuePlugin);

    app.add_systems(Startup, (
        systems::player::setup,
    ));

    app.add_systems(Update, (
        systems::movement::update,
        systems::player::handle_controls,
    ));

    app.add_systems(PostUpdate, systems::player::camera_follow);

    app.insert_resource(ClearColor(Palette::BLACK));

    app.run();
}
