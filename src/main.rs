use bevy::{prelude::*, window::WindowResolution};
use palette::Palette;
use plugins::glyph::{GlyphAtlas, GlyphPlugin};

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
                resizable: false,
                ..default()
            }),
            ..default()
        }
    ).set(ImagePlugin::default_nearest()));

    app.add_plugins(GlyphPlugin {
        glyph_atlas: GlyphAtlas {
            tileset: "cp437_8x16.png".to_string(),
            glyph_size: Vec2::new(8.0, 16.0),
            background_color: Palette::BLUE,
            columns: 32,
            rows: 8,
            scale: 2.0,
        },
    });

    app.add_systems(Startup, systems::player::setup);
    app.add_systems(Update, (
        systems::player::handle_controls,
        systems::movement::update,
    ));

    app.insert_resource(ClearColor(Color::BLACK));

    app.run();
}
