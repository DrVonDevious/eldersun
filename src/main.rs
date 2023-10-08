use bevy::{prelude::*, window::WindowResolution, input::mouse::{MouseScrollUnit, MouseWheel}};
use plugins::glyph::{components::Glyph, GlyphAtlas};

pub mod plugins;

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

    app.add_plugins(plugins::glyph::GlyphPlugin {
        glyph_atlas: GlyphAtlas {
            tileset: "cp437_8x16.png".to_string(),
            glyph_size: Vec2::new(8.0, 16.0),
            columns: 32,
            rows: 8,
            scale: 1.0,
        },
    });

    app.add_systems(Startup, test_system);
    app.add_systems(Update, (test_zoom, test_move));

    app.run();
}

fn test_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    for x in 0..100 {
        for y in 0..100 {
            commands.spawn((
                Glyph {
                    index: '.' as usize,
                    color: Color::GREEN,
                    x,
                    y,
                },
            ));
        }
    }
}

fn test_zoom(
    mut mouse_scroll_events: EventReader<MouseWheel>,
    mut glyph_atlas: ResMut<plugins::glyph::GlyphAtlas>,
) {
    for event in mouse_scroll_events.iter() {
        if event.unit == MouseScrollUnit::Line {
            glyph_atlas.scale += event.y * 1.0;
            glyph_atlas.scale = glyph_atlas.scale.clamp(1.0, 5.0);
        }
    }
}

fn test_move(
    keys: Res<Input<KeyCode>>,
    glyph_atlas: ResMut<plugins::glyph::GlyphAtlas>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in camera_query.iter_mut() {
        if keys.pressed(KeyCode::W) {
            transform.translation.y += 8.0 * glyph_atlas.scale;
        }
        if keys.pressed(KeyCode::A) {
            transform.translation.x -= 8.0 * glyph_atlas.scale;
        }
        if keys.pressed(KeyCode::S) {
            transform.translation.y -= 8.0 * glyph_atlas.scale;
        }
        if keys.pressed(KeyCode::D) {
            transform.translation.x += 8.0 * glyph_atlas.scale;
        }
    }
}