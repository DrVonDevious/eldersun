use bevy::prelude::*;

use crate::{bundles::player::PlayerBundle, plugins::{glyph::components::Glyph, action_queue::components::ActionQueue}, components::{position::Position, tags::Player}};

pub fn setup(
    mut commands: Commands
) {
    commands.spawn(PlayerBundle::default());

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Glyph {
            index: 'r' as usize,
            fg: Color::RED,
            bg: Some(Color::GREEN),
            x: -9,
            y: 6,
            ..default()
        },
    ));
}

pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    for player_transform in player_query.iter() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation = player_transform.translation;
        }
    }
}

pub fn handle_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Position, &mut ActionQueue), With<Player>>,
) {
    for (mut position, mut action_queue) in query.iter_mut() {
        if action_queue.ready {
            let mut direction = Vec2::ZERO;

            if keyboard_input.pressed(KeyCode::A) {
                direction.x -= 1.0;
                action_queue.next();
            }
            if keyboard_input.pressed(KeyCode::D) {
                direction.x += 1.0;
                action_queue.next();
            }
            if keyboard_input.pressed(KeyCode::W) {
                direction.y += 1.0;
                action_queue.next();
            }
            if keyboard_input.pressed(KeyCode::S) {
                direction.y -= 1.0;
                action_queue.next();
            }

            if direction != Vec2::ZERO {
                direction = direction.normalize();
            }

            position.x += direction.x as i32;
            position.y += direction.y as i32;
        }
    }
}
