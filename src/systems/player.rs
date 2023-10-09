use bevy::prelude::*;

use crate::{bundles::player::PlayerBundle, plugins::glyph::components::Glyph, components::{position::Position, tags::Player}};

pub fn setup(
    mut commands: Commands
) {
    commands.spawn(PlayerBundle::default());

    commands.spawn((
        Glyph {
            index: 'r' as usize,
            fg: Color::RED,
            bg: Some(Color::GREEN),
            x: -9,
            y: 12,
            ..default()
        },
    ));
}

pub fn handle_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Position, With<Player>>,
) {
    for mut position in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
        }

        position.x += direction.x as i32;
        position.y += direction.y as i32;
    }
}